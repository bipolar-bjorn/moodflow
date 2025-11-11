use eframe::egui;
use rusqlite::Connection;

mod mood;
mod database;
mod analytics;

use mood::Entry;
use analytics::mood_summary;

struct MoodFlowApp {
    mood_input: String,
    note_input: String,
    entries: Vec<Entry>,
    connection: Connection,
}

impl eframe::App for MoodFlowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MoodFlow");
            ui.horizontal(|ui| {
                ui.label("Mood:");
                ui.text_edit_singleline(&mut self.note_input);
            });

            if ui.button("Add Entry").clicked() {
                let entry = Entry::new(&self.mood_input,
                    if self.note_input.is_empty() { None } else { Some(self.note_input.clone()) });
                database::add_entry(&self.connection, &entry).unwrap();
                self.entries.push(entry);
                self.mood_input.clear();
                self.note_input.clear();    
            }

            ui.separator();
            ui.heading("History:");
            for entry in &self.entries {
                ui.label(format!("{} - {} ({})",
                    entry.date.format("%Y-%m-%d %H:%M"),
                    entry.mood,
                    entry.note.clone().unwrap_or_default()));
            }

            ui.separator();
            ui.heading("Mood Summary:");
            let summary = mood_summary(&self.entries);
            for (mood, count) in summary {
                ui.label(format!("{}: {}", mood, count));
            }
        });
    }
}


fn main() {
    let connection = database::init_db().unwrap();
    let entries = database::get_entries(&connection).unwrap_or_default();

    let app = MoodFlowApp {
        mood_input: String::new(),
        note_input: String::new(),
        entries,
        connection,
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "MoodFlow",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    ).unwrap();
}
