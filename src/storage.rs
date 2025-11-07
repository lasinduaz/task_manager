use std::os::unix::raw::time_t;

use rusqlite::{Connection, Result};

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