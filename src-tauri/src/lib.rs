mod project;
mod task;
mod task_item;


#[tauri::command]
fn create_project(title: String, description: Option<String>) -> Result<i64, String> {
    project::create_project(&title, description.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_projects() -> Result<Vec<project::Project>, String> {
    project::get_all_projects()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_project(id: i64) -> Result<(), String> {
    project::delete_project(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_project_content(id: i64, title: String, description: Option<String>) -> Result<(), String> {
    project::update_project_content(id, &title, description.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_project_sequence(id: i64, sequence: i64) -> Result<(), String> {
    project::update_project_sequence(id, sequence)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_project(id: i64) -> Result<project::Project, String> {
    project::get_project(id)
        .map_err(|e| e.to_string())
}


#[tauri::command]
fn create_task(project_id: i64, title: String, description: Option<String>) -> Result<i64, String> {
    task::create_task(project_id, &title, description.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_tasks(project_id: i64) -> Result<Vec<task::Task>, String> {
    task::get_tasks(project_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_task(id: i64) -> Result<(), String> {
    task::delete_task(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_task_content(id: i64, title: String, description: Option<String>) -> Result<(), String> {
    task::update_task_content(id, &title, description.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_task_sequence(id: i64, sequence: i64) -> Result<(), String> {
    task::update_task_sequence(id, sequence)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_task(id: i64) -> Result<task::Task, String> {
    task::get_task(id)
        .map_err(|e| e.to_string())
}


#[tauri::command]
fn create_task_item(content: String) -> Result<i64, String> {
    task_item::create_task_item(&content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn link_task_item(task_id: i64, item_id: i64) -> Result<i64, String> {
    task_item::link_task_item(task_id, item_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_task_items(task_id: i64) -> Result<Vec<task_item::TaskItemWithSequence>, String> {
    task_item::get_task_items(task_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_task_item_content(task_item_id: i64, content: String) -> Result<(), String> {
    task_item::update_task_item_content(task_item_id, &content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_task_item(id: i64) -> Result<(), String> {
    task_item::delete_task_item(id)
        .map_err(|e| e.to_string())
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_project, get_projects, delete_project, update_project_content, update_project_sequence, get_project,
            create_task, get_tasks, delete_task, update_task_content, update_task_sequence, get_task,
            create_task_item, link_task_item, get_task_items, update_task_item_content, delete_task_item
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
