mod pandoc;

use pandoc::{
    clear_sessions, convert_markdown, install_crossref, install_pandoc, is_crossref_installed,
    is_pandoc_installed, list_templates, pandoc_version, prepare_input_payload,
    prepare_template_protected,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("format-tools.log".to_string()),
                    }),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                ])
                .level(if cfg!(debug_assertions) {
                    tauri_plugin_log::log::LevelFilter::Debug
                } else {
                    tauri_plugin_log::log::LevelFilter::Info
                })
                .filter(|metadata| !metadata.target().starts_with("tao"))
                .build(),
        )
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
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
            list_templates,
            clear_sessions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
