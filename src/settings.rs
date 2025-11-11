use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UserSettings {
    pub available_moods: Vec<String>,
    pub tags: Vec<String>,
    pub goals: Vec<String>,
}

impl UserSettings {
    pub fn load() -> Self {
        let path = "settings.json";
        let mut settings = if let Ok(data) = fs::read_to_string(path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        };

        if settings.available_moods.is_empty() {
            settings.available_moods = vec![
                "Happy".to_string(),
                "Calm".to_string(),
                "Tired".to_string(),
                "Sad".to_string(),
                "Motivated".to_string(),
                "Exited".to_string(),
            ];
        }

        if settings.tags.is_empty() {
            settings.tags = vec![
                "#thinking".to_string(),
                "#stressed".to_string(),
                "#relaxed".to_string(),
                "#unsure".to_string(),
                "#angry".to_string(),
            ];
        }

        if settings.goals.is_empty() {
            settings.goals = vec!["No Alcohol".to_string(), "No Cigarettes".to_string()]
        }
        settings
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write("settings.json", json).expect("Failed to write settings");
    }
}
