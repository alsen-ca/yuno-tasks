use rusqlite::{Connection, Result};
use serde::Serialize;


#[derive(Debug, Serialize)]
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
