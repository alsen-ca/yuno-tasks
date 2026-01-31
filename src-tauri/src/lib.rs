mod db;

#[tauri::command]
fn create_task(title: String, description: Option<String>) -> Result<i64, String> {
    db::create_task(&title, description.as_deref())
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![create_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
