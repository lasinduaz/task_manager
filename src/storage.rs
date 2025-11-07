use std::os::unix::raw::time_t;

use rusqlite::{params, Connection, Result};

pub fn establish_connection() -> Result<()> {
    let conn = Connection::open("task_manager.db").expect("Failed to open db");

    let sql = "
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
    ";

    conn.execute_batch(sql)?;

    println!("Database initialized successfully!");

    Ok(())
}
pub fn add_task(
    title: &str,
    description: &str,
    status: &str,
    priority: i32,
    due_date: Option<&str>,
) -> Result<i64> {
    let conn = Connection::open("task_manager.db")?;
    conn.execute(
        "INSERT INTO tasks (title, description, status, priority, due_date)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![title, description, status, priority, due_date],
    )?;
    let id = conn.last_insert_rowid();
    Ok(id)
}