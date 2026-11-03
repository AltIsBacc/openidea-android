use std::error::Error;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

/// Settings live in private storage; projects go to `Android/data/<id>/files` (readable, never executed).
pub struct Dirs {
    pub config: PathBuf,
    pub projects: PathBuf,
}

fn env_dir(key: &str) -> Option<PathBuf> {
    std::env::var_os(key).map(PathBuf::from).filter(|p| p.is_absolute())
}

impl Dirs {
    pub fn resolve(app: &AppHandle) -> Result<Self, Box<dyn Error>> {
        let private = match env_dir("OPENIDEA_PRIVATE_DIR") {
            Some(dir) => dir,
            None => app.path().app_data_dir()?,
        };
        let shared = env_dir("OPENIDEA_SHARED_DIR").unwrap_or_else(|| private.clone());
        let config = private.join("config");
        let projects = shared.join("projects");
        std::fs::create_dir_all(&config)?;
        std::fs::create_dir_all(&projects)?;
        Ok(Self { config, projects })
    }
}
