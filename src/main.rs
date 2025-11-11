use eframe::egui;
mod database;
mod mood;
mod settings;

use mood::Entry;
use settings::UserSettings;

fn main() -> eframe::Result<()> {
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([350.0, 600.0])
        .with_min_inner_size([250.0, 400.0])
        .with_title("MoodFlow");

    let options = eframe::NativeOptions {
        viewport: viewport,
        ..Default::default()
    };

    eframe::run_native(
        "MoodFlow",
        options,
        Box::new(|_cc| Ok(Box::new(MoodFlowApp::default()))),
    )
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tab {
    Add,
    History,
    Analytics,
    Settings,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum SettingsTab {
    General,
    Moods,
    Tags,
    Goals,
}

struct MoodFlowApp {
    current_tab: Tab,
    current_settings_tab: SettingsTab,
    entries: Vec<Entry>,
    new_entry: Entry,
    settings: UserSettings,
}

impl Default for MoodFlowApp {
    fn default() -> Self {
        Self {
            current_tab: Tab::Add,
            current_settings_tab: SettingsTab::General,
            entries: Vec::new(),
            new_entry: Entry::default(),
            settings: UserSettings::load(),
        }
    }
}

impl eframe::App for MoodFlowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Add, "➕ Add");
                ui.selectable_value(&mut self.current_tab, Tab::History, "📜 History");
                ui.selectable_value(&mut self.current_tab, Tab::Analytics, "📈 Analytics");
                ui.selectable_value(&mut self.current_tab, Tab::Settings, "⚙️ Settings");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.current_tab {
            Tab::Add => self.show_add_tab(ui),
            Tab::History => self.show_history_tab(ui),
            Tab::Analytics => self.show_analytics_tab(ui),
            Tab::Settings => self.show_settings_tab(ui),
        });
    }
}

impl MoodFlowApp {
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
