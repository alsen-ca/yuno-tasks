use rusqlite::{Connection, Result};

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
            FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE
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

#[derive(Debug)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub sequence: i64
}

pub fn create_project(title: &str, description: Option<&str>) -> Result<i64> {
    let conn = Connection::open(DB_PATH)?;

    // Get the highest current sequence number
    let max_sequence: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sequence), 0) FROM projects",
        [],
        |row| row.get(0)
    )?;

    conn.execute(
        "INSERT INTO projects (title, description, sequence) VALUES (?1, ?2, ?3)",
        (title, description, max_sequence + 1)
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_all_projects() -> Result<Vec<Project>> {
    let conn = Connection::open(DB_PATH)?;
    let mut stmt = conn.prepare("SELECT id, title, description, sequence FROM projects ORDER BY sequence")?;

    let projects: Vec<Project> = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            sequence: row.get(3)?,
        })
    })?.collect::<Result<Vec<Project>, _>>()?;

    Ok(projects)
}

pub fn update_project_content(project_id: i64, title: &str, description: Option<&str>) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "UPDATE projects SET title = ?1, description = ?2 WHERE id = ?3",
        (title, description, project_id)
    )?;

    Ok(())
}

// When updating the sequence of a project, ensure that the sequences of all other projects also happen
pub fn update_project_sequence(project_id: i64, new_sequence: i64) -> Result<()> {
    let mut conn = Connection::open(DB_PATH)?;
    let tx = conn.transaction()?;

    // Get the current sequence of the project being updated
    let current_sequence: i64 = tx.query_row(
        "SELECT sequence FROM projects WHERE id = ?1",
        [project_id],
        |row| row.get(0)
    )?;

    // Only proceed if the sequence is actually changing
    if current_sequence != new_sequence {
        // Determine the direction of the move (up or down)
        let is_moving_up = new_sequence < current_sequence;

        if is_moving_up {
            // Moving up: increment sequences between new_sequence and current_sequence
            tx.execute(
                "UPDATE projects
                 SET sequence = sequence + 1
                 WHERE sequence >= ?1 AND sequence < ?2",
                (new_sequence, current_sequence)
            )?;
        } else {
            // Moving down: decrement sequences between current_sequence and new_sequence
            tx.execute(
                "UPDATE projects
                 SET sequence = sequence - 1
                 WHERE sequence > ?1 AND sequence <= ?2",
                (current_sequence, new_sequence)
            )?;
        }

        // Finally, update the sequence of the target project
        tx.execute(
            "UPDATE projects SET sequence = ?1 WHERE id = ?2",
            (new_sequence, project_id)
        )?;
    }

    tx.commit()?;

    Ok(())
}

pub fn delete_project(id: i64) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
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
