use rusqlite::{Connection, Result};
use crate::mood::Entry;
use chrono::Local;

pub fn init_db() -> Result<Connection> {
    let connection = Connection::open("moodflow.db")?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            mood TEXT NOT NULL,
            note TEXT
        )",
        [],
    )?;
    Ok(connection)
}

pub fn add_entry(connection: &Connection, entry: &Entry) -> Result<()> {
    connection.execute(
        "INSERT INTO entries (date, mood, note) VALUES (?1, ?2, ?3)",
        rusqlite::params![
        entry.date.to_rfc3339(),
        entry.mood,
        entry.note.as_deref(),
        ],
    )?;
    Ok(())
}

pub fn get_entries(connection: &Connection) -> Result<Vec<Entry>> {
    let mut statement = connection.prepare("SELECT id, date, mood, note FROM entries")?;
    let mood_iter = statement.query_map([], |row| {
        let date_str: String = row.get(1)?;
        Ok(Entry {
            id: row.get(0)?,
            date: date_str.parse::<chrono::DateTime<Local>>().unwrap_or_else(|_| Local::now()),
            mood: row.get(2)?,
            note: row.get(3)?,
        })
    })?;

    let mut entries = Vec::new();
    for entry in mood_iter {
        entries.push(entry?);
    }
    Ok(entries)
}