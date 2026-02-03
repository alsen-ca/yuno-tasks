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
    pub sequence: Option<i64>
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
    println!("Assigning new link_task_item to task item ID: {}", item_id);
    let conn = Connection::open(DB_PATH)?;

    // Get the max sequence for this task
    let max_sequence: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sequence), 0) FROM task_item_links WHERE task_id = ?1",
        [task_id],
        |row| row.get(0)
    )?;
    println!("link_task_item for task item is of value: {}", max_sequence + 1);

    match conn.execute(
        "INSERT INTO task_item_links (task_id, item_id, sequence)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(task_id, item_id)
         DO UPDATE SET sequence = ?3",
        (task_id, item_id, max_sequence + 1),
    ) {
        Ok(_) => {
            println!("debugging, this is the last row? {:?}", conn.last_insert_rowid());
            Ok(conn.last_insert_rowid())
        }
        Err(e) => {
            eprintln!("Failed to insert task_item_link: {}", e);
            Err(e)
        }
    }
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

pub fn update_task_item_content(task_item_id: i64, content: &str, status: TaskItemStatus) -> Result<()> {
    println!("Trying to update task item of id {} with content {} and status {:?}", task_item_id, content, status);
    let mut conn = Connection::open(DB_PATH)?;
    let tx = conn.transaction()?;
    let status_int = i64::from(status);

    tx.execute(
        "UPDATE task_items SET content = ?1, status = ?2 WHERE id = ?3",
        (content, status_int, task_item_id)
    )?;

    // If new status is not Pending, remove it from task_item_links
    if status_int != 0 {
        tx.execute(
            "UPDATE task_item_links SET sequence = NULL WHERE item_id = ?1",
            [task_item_id],
        )?;
    }

    tx.commit()?;
    Ok(())
}

pub fn update_task_item_sequence(task_id: i64, item_id: Option<i64>, new_sequence: Option<i64>) -> Result<(), Error> {
    println!("Updating task item sequence for task: {}, item: {:?}, new sequence: {:?}", task_id, item_id, new_sequence);
    let mut conn = Connection::open(DB_PATH)?;
    let tx = conn.transaction()?;

    // Get all items for this task, ordered by sequence (NULLs last)
    let mut items = tx
        .prepare(
            "SELECT item_id, sequence FROM task_item_links
             WHERE task_id = ?1 AND sequence IS NOT NULL
             ORDER BY sequence",
        )?
        .query_map([task_id], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<Result<Vec<(i64, i64)>>>()?;

    // If item_id and new_sequence are provided, update the sequence for that item
    if let (Some(item_id), Some(new_sequence)) = (item_id, new_sequence) {
        // Find the item in the list
        if let Some(index) = items.iter().position(|(id, _)| *id == item_id) {
            let (_, old_sequence) = items.remove(index);
            // Ensure new_sequence is within bounds
            let new_index = (new_sequence - 1) as usize;
            if new_index > items.len() {
                return Err(Error::InvalidQuery);
            }
            items.insert(new_index, (item_id, new_sequence));
        }
    }

    // Step 1: Temporarily set sequences to negative values
    for (new_index, (id, _)) in items.iter().enumerate() {
        let temp_seq = -(new_index as i64 + 1);
        tx.execute(
            "UPDATE task_item_links SET sequence = ?1 WHERE task_id = ?2 AND item_id = ?3",
            (temp_seq, task_id, id),
        )?;
    }

    // Step 2: Set sequences to their final positive values
    for (new_index, (id, _)) in items.iter().enumerate() {
        let final_seq = (new_index + 1) as i64;
        tx.execute(
            "UPDATE task_item_links SET sequence = ?1 WHERE task_id = ?2 AND item_id = ?3",
            (final_seq, task_id, id),
        )?;
    }

    tx.commit()?;
    Ok(())
}

pub fn delete_task_item(id: i64) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute("DELETE FROM task_items WHERE id = ?1", [id])?;
    Ok(())
}
