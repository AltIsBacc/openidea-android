//! Android shell. Only hosts the WebView for now: the commands the UI invokes
//! (`src/bindings.ts`) are not registered here, so they reject until the remote
//! client lands.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().run(tauri::generate_context!()).expect("error while running OpenIDEA");
}
