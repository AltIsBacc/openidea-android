//! Android shell. Hosts the WebView and the app-level commands the UI needs before a remote
//! client lands (`app_info`, settings); every other command in `src/bindings.ts` is not
//! registered here, so it rejects.

mod commands;
mod storage;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(storage::Dirs::resolve(app.handle())?);
            app.manage(commands::SettingsFile::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app_info,
            commands::settings_get,
            commands::settings_get_global,
            commands::settings_set,
        ])
        .run(tauri::generate_context!())
        .expect("error while running OpenIDEA");
}
