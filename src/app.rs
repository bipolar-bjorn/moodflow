use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use eframe::egui;

use crate::database::init_db;
use crate::models::{AppScreen, SettingsTab, Tab, User};
use crate::mood::Entry;
use crate::settings::UserSettings;
use rusqlite::Connection;

pub struct MoodFlowApp {
    screen: AppScreen,
    current_user: Option<User>,
    db_connection: Option<Connection>,
    current_tab: Tab,
    current_settings_tab: SettingsTab,
    entries: Vec<Entry>,
    new_entry: Entry,
    settings: UserSettings,
    temp_name: String,
    temp_pin: String,
    temp_email: String,
    temp_sync_cloud: bool,
}

impl Default for MoodFlowApp {
    fn default() -> Self {
        Self {
            screen: AppScreen::Welcome,
            current_user: None,
            db_connection: None,
            current_tab: Tab::Add,
            current_settings_tab: SettingsTab::General,
            entries: Vec::new(),
            new_entry: Entry::default(),
            settings: UserSettings::load(),
            temp_name: String::new(),
            temp_pin: String::new(),
            temp_email: String::new(),
            temp_sync_cloud: false,
        }
    }
}

impl eframe::App for MoodFlowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            AppScreen::Welcome => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.show_welcome_screen(ui);
                });
            }
            AppScreen::AddUser => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.show_add_user_screen(ui);
                });
            }
            AppScreen::MainApp => {
                egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.current_tab, Tab::Add, "➕ Add");
                        ui.selectable_value(&mut self.current_tab, Tab::History, "📜 History");
                        ui.selectable_value(&mut self.current_tab, Tab::Analytics, "📈 Analytics");
                        ui.selectable_value(&mut self.current_tab, Tab::Settings, "⚙️ Settings");
                    });
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.show_main_app_screen(ui);
                    match self.current_tab {
                        Tab::Add => self.show_add_tab(ui),
                        Tab::History => self.show_history_tab(ui),
                        Tab::Analytics => self.show_analytics_tab(ui),
                        Tab::Settings => self.show_settings_tab(ui),
                    }
                });
            }
        }

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| ui.label("2025 (c) 3PiStudio"));
        });
    }
}

impl MoodFlowApp {
    fn show_welcome_screen(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Welcome to MoodFlow");
            ui.label("Track your moods safely and beautifully");
            if ui.button("Add User").clicked() {
                self.screen = AppScreen::AddUser;
            }
        });
    }

    fn show_add_user_screen(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Create new user");

            ui.label("Name:");
            ui.text_edit_singleline(&mut self.temp_name);

            ui.label("PIN Code (4-6 digits, optional):");
            ui.text_edit_singleline(&mut self.temp_pin);

            ui.label("Email (Optional):");
            ui.text_edit_singleline(&mut self.temp_email);

            ui.checkbox(&mut self.temp_sync_cloud, "Sync with cloud (optional)");

            if ui.button("Done").clicked() {
                let pin_hash = if !self.temp_pin.is_empty() {
                    let mut salt_bytes = [0u8; 16];
                    let salt = SaltString::encode_b64(&mut salt_bytes).unwrap();
                    Some(
                        Argon2::default()
                            .hash_password(self.temp_pin.as_bytes(), &salt)
                            .unwrap()
                            .to_string(),
                    )
                } else {
                    None
                };

                let user = User {
                    id: None,
                    name: self.temp_name.clone(),
                    pin_hash,
                    email: if self.temp_email.is_empty() {
                        None
                    } else {
                        Some(self.temp_email.clone())
                    },
                };
                self.current_user = Some(user);

                self.db_connection = Some(init_db().unwrap());
                self.screen = AppScreen::MainApp;
            }
        });
    }

    fn show_main_app_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading(format!(
            "Hello, {}!",
            self.current_user.as_ref().unwrap().name
        ));
        ui.separator();
        ui.label("Here goes the main app UI (Add/History/Analytics/Settings)...");
    }

    fn show_add_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Add Mood Entry");
        ui.separator();

        egui::ComboBox::from_label("Mood")
            .selected_text(&self.new_entry.mood)
            .show_ui(ui, |ui| {
                for mood in &self.settings.available_moods {
                    ui.selectable_value(&mut self.new_entry.mood, mood.clone(), mood);
                }
            });

        ui.label("Tags:");
        for tag in &self.settings.tags {
            let mut selected = self.new_entry.tags.contains(tag);
            if ui.checkbox(&mut selected, tag).clicked() {
                if selected {
                    self.new_entry.tags.push(tag.clone());
                } else {
                    self.new_entry.tags.retain(|t| t != tag);
                }
            }
        }

        ui.label("Note:");
        if let Some(note) = &mut self.new_entry.note {
            ui.text_edit_multiline(note);
        } else {
            let mut temp = String::new();
            ui.text_edit_multiline(&mut temp);
            if !temp.is_empty() {
                self.new_entry.note = Some(temp);
            }
        }

        if ui.button("Save Entry").clicked() {
            self.new_entry.save();
            self.entries.push(self.new_entry.clone());
            self.new_entry = Entry::default();
        }
    }

    fn show_history_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("History");
        ui.separator();
        if self.entries.is_empty() {
            ui.label("No entries yet.");
        } else {
            for entry in &self.entries {
                ui.label(format!(
                    "{} — {} ({})",
                    entry.date.format("%Y-%m-%d %H:%M"),
                    entry.mood,
                    entry.note.clone().unwrap_or_default()
                ));
            }
        }
    }

    fn show_analytics_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Analytics");
        ui.separator();
        ui.label(format!("Total mood entries: {}", self.entries.len()));
    }

    fn show_settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.separator();

        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.current_settings_tab,
                SettingsTab::General,
                "General",
            );
            ui.selectable_value(&mut self.current_settings_tab, SettingsTab::Moods, "Moods");
            ui.selectable_value(&mut self.current_settings_tab, SettingsTab::Tags, "Tags");
            ui.selectable_value(&mut self.current_settings_tab, SettingsTab::Goals, "Goals");
        });
        ui.separator();

        match self.current_settings_tab {
            SettingsTab::General => {
                ui.label("General settings:");
                ui.separator();
            }
            SettingsTab::Moods => {
                ui.label("Moods settings:");
                ui.separator();
                ui.label("Available moods:");
                for mood in &mut self.settings.available_moods {
                    ui.text_edit_singleline(mood);
                }

                if ui.button("Add new mood").clicked() {
                    self.settings.available_moods.push(String::new());
                }
            }
            SettingsTab::Tags => {
                ui.label("Tags settings:");
                ui.separator();
                ui.label("Available tags:");
                for tag in &mut self.settings.tags {
                    ui.text_edit_singleline(tag);
                }
                if ui.button("Add new tag").clicked() {
                    self.settings.tags.push(String::new());
                }
            }
            SettingsTab::Goals => {
                ui.label("Goals settings:");
                ui.separator();
                ui.label("Available goals:");
                for goal in &mut self.settings.goals {
                    ui.text_edit_singleline(goal);
                }
                if ui.button("Add new goal").clicked() {
                    self.settings.goals.push(String::new());
                }
            }
        }
        ui.separator();

        if ui.button("Save settings").clicked() {
            self.settings.save();
        }
    }
}
