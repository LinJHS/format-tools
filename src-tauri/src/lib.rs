mod pandoc;

use pandoc::{
    install_pandoc, install_crossref, is_pandoc_installed, 
    is_crossref_installed, pandoc_version, convert_markdown, prepare_input_payload, prepare_template_protected, list_templates
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            install_pandoc,
            install_crossref,
            is_pandoc_installed,
            is_crossref_installed,
            pandoc_version,
            convert_markdown,
            prepare_input_payload,
            prepare_template_protected,
            list_templates
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
