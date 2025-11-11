use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UserSettings {
    pub available_moods: Vec<String>,
}

impl UserSettings {
    pub fn load() -> Self {
        let path = "settings.json";
        if let Ok(data) = fs::read_to_string(path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            let default = Self {
                available_moods: vec![
                    "Happy".to_string(),
                    "Calm".to_string(),
                    "Tired".to_string(),
                    "Sad".to_string(),
                    "Motivated".to_string(),
                    "Exited".to_string(),
                ],
            };
            default.save();
            default
        }
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write("settings.json", json).expect("Failed to write settings");
    }
}
