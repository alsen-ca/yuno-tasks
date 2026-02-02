use rusqlite::{Connection, Result, Error};
use serde::{Serialize, Deserialize};

pub const DB_PATH: &str = "yuno-tasks.db";

#[derive(Debug, Serialize)]
pub struct TaskItem {
    pub id: i64,
    pub content: String,
    #[serde(skip_serializing)]
    pub status: TaskItemStatus
}
impl TaskItem {
    pub fn from_row(row: &rusqlite::Row) -> Result<Self> {
        Ok(TaskItem {
            id: row.get(0)?,
            content: row.get(1)?,
            status: TaskItemStatus::from(row.get::<_, i64>(2)?),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct TaskItemLink {
    pub task_id: i64,
    pub item_id: i64,
    pub sequence: i64
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TaskItemStatus {
    Pending = 0,
    Completed = 1,
    Canceled = 2,
}
impl From<i64> for TaskItemStatus {
    fn from(value: i64) -> Self {
        match value {
            0 => TaskItemStatus::Pending,
            1 => TaskItemStatus::Completed,
            2 => TaskItemStatus::Canceled,
            _ => panic!("Invalid status value: {}", value),
        }
    }
}

impl From<TaskItemStatus> for i64 {
    fn from(status: TaskItemStatus) -> Self {
        status as i64
    }
}


#[derive(Debug, Serialize)]
pub struct TaskItemWithSequence {
    pub id: i64,
    pub content: String,
    pub status: TaskItemStatus,
    pub sequence: i64
}


pub fn create_task_item(content: &str) -> Result<i64> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "INSERT INTO task_items (content) VALUES (?1)",
        [content],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn link_task_item(task_id: i64, item_id: i64) -> Result<i64> {
    let conn = Connection::open(DB_PATH)?;

    // Get the max sequence for this task
    let max_sequence: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sequence), 0) FROM task_item_links WHERE task_id = ?1",
        [task_id],
        |row| row.get(0)
    )?;

    conn.execute(
        "INSERT INTO task_item_links (task_id, item_id, sequence) VALUES (?1, ?2, ?3)",
        (task_id, item_id, max_sequence + 1),
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_task_items(task_id: i64) -> Result<Vec<TaskItemWithSequence>> {
    let conn = Connection::open(DB_PATH)?;

    let mut stmt = conn.prepare(
        "SELECT task_items.id, task_items.content, task_items.status, task_item_links.sequence
         FROM task_items
         JOIN task_item_links ON task_items.id = task_item_links.item_id
         WHERE task_item_links.task_id = ?1
         ORDER BY task_item_links.sequence"
    )?;

    let items = stmt
        .query_map([task_id], |row| {
            Ok(TaskItemWithSequence {
                id: row.get(0)?,
                content: row.get(1)?,
                status: TaskItemStatus::from(row.get::<_, i64>(2)?),
                sequence: row.get(3)?
            })
        })?
        .collect::<Result<Vec<TaskItemWithSequence>, _>>()?;

    Ok(items)
}

pub fn update_task_item_content(task_item_id: i64, content: &str) -> Result<()> {
    println!("Trying to update task item of id {} with content {}", task_item_id, content);
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "UPDATE task_items SET content = ?1 WHERE id = ?2",
        (content, task_item_id)
    )?;
    Ok(())
}

pub fn delete_task_item(id: i64) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute("DELETE FROM task_items WHERE id = ?1", [id])?;
    Ok(())
}
