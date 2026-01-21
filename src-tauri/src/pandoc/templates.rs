use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize)]
pub struct TemplateInfo {
    pub reference_doc: String,
    pub protected_path: String,
}

pub fn prepare_template(app_handle: &AppHandle, template_name: &str) -> Result<TemplateInfo, String> {
    // Try to find template in resources
    let resource_path = find_template_resource(template_name)?;

    let cache_root = app_handle
        .path()
        .cache_dir()
        .map_err(|e| format!("Failed to get cache dir: {}", e))?;

    let runtime_dir = cache_root
        .join("format-tools")
        .join("templates")
        .join("runtime");
    fs::create_dir_all(&runtime_dir)
        .map_err(|e| format!("Failed to create runtime dir: {}", e))?;

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis();

    let runtime_docx = runtime_dir.join(format!("{}-{}.docx", template_name, millis));
    fs::copy(&resource_path, &runtime_docx)
        .map_err(|e| format!("Failed to stage template: {}", e))?;

    Ok(TemplateInfo {
        reference_doc: runtime_docx.to_string_lossy().to_string(),
        protected_path: resource_path.to_string_lossy().to_string(),
    })
}

fn find_template_resource(template_name: &str) -> Result<PathBuf, String> {
    // 1. Try to find template in the current executable directory
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let resource_path = exe_dir.join("resources").join("templates").join(template_name);
            if resource_path.exists() {
                return Ok(resource_path);
            }
        }
    }

    // 2. Try to find template relative to project root (for development)
    let candidates = [
        "src-tauri/resources/templates",
        "resources/templates",
        "../resources/templates",
    ];

    for base in &candidates {
        let path = Path::new(base).join(template_name);
        if path.exists() {
            if let Ok(canonical) = path.canonicalize() {
                return Ok(canonical);
            }
            return Ok(path);
        }
    }

    Err(format!("Template '{}' not found in resources", template_name))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub isFree: Option<bool>,
    pub isPro: Option<bool>,
}

pub fn list_templates(app_handle: &AppHandle) -> Result<Vec<TemplateMeta>, String> {
    // Locate templates metadata file in resources/templates/templates.json
    let metadata_candidates = [
        // exe dir resources
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("resources").join("templates").join("templates.json"))),
        // dev paths
        Some(PathBuf::from("src-tauri/resources/templates/templates.json")),
        Some(PathBuf::from("resources/templates/templates.json")),
        Some(PathBuf::from("../resources/templates/templates.json")),
    ];

    for opt_path in metadata_candidates.iter().flatten() {
        if opt_path.exists() {
            let content = fs::read_to_string(opt_path)
                .map_err(|e| format!("Failed to read metadata: {}", e))?;
            let parsed: serde_json::Value = serde_json::from_str(&content)
                .map_err(|e| format!("Invalid JSON in metadata: {}", e))?;
            // Expect either { templates: [...] } or just an array
            let templates_val = if parsed.is_object() {
                parsed.get("templates").cloned().unwrap_or(serde_json::Value::Array(vec![]))
            } else {
                parsed
            };
            let list: Vec<TemplateMeta> = serde_json::from_value(templates_val)
                .map_err(|e| format!("Failed to parse templates: {}", e))?;
            return Ok(list);
        }
    }

    Err("No templates metadata found in resources".to_string())
}
