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

struct MoodFlowApp {
    current_tab: Tab,
    entries: Vec<Entry>,
    new_entry: Entry,
    settings: UserSettings,
}

impl Default for MoodFlowApp {
    fn default() -> Self {
        Self {
            current_tab: Tab::Add,
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
        // let mut note = self.new_entry.note.clone().unwrap_or_default();
        // ui.text_edit_multiline(&mut note);

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
        ui.label("Available moods:");

        for mood in &mut self.settings.available_moods {
            ui.text_edit_singleline(mood);
        }

        if ui.button("Add new mood").clicked() {
            self.settings.available_moods.push(String::new());
        }

        if ui.button("Save settings").clicked() {
            self.settings.save();
        }
    }
}
