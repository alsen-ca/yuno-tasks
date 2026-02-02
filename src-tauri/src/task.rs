use rusqlite::{Connection, Result, Error};
use serde::Serialize;

pub const DB_PATH: &str = "yuno-tasks.db";

#[derive(Debug, Serialize)]
pub struct Task {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub sequence: i64
}

pub fn create_task(project_id: i64, title: &str, description: Option<&str>) -> Result<i64> {
    let conn = Connection::open(DB_PATH)?;

    let max_sequence: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sequence), 0) FROM tasks WHERE project_id = ?1",
        [project_id],
        |row| row.get(0)
    )?;

    conn.execute(
        "INSERT INTO tasks (project_id, title, description, sequence) VALUES (?1, ?2, ?3, ?4)",
        (project_id, title, description, max_sequence + 1),
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_tasks(project_id: i64) -> Result<Vec<Task>> {
    let conn = Connection::open(DB_PATH)?;

    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE project_id = ?1 ORDER BY sequence")?;

    let tasks = stmt
        .query_map([project_id], |row| {
            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                sequence: row.get(4)?
            })
        })?
        .collect::<Result<Vec<Task>, _>>()?;

    Ok(tasks)
}

pub fn update_task_content(task_id: i64, title: &str, description: Option<&str>) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "UPDATE tasks SET title = ?1, description = ?2 WHERE id = ?3",
        (title, description, task_id)
    )?;

    Ok(())
}

pub fn update_task_sequence(task_id: i64, new_sequence: i64) -> Result<(), Error> {
    println!("This is the task: {} with this new sequence: {}", task_id, new_sequence);
    let mut conn = Connection::open(DB_PATH)?;
    let tx = conn.transaction()?;

    // Get the project_id for the task
    let project_id: i64 = tx.query_row(
        "SELECT project_id FROM tasks WHERE id = ?1",
        [task_id],
        |row| row.get(0),
    )?;

    // Make copy of tasks in array
    let mut tasks = tx.prepare("SELECT id, sequence FROM tasks WHERE project_id = ?1 ORDER BY sequence")?
        .query_map([project_id], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?
        .collect::<Result<Vec<(i64, i64)>>>()?;

    // Find the project and move to new position
    let index = tasks.iter().position(|(id, _)| *id == task_id).ok_or_else(|| Error::InvalidQuery)?;
    let (_, old_sequence) = tasks.remove(index);

    // Ensure new_sequence is within bounds
    let new_index = new_sequence as usize - 1;
    if new_index > tasks.len() {
        return Err(Error::InvalidQuery);
    }

    tasks.insert(new_index, (task_id, old_sequence));

    // Step 1: Temporarily set sequences to negative values
    for (new_index, (id, _)) in tasks.iter().enumerate() {
        let temp_seq = -(new_index as i64 + 1);
        tx.execute(
            "UPDATE tasks SET sequence = ?1 WHERE id = ?2",
            (temp_seq, *id)
        )?;
    }

    // Step 2: Set sequences to their final positive values
    for (new_index, (id, _)) in tasks.iter().enumerate() {
        let final_seq = (new_index + 1) as i64;
        tx.execute(
            "UPDATE tasks SET sequence = ?1 WHERE id = ?2",
            (final_seq, *id)
        )?;
    }

    tx.commit()?;
    Ok(())
}

pub fn get_task(id: i64) -> Result<Task> {
    let conn = Connection::open(DB_PATH)?;

    let tasks = conn.query_row(
        "SELECT * FROM tasks WHERE id = ?1",
        [id],
        |row| {
            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                sequence: row.get(4)?,
            })
        }
    )?;

    Ok(tasks)
}


pub fn delete_task(id: i64) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
    Ok(())
}
