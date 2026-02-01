use rusqlite::{Connection, Result, Error};
use serde::Serialize;

pub const DB_PATH: &str = "yuno-tasks.db";

#[derive(Debug, Serialize)]
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
pub fn update_project_sequence(project_id: i64, new_sequence: i64) -> Result<(), Error> {
    let mut conn = Connection::open(DB_PATH)?;
    let tx = conn.transaction()?;

    // Make copy of projects in array
    let mut projects = tx.prepare("SELECT id, sequence FROM projects ORDER BY sequence")?
        .query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?
        .collect::<Result<Vec<(i64, i64)>>>()?;

    // Find the project and move to new position
    let index = projects.iter().position(|(id, _)| *id == project_id).ok_or_else(|| Error::InvalidQuery)?;
    let (_, old_sequence) = projects.remove(index);
    projects.insert(new_sequence as usize - 1, (project_id, old_sequence));

    // Step 1: Temporarily set sequences to negative values
    for (new_index, (id, _)) in projects.iter().enumerate() {
        let temp_seq = -(new_index as i64 + 1);
        tx.execute(
            "UPDATE projects SET sequence = ?1 WHERE id = ?2",
            (temp_seq, *id)
        )?;
    }

    // Step 2: Set sequences to their final positive values
    for (new_index, (id, _)) in projects.iter().enumerate() {
        let final_seq = (new_index + 1) as i64;
        tx.execute(
            "UPDATE projects SET sequence = ?1 WHERE id = ?2",
            (final_seq, *id)
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
