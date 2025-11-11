use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id: Option<i32>,
    pub date: DateTime<Local>,
    pub mood: String,
    pub note: Option<String>,
}

impl Entry {
    pub fn new(mood: &str, note: Option<String>) -> Self {
        Entry {
            id: None,
            date: Local::now(),
            mood: mood.to_string(),
            note,
        }
    }
}
