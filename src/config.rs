use chrono::{Local, NaiveDate};

#[derive(Debug, Clone)]
pub struct Config {
    pub birthdate: Option<NaiveDate>,
    pub elapsed_date_bool: bool,
    pub elapsed_date: chrono::NaiveDate,
    pub life_expectancy: i32,

    //pub display_weekends: bool,
    //pub display_birthday: bool,
    //pub enable_transparency: bool,
    //pub enable_mouse_passthrough: bool,
    pub color_weekday: egui::Color32,
    pub color_weekday_elapsed: egui::Color32,
    pub color_weekend: egui::Color32,
    pub color_weekend_elapsed: egui::Color32,
    pub color_birthday: egui::Color32,
    pub color_birthday_elapsed: egui::Color32,
    pub color_today: egui::Color32,
    pub unit_ratio: f32,
    pub col_spacing: f32,
    pub row_spacing: f32,
    pub border_spacing: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            birthdate: NaiveDate::from_ymd_opt(2000, 1, 1),
            elapsed_date_bool: true,
            elapsed_date: Local::now().date_naive(),
            life_expectancy: 80,

            //display_weekends: true,
            //display_birthday: true,
            //enable_transparency: false,
            //enable_mouse_passthrough: false,
            color_weekday: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 127),
            color_weekday_elapsed: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 15),
            color_weekend: egui::Color32::from_rgba_unmultiplied(225, 225, 250, 127),
            color_weekend_elapsed: egui::Color32::from_rgba_unmultiplied(225, 225, 250, 15),
            color_birthday: egui::Color32::from_rgba_unmultiplied(255, 47, 47, 127),
            color_birthday_elapsed: egui::Color32::from_rgba_unmultiplied(255, 47, 47, 15),
            color_today: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 127),
            unit_ratio: 0.8,
            col_spacing: 1.0,
            row_spacing: 0.0,
            border_spacing: 1.0,
        }
    }
}
