#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod config;
mod draw;
mod gui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("lyfcal")
            .with_maximize_button(false)
            //.with_icon(icon)
            .with_inner_size([345.0, 470.0])
            //.with_transparent(true)
            .with_resizable(false)
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "lyfcal config",
        options,
        Box::new(|_cc| Box::<gui::LyfcalApp>::default()),
    )
}
