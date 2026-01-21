mod pandoc;

use pandoc::{
    install_pandoc, install_crossref, is_pandoc_installed, 
    is_crossref_installed, pandoc_version, convert_markdown, prepare_input_payload
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            install_pandoc,
            install_crossref,
            is_pandoc_installed,
            is_crossref_installed,
            pandoc_version,
            convert_markdown,
            prepare_input_payload
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
