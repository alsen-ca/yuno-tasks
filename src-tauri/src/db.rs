use rusqlite::{Connection, Result};

pub const DB_PATH: &str = "yuno-tasks.db";

pub fn init_db() -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS task_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            \"order\" INTEGER NOT NULL,
            status INTEGER NOT NULL,
            FOREIGN KEY(task_id) REFERENCES tasks(id)
        )",
        [],
    )?;

    Ok(())
}

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
}

pub fn create_task(title: &str, description: Option<&str>) -> Result<i64> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "INSERT INTO tasks (title, description) VALUES (?1, ?2)",
        (title, description),
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_tasks() -> Result<Vec<Task>> {
    let conn = Connection::open(DB_PATH)?;

    let mut stmt = conn.prepare(
        "SELECT id, title, description FROM tasks ORDER BY id DESC",
    )?;

    let tasks = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(tasks)
}

pub fn delete_task(id: i64) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
    Ok(())
}
