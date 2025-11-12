// ...existing code...
use rusqlite::{params, Connection, Result, OptionalExtension};

pub fn establish_connection() -> Result<()> {
    let conn = Connection::open("task_manager.db")?;
    let sql = r#"
    CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        description TEXT,
        status TEXT DEFAULT 'pending' CHECK(status IN ('pending', 'in_progress', 'done')),
        priority INTEGER DEFAULT 1 CHECK(priority BETWEEN 1 AND 3),
        due_date TEXT,
        created_at TEXT DEFAULT (datetime('now')),
        updated_at TEXT
    );
    "#;
    conn.execute_batch(sql)?;
    Ok(())
}

pub fn add_task(
    title: &str,
    description: Option<&str>,
    status: &str,
    priority: i32,
    due_date: Option<&str>,
) -> Result<i64> {
    let conn = Connection::open("task_manager.db")?;
    conn.execute(
        "INSERT INTO tasks (title, description, status, priority, due_date) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![title, description, status, priority, due_date],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn view_tasks() -> Result<()> {
    let conn = Connection::open("task_manager.db")?;
    let mut stmt = conn.prepare(
        "SELECT id, title, description, status, priority, due_date, created_at, updated_at FROM tasks ORDER BY id",
    )?;

    let task_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, i32>(4)?,
            row.get::<_, Option<String>>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, Option<String>>(7)?,
        ))
    })?;

    for task_res in task_iter {
        let (id, title, description, status, priority, due_date, created_at, updated_at) =
            task_res?;
        println!("----------------------------------------");
        println!("ID         : {}", id);
        println!("Title      : {}", title);
        println!("Description: {}", description.unwrap_or_default());
        println!("Status     : {}", status);
        println!("Priority   : {}", priority);
        println!("Due date   : {}", due_date.unwrap_or_default());
        println!("Created at : {}", created_at);
        println!("Updated at : {}", updated_at.unwrap_or_default());
    }

    Ok(())
}

pub fn get_task_status(id: i32) -> Result<Option<String>> {
    let conn = Connection::open("task_manager.db")?;
    let mut stmt = conn.prepare("SELECT status FROM tasks WHERE id = ?1")?;
    let status_opt = stmt
        .query_row(params![id], |row| row.get::<_, String>(0))
        .optional()?;
    Ok(status_opt)
}

pub fn update_task_status(id: i32, status: &str) -> Result<usize> {
    let conn = Connection::open("task_manager.db")?;
    let changed = conn.execute(
        "UPDATE tasks SET status = ?1, updated_at = datetime('now') WHERE id = ?2",
        params![status, id],
    )?;
    Ok(changed)
}

pub fn delete_task(id: i32) -> Result<usize> {
    let conn = Connection::open("task_manager.db")?;
    let changed = conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    Ok(changed)
}