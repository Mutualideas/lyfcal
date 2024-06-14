#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chrono::{offset, Datelike, Local, NaiveDate, Weekday};
use eframe::egui::*;
//use egui::style;
use core::f32;
use egui_extras;
use std::{collections::BTreeMap, usize};
//use std::sync::{atomic::{AtomicBool, Ordering}, Arc,};

#[derive(Debug)]
struct Config {
    birthdate: Option<NaiveDate>,
    elapsed_date_bool: bool,
    elapsed_date: chrono::NaiveDate,
    life_expectancy: i32,
    events: BTreeMap<NaiveDate, String>,
    display_colour: egui::Color32,
    display_colour2: egui::Color32,
    display_colour3: egui::Color32,
    display_colour4: egui::Color32,
    col_spacing: f32,
    row_spacing: f32,
    edge_spacing: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            //minimized: true,
            birthdate: NaiveDate::from_ymd_opt(2000, 1, 1),
            elapsed_date_bool: true,
            elapsed_date: Local::now().date_naive(),
            life_expectancy: 80,
            events: BTreeMap::new(),
            display_colour: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 100),
            display_colour2: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20),
            display_colour3: egui::Color32::from_rgba_unmultiplied(225, 225, 250, 100),
            display_colour4: egui::Color32::from_rgba_unmultiplied(225, 225, 250, 20),
            col_spacing: 1.0,
            row_spacing: 0.0,
            edge_spacing: 1.0,
        }
    }
}

#[derive(Default, Debug)]
struct LyfcalApp {
    config: Config,
    show_immediate_viewport: bool,
    //draw_data: DrawData
    //show_deferred_viewport: Arc<AtomicBool>,
}

impl LyfcalApp {
    //Calculate to maximize unit size/spacing for the given screen space and spacing
    fn calculate_grid(&self, ui: &mut Ui) -> (usize, usize, f32) {
        let event_no = self.config.events.len();
        let birthday_offset = self
            .config
            .birthdate
            .unwrap()
            .weekday()
            .num_days_from_monday() as usize;
        let mut col_output: usize = 0;
        let mut row_output: usize = 0;
        let mut size_output: f32 = 0.0;

        //Test for the number of columns needed to fit the total number of event entries.
        for col_num in 1.. {
            //Calculate the unit size for each number of columns.
            let unit_size = ui.available_width()
                / (col_num as f32 * 7.0
                    + self.config.col_spacing * (col_num as f32 - 1.0)
                    + 2.0 * self.config.edge_spacing);
            //Work out the maximum number of rows given the unit size.
            let row_num = ((ui.available_height() - 2.0 * self.config.edge_spacing * unit_size
                + self.config.row_spacing as f32 * unit_size)
                / (unit_size + self.config.row_spacing as f32 * unit_size))
                as usize;
            //Then check if the maximum number of event entries is greater than the total number of event entries.
            if (col_num * 7) * row_num >= (event_no - birthday_offset) {
                col_output = col_num;
                row_output = row_num;
                size_output = unit_size;
                break;
            }
        }
        (col_output, row_output, size_output)
    }

    //Offset unit body to account for empty column
    fn col_offset(&self, col: usize, row: usize, unit_size: f32) -> f32 {
        let max_unit_num = col * row * 7;
        let unit_num = self.config.events.len()
            + self
                .config
                .birthdate
                .unwrap()
                .weekday()
                .num_days_from_monday() as usize;
        let col_capacity = row * 7;

        //the line below crashes with overflow issue if left at usize
        if (max_unit_num as i32 - unit_num as i32) >= col_capacity as i32 {
            unit_size * (7.0 + self.config.col_spacing) / (col as f32 - 2.0)
        } else {
            0.0
        }
    }

    //Calculate unit location
    fn calculate_pos(
        &self,
        col: usize,
        row: usize,
        unit_size: f32,
        date: NaiveDate,
        col_offset: f32,
        x_offset: f32,
        y_offset: f32,
    ) -> Rect {
        let x = unit_size
            * (self.config.edge_spacing
                + date.weekday().number_from_monday() as f32
                + 7.0 * col as f32
                + self.config.col_spacing * (col as f32 - 1.0))
            + col as f32 * col_offset
            + x_offset;
        let y = unit_size
            * (self.config.edge_spacing
                + row as f32
                + self.config.row_spacing * (row as f32 - 1.0))
            + y_offset;
        Rect::from_min_size(
            egui::pos2(x + 0.1 * unit_size, y + 0.1 * unit_size),
            egui::vec2(0.8 * unit_size, 0.8 * unit_size),
        )
    }

    //Draw logic
    fn draw_unit(&self, ui: &mut Ui, rect: Rect, date: NaiveDate, unit_size: f32) {
        if date > self.config.elapsed_date && is_weekday(date) {
            ui.painter()
                .rect_filled(rect, 0.0, self.config.display_colour);
        } else if date > self.config.elapsed_date && !is_weekday(date) {
            ui.painter()
                .rect_filled(rect, 0.0, self.config.display_colour3);
        } else if date <= self.config.elapsed_date && is_weekday(date) {
            ui.painter()
                .rect_filled(rect, 0.0, self.config.display_colour2);
        } else {
            ui.painter()
                .rect_filled(rect, 0.0, self.config.display_colour4);
        }

        if date == self.config.elapsed_date {
            ui.painter().rect_stroke(
                rect,
                0.0,
                Stroke::new(unit_size * 0.1 + 0.5, self.config.display_colour),
            );
        };
        //TODO weekends, 2ndary colours, date elapsed
    }
}

impl eframe::App for LyfcalApp {
    /*
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        //egui::Rgba::TRANSPARENT.to_array()
    }
    */

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            //.frame(egui::Frame::none())
            .show(ctx, |ui| {
                //Initialise adjustable style for inner scope
                let mut style = (*ui.ctx().style()).clone();
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
                                .clamp_range(1..=120)
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
                                let mut date = Local::now().date_naive();
                                if self.config.elapsed_date_bool == true {
                                    self.config.elapsed_date = Local::now().date_naive();
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
                        self.show_immediate_viewport = true;
                        /* == DEFFERRED VIEWPORT ==
                        let current_value = self.show_deferred_viewport.load(Ordering::Relaxed);
                        self.show_deferred_viewport
                            .store(!current_value, Ordering::Relaxed);
                        */
                    }
                });
                ui.separator();

                let birth_year = self.config.birthdate.unwrap().year();
                let end_year = birth_year + self.config.life_expectancy;
                let duration = NaiveDate::from_ymd_opt(
                    end_year,
                    self.config.birthdate.unwrap().month(),
                    self.config.birthdate.unwrap().day(),
                )
                .unwrap()
                .signed_duration_since(
                    NaiveDate::from_ymd_opt(
                        birth_year,
                        self.config.birthdate.unwrap().month(),
                        self.config.birthdate.unwrap().day(),
                    )
                    .unwrap(),
                )
                .num_days() as i32;

                ui.label(format!("life expectancy: {} days", duration));

                ui.label(format!("elapsed date: {}", self.config.elapsed_date));

                ui.label(format!("event number: {}", self.config.events.len()));
            });

        if self.show_immediate_viewport {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("lyfcal")
                    .with_min_inner_size([480.0, 320.0])
                    //.with_transparent(true)
                    .with_maximized(true)
                    //.with_decorations(false)
                    //.with_mouse_passthrough(true)
                    .with_fullsize_content_view(true),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default()
                        .frame(egui::Frame::none())
                        .show(ctx, |ui| {
                            let mut date_incre = self.config.birthdate.unwrap();
                            let mut rows_skipped = 0;
                            let birthday = self.config.birthdate.unwrap();

                            //To allow exception to the first week shown where the week doesn't begin on a monday, extra day added for the birthday itself.
                            let grid_scale: (usize, usize, f32) = self.calculate_grid(ui);

                            for col in 0..(grid_scale.0) {
                                for row in 0..(grid_scale.1) {
                                    for _ in date_incre.weekday().num_days_from_monday() as usize..7
                                    {
                                        if date_incre
                                            > *self.config.events.last_key_value().unwrap().0
                                        {
                                            break;
                                        }
                                        self.draw_unit(
                                            ui,
                                            self.calculate_pos(
                                                col,
                                                row,
                                                grid_scale.2,
                                                date_incre,
                                                self.col_offset(
                                                    grid_scale.0,
                                                    grid_scale.1,
                                                    grid_scale.2,
                                                ),
                                                0.0,
                                                0.0,
                                            ),
                                            date_incre,
                                            grid_scale.2,
                                        );
                                        date_incre += chrono::Duration::days(1);
                                    }
                                }
                            }
                        });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_immediate_viewport = false; //close viewport
                    }
                },
            );
        }

        /* For defferred viewport.
        if self.show_deferred_viewport.load(Ordering::Relaxed) {
            let show_deferred_viewport = self.show_deferred_viewport.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("deferred_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("lyfcal")
                    .with_transparent(true)
                    //.with_window_level(egui::WindowLevel::AlwaysOnBottom)
                    .with_maximized(true)
                    .with_fullsize_content_view(true),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        for days in self.config.events {
                            ui.label(format!("{:?}", days));
                        }
                    });
                    if ctx.input(|i| i.viewport().close_requested()) {
                        show_deferred_viewport.store(false, Ordering::Relaxed);
                    }
                },
            );
        }
        */
    }
}

// UI elements & Misc functions

fn grid_col_width(ui: &egui::Ui, n_col: usize) -> f32 {
    let gap_space = ui.spacing().item_spacing.x * (n_col as f32 - 1.0);
    let grid_w = ui.available_width();
    (grid_w - gap_space) / n_col as f32
}

fn is_weekday(date: NaiveDate) -> bool {
    match date.weekday() {
        Weekday::Sat | Weekday::Sun => false,
        _ => true,
    }
}

// Function to populate events with every day from the birthdate up to the end of the life expectancy
fn populate_events(config: &mut Config) {
    config.events.clear();
    if let Some(birthdate) = config.birthdate {
        let birth_year = birthdate.year();
        let end_year = birth_year + config.life_expectancy;
        let mut current_date = birthdate;
        let mut expectancy_day_count = 0;

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
            .with_maximize_button(false)
            //.with_icon(icon)
            .with_inner_size([345.0, 275.0])
            //.with_transparent(true)
            .with_resizable(false)
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "lyfcal config",
        options,
        Box::new(|_cc| Box::<LyfcalApp>::default()),
    )
}
