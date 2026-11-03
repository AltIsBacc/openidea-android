use crate::storage::Dirs;
use ide_core::settings::Settings;
use ide_core::{IdeError, IdeResult, IpcPath};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::io::ErrorKind;
use std::path::Path;
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    version: String,
    platform: String,
    config_dir: IpcPath,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SettingsScope {
    Global,
    Project,
}

pub struct SettingsFile(Mutex<()>);

impl SettingsFile {
    pub fn new() -> Self {
        Self(Mutex::new(()))
    }
}

fn file(dirs: &Dirs) -> std::path::PathBuf {
    dirs.config.join("settings.toml")
}

fn defaults() -> Value {
    serde_json::to_value(Settings::default()).expect("settings serialize")
}

fn deep_merge(base: &mut Value, layer: Value) {
    match (base, layer) {
        (Value::Object(base), Value::Object(layer)) => {
            for (k, v) in layer {
                match base.get_mut(&k) {
                    Some(existing) => deep_merge(existing, v),
                    None => {
                        base.insert(k, v);
                    }
                }
            }
        }
        (base, layer) => *base = layer,
    }
}

fn read_layer(path: &Path) -> IdeResult<Value> {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Value::Object(Map::new())),
        Err(e) => return Err(e.into()),
    };
    let table: toml::Table = toml::from_str(&text)
        .map_err(|e| IdeError::invalid(format!("{} is not valid TOML: {e}", path.display())))?;
    serde_json::to_value(table).map_err(|e| IdeError::invalid(e.to_string()))
}

fn effective(layer: &Value) -> Result<Settings, String> {
    let mut merged = defaults();
    deep_merge(&mut merged, layer.clone());
    serde_json::from_value(merged).map_err(|e| e.to_string())
}

fn load(path: &Path) -> Settings {
    let layer = read_layer(path).unwrap_or_else(|_| Value::Object(Map::new()));
    effective(&layer).unwrap_or_default()
}

fn assign(root: &mut Value, segments: &[&str], value: Value) {
    let (last, parents) = segments.split_last().expect("non-empty key");
    let mut cursor = root;
    for segment in parents {
        if !cursor.is_object() {
            *cursor = Value::Object(Map::new());
        }
        cursor = cursor
            .as_object_mut()
            .expect("object just ensured")
            .entry(*segment)
            .or_insert_with(|| Value::Object(Map::new()));
    }
    if !cursor.is_object() {
        *cursor = Value::Object(Map::new());
    }
    let map = cursor.as_object_mut().expect("object just ensured");
    if value.is_null() {
        map.remove(*last);
    } else {
        map.insert((*last).to_owned(), value);
    }
}

fn check_key(segments: &[&str], key: &str) -> IdeResult<()> {
    if segments.iter().any(|s| s.is_empty()) {
        return Err(IdeError::invalid(format!("invalid settings key {key:?}")));
    }
    let defaults = defaults();
    let mut cursor = &defaults;
    for segment in segments {
        match cursor.get(segment) {
            Some(next) => cursor = next,
            None if cursor.as_object().is_some_and(|m| m.is_empty()) => return Ok(()),
            None => return Err(IdeError::invalid(format!("unknown setting {key:?}"))),
        }
    }
    Ok(())
}

fn write_atomic(path: &Path, text: &str) -> IdeResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("toml.tmp");
    std::fs::write(&tmp, text)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

fn store(path: &Path, key: &str, value: Value) -> IdeResult<Settings> {
    let segments: Vec<&str> = key.split('.').collect();
    check_key(&segments, key)?;
    let mut layer = read_layer(path)?;
    assign(&mut layer, &segments, value);
    let settings = effective(&layer).map_err(|e| IdeError::invalid(format!("invalid value for {key:?}: {e}")))?;
    let table = toml::Table::try_from(&layer).map_err(|e| IdeError::invalid(e.to_string()))?;
    let text = toml::to_string_pretty(&table).map_err(|e| IdeError::invalid(e.to_string()))?;
    write_atomic(path, &text)?;
    Ok(settings)
}

#[tauri::command]
pub fn app_info(dirs: State<'_, Dirs>) -> AppInfo {
    AppInfo {
        version: env!("CARGO_PKG_VERSION").into(),
        platform: "linux".into(),
        config_dir: dirs.config.clone().into(),
    }
}

#[tauri::command]
pub fn settings_get(dirs: State<'_, Dirs>) -> Settings {
    load(&file(&dirs))
}

#[tauri::command]
pub fn settings_get_global(dirs: State<'_, Dirs>) -> Settings {
    load(&file(&dirs))
}

#[tauri::command]
pub fn settings_set(
    dirs: State<'_, Dirs>,
    lock: State<'_, SettingsFile>,
    scope: SettingsScope,
    key: String,
    value: String,
) -> IdeResult<Settings> {
    let _ = scope;
    let value: Value = serde_json::from_str(&value).map_err(|e| IdeError::invalid(e.to_string()))?;
    let _guard = lock.0.lock().unwrap_or_else(|e| e.into_inner());
    store(&file(&dirs), &key, value)
}
