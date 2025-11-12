use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let connection = Connection::open("moodflow.db")?;
    connection.execute_batch(
        r#"
        -- USERS
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            pin_hash TEXT,
            created_at TEXT DEFAULT (datetime('now'))
        );

        -- MOODS
        CREATE TABLE IF NOT EXISTS moods (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            color TEXT DEFAULT '#999999'
        );

        -- TAGS
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL
        );

        -- GOALS
        CREATE TABLE IF NOT EXISTS goals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            created_at TEXT DEFAULT (datetime('now')),
            completed INTEGER DEFAULT 0,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );

        -- ENTRIES
        CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            mood_id INTEGER NOT NULL,
            note TEXT,
            goal_id INTEGER,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (mood_id) REFERENCES moods(id),
            FOREIGN KEY (goal_id) REFERENCES goals(id)
        );

        -- ENTRY_TAGS
        CREATE TABLE IF NOT EXISTS entry_tags (
            entry_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (entry_id, tag_id),
            FOREIGN KEY (entry_id) REFERENCES entries(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        );

        -- STATS CACHE (optional)
        CREATE TABLE IF NOT EXISTS stats_cache (
            user_id INTEGER NOT NULL,
            metric TEXT NOT NULL,
            value REAL,
            updated_at TEXT DEFAULT (datetime('now')),
            PRIMARY KEY (user_id, metric),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );

        -- INDEXES
        CREATE INDEX IF NOT EXISTS idx_entries_user_date ON entries (user_id, date);
        CREATE INDEX IF NOT EXISTS idx_entry_tags_tag ON entry_tags (tag_id);
        CREATE INDEX IF NOT EXISTS idx_goals_user ON goals (user_id);
    "#,
    )?;
    Ok(connection)
}
