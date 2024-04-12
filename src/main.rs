use eframe::egui;
use chrono::{NaiveDate, Datelike};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Visual Calendar",
        options,
        Box::new(|_cc| Box::new(VisualCalendarApp::default())),
    );
}

#[derive(Default)]
struct VisualCalendarApp {
    date: NaiveDate,
}

impl eframe::App for VisualCalendarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Visual Calendar");
            ui.label(format!("Today's Date: {}", self.date));

            if ui.button("Next Day").clicked() {
                self.date = self.date.succ();
            }
            if ui.button("Previous Day").clicked() {
                self.date = self.date.pred();
            }
        });
    }
}