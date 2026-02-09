mod pandoc;

use std::fs::File;
use std::io::{Read, Write};
use tauri::Manager;

use pandoc::{
    clear_sessions, convert_markdown, install_crossref, install_pandoc, is_crossref_installed,
    is_pandoc_installed, list_templates, pandoc_version, prepare_input_payload,
    prepare_template_protected,
};

#[tauri::command]
async fn export_logs(app_handle: tauri::AppHandle, save_path: String) -> Result<(), String> {
    let log_dir = app_handle.path().app_log_dir().map_err(|e| e.to_string())?;

    let file = File::create(&save_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options: zip::write::FileOptions<()> = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    if log_dir.exists() {
        for entry in std::fs::read_dir(&log_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_file() {
                let name = path.file_name().unwrap().to_string_lossy();
                zip.start_file(name.into_owned(), options).map_err(|e| e.to_string())?;
                let mut f = File::open(path).map_err(|e| e.to_string())?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
                zip.write_all(&buffer).map_err(|e| e.to_string())?;
            }
        }
    }

    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("formatsman.log".to_string()),
                    }),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                ])
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .level_for("tao", tauri_plugin_log::log::LevelFilter::Error)
                .level_for("subclass", tauri_plugin_log::log::LevelFilter::Error)
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
            clear_sessions,
            export_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
