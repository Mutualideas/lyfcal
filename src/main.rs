// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui::IconData;

mod config;
mod draw;
mod gui;

fn main() -> Result<(), eframe::Error> {
    let icon = image::open("src/assets/icon.png")
        .expect("Failed to open icon path")
        .to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    let icon_data = IconData {
        rgba: icon.into_raw(),
        width: icon_width,
        height: icon_height,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("lyfcal")
            .with_maximize_button(false)
            .with_icon(icon_data)
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
