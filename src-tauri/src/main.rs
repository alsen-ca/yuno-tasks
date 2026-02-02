// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, Result};

fn main() {
    if let Err(e) = init_db() {
        panic!("Database init failed: {:?}", e);
    }

    println!("SQLite ready");

    yuno_tasks_lib::run();
}

pub const DB_PATH: &str = "yuno-tasks.db";

pub fn init_db() -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            sequence INTEGER NOT NULL UNIQUE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            sequence INTEGER NOT NULL,
            FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE,
            UNIQUE(project_id, sequence)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS task_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            status INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;

    // TaskItem can be shared between multiple tasks
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task_item_links (
            task_id INTEGER NOT NULL,
            item_id INTEGER NOT NULL,
            sequence INTEGER NOT NULL,
            PRIMARY KEY (task_id, item_id),
            FOREIGN KEY(task_id) REFERENCES tasks(id),
            FOREIGN KEY(item_id) REFERENCES task_items(id) ON DELETE CASCADE
        )",
        [],
    )?;

    Ok(())
}
