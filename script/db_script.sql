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
INSERT INTO tasks (title, description, priority, due_date)
VALUES ('Buy groceries', 'Milk, Eggs, Bread', 2, '2025-10-15');

