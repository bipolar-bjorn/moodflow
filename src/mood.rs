use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: Option<i32>,
    pub date: DateTime<Local>,
    pub mood: String,
    pub note: Option<String>,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            id: None,
            date: Local::now(),
            mood: "Stable".to_string(),
            note: None,
        }
    }
}

impl Entry {
    pub fn save(&self) {
        println!("Saving entry: {} ({:?})", self.mood, self.note);
        // TODO: insert into SQLite
    }
    // pub fn new(mood: &str, note: Option<String>) -> Self {
    //     Entry {
    //         id: None,
    //         date: Local::now(),
    //         mood: mood.to_string(),
    //         note,
    //     }
    // }
}
