#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chrono::{Datelike, NaiveDate};
use eframe::egui;
//use egui::style;
use egui_extras;
use std::collections::BTreeMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
#[derive(Debug)]
struct Config {
    birthdate: Option<NaiveDate>,
    elapsed_date_bool: bool,
    elapsed_date: chrono::NaiveDate,
    life_expectancy: i32,
    events: BTreeMap<NaiveDate, String>,
    display_opacity: i32,
    display_colour: egui::Color32,
    display_colour2: egui::Color32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            //minimized: true,
            birthdate: Some(chrono::offset::Utc::now().date_naive()),
            elapsed_date_bool: true,
            elapsed_date: chrono::offset::Utc::now().date_naive(),
            life_expectancy: 80,
            events: BTreeMap::new(),
            display_opacity: 188,
            display_colour: egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255), //TODO
            display_colour2: egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255), //TODO
        }
    }
}

#[derive(Default, Debug)]
struct LyfcalApp {
    config: Config,
    show_deferred_viewport: Arc<AtomicBool>,
}

impl eframe::App for LyfcalApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ui.ctx().style()).clone(); //creating adjustable style for inner scope
            style.spacing.item_spacing.x = 4.0;
            ui.ctx().set_style(style.clone());

            ui.heading("lyfcal config");
            egui::Grid::new("expectancy_grid")
                .min_col_width(grid_col_width(ui, 2))
                .show(ui, |ui| {
                    ui.label("birthdate:");
                    if let Some(ref mut birthdate) = self.config.birthdate {
                        let mut date = *birthdate;
                        if ui
                            .add_sized(
                                [ui.available_width(), ui.spacing().interact_size.y],
                                egui_extras::DatePickerButton::new(&mut date),
                            )
                            .changed()
                        {
                            *birthdate = date;
                        }
                    } else {
                        // Handle the case where birthdate is None, e.g., set an initial date or show a placeholder
                        let mut date = chrono::NaiveDate::from_ymd_opt(2000, 1, 1); // or some other default
                        if ui
                            .add(egui_extras::DatePickerButton::new(&mut date.unwrap()))
                            .changed()
                        {
                            self.config.birthdate = Some(date.unwrap());
                        }
                    }
                    ui.end_row();

                    ui.label(format!("life expectancy:"));
                    ui.add_sized(
                        [ui.available_width(), ui.spacing().interact_size.y],
                        egui::DragValue::new(&mut self.config.life_expectancy)
                            .clamp_range(0..=120)
                            .suffix(" years"),
                    );
                    ui.end_row();
                    ui.label("");
                    ui.scope(|ui| {
                        let style = ui.style_mut();
                        if let Some(text_style) =
                            style.text_styles.get_mut(&egui::TextStyle::Button)
                        {
                            let new_size = text_style.size - 2.0; // Calculate the new size by subtracting 2
                            text_style.size = if new_size < 6.0 { 6.0 } else { new_size };
                            //Apply minimum
                        }

                        egui::Grid::new("expectancy_grid")
                            .min_col_width(grid_col_width(ui, 4))
                            .show(ui, |ui| {
                                ui.vertical_centered_justified(|ui| {
                                    if ui.button("−10").clicked() {
                                        self.config.life_expectancy -= 10;
                                    }
                                });
                                ui.vertical_centered_justified(|ui| {
                                    if ui.button("−1").clicked() {
                                        self.config.life_expectancy -= 1;
                                    }
                                });
                                ui.vertical_centered_justified(|ui| {
                                    if ui.button("+1").clicked() {
                                        self.config.life_expectancy += 1;
                                    }
                                });
                                ui.vertical_centered_justified(|ui| {
                                    if ui.button("+10").clicked() {
                                        self.config.life_expectancy += 10;
                                    }
                                });
                            });
                    });
                    ui.end_row();
                    ui.label("elapsed date:");
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.config.elapsed_date_bool, "now ")
                            .on_hover_text("set current date date to date elapsed");
                        ui.add_enabled_ui(!self.config.elapsed_date_bool, |ui| {
                            let mut date = chrono::offset::Utc::now().date_naive();
                            if self.config.elapsed_date_bool == true {
                                self.config.elapsed_date = chrono::offset::Utc::now().date_naive();
                                if ui
                                    .add_sized(
                                        [ui.available_width(), ui.spacing().interact_size.y],
                                        egui_extras::DatePickerButton::new(&mut date),
                                    )
                                    .changed()
                                {}
                            } else {
                                if ui
                                    .add_sized(
                                        [ui.available_width(), ui.spacing().interact_size.y],
                                        egui_extras::DatePickerButton::new(&mut date),
                                    )
                                    .changed()
                                {
                                    self.config.elapsed_date = date
                                }
                            }
                        });
                    });
                }); //end grid

            ui.separator();
            ui.vertical_centered(|ui| {
                if ui.button("initialize").clicked() {
                    populate_events(&mut self.config);
                    let current_value = self.show_deferred_viewport.load(Ordering::Relaxed);
                    self.show_deferred_viewport
                        .store(!current_value, Ordering::Relaxed);
                }
            });
            ui.separator();

            ui.label(format!(
                "life expectancy: {} years",
                self.config.life_expectancy
            ));

            ui.label(format!("elapsed date: {}", self.config.elapsed_date));
        });

        if self.show_deferred_viewport.load(Ordering::Relaxed) {
            let show_deferred_viewport = self.show_deferred_viewport.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("deferred_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("lyfcal")
                    .with_transparent(true)
                    //.with_window_level(egui::WindowLevel::AlwaysOnBottom)
                    //.with_fullscreen(true)
                    .with_maximized(true)
                    .with_fullsize_content_view(true),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );

                    //egui::CentralPanel::default().show(ctx, |ui| {
                    /*
                        for days in self.config.events.clone() {
                            ui.label(format!("{:?}", days));
                        }
                    */
                    //});
                    if ctx.input(|i| i.viewport().close_requested()) {
                        show_deferred_viewport.store(false, Ordering::Relaxed);
                    }
                },
            );
        }
    }
}

// UI elements

fn grid_col_width(ui: &egui::Ui, n_col: usize) -> f32 {
    let gap_space = ui.spacing().item_spacing.x * (n_col as f32 - 1.0);
    let grid_w = ui.available_width();
    (grid_w - gap_space) / n_col as f32
}

// Function to populate events with every day from the birthdate up to the end of the life expectancy
fn populate_events(config: &mut Config) {
    config.events.clear();
    if let Some(birthdate) = config.birthdate {
        let birth_year = birthdate.year();
        let end_year = birth_year + config.life_expectancy;
        let mut current_date = birthdate;
        let mut expectancy_day_count = 0;
        let mut weekday = birthdate.weekday();

        // Iterate through each year from birth year to end year for leap years
        for year in birth_year..=end_year - 1 {
            let days_between_bday =
                NaiveDate::from_ymd_opt(year + 1, birthdate.month(), birthdate.day())
                    .unwrap()
                    .signed_duration_since(
                        NaiveDate::from_ymd_opt(year, birthdate.month(), birthdate.day()).unwrap(),
                    )
                    .num_days();
            expectancy_day_count += days_between_bday;
            for _ in 0..days_between_bday {
                if current_date > birthdate + chrono::Duration::days(expectancy_day_count) {
                    break; // Stop if we exceed the life expectancy
                }
                config
                    .events
                    .insert(current_date, format!("{}", current_date.weekday()));
                current_date = match current_date.checked_add_signed(chrono::Duration::days(1)) {
                    Some(date) => date,
                    None => break, // Exit the loop if overflow occurs
                };
            }
        }
    }
}
// ----------------------------------------------------------------------------
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("lyfcal")
            .with_inner_size([345.0, 275.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "lyfcal config",
        options,
        Box::new(|_cc| Box::<LyfcalApp>::default()),
    )
}
