use chrono::{Datelike, Local, NaiveDate, Weekday};
use eframe::egui::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Config {
    pub birthdate: Option<NaiveDate>,
    pub elapsed_date_bool: bool,
    pub elapsed_date: chrono::NaiveDate,
    pub life_expectancy: i32,

    pub display_colour: egui::Color32,
    pub display_colour2: egui::Color32,
    pub display_colour3: egui::Color32,
    pub display_colour4: egui::Color32,
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
            display_colour: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 100),
            display_colour2: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20),
            display_colour3: egui::Color32::from_rgba_unmultiplied(225, 225, 250, 100),
            display_colour4: egui::Color32::from_rgba_unmultiplied(225, 225, 250, 20),
            col_spacing: 1.0,
            row_spacing: 0.0,
            border_spacing: 1.0,
        }
    }
}

#[derive(Default, Debug)]
pub struct DrawData {
    pub config: Config,
    pub events: BTreeMap<NaiveDate, String>,
}

impl DrawData {
    pub fn initialize(&mut self, config: &Config) {
        self.config = config.clone();
    }

    //Function to populate events with every day from the birthdate up to the end of the life expectancy
    pub fn populate_events(&mut self) {
        self.events.clear();
        if let Some(birthdate) = self.config.birthdate {
            let birth_year = birthdate.year();
            let end_year = birth_year + self.config.life_expectancy;
            let mut current_date = birthdate;
            let mut expectancy_day_count = 0;

            //Iterate through each year from birth year to end year for leap years.
            for year in birth_year..end_year {
                let days_between_bday =
                    NaiveDate::from_ymd_opt(year + 1, birthdate.month(), birthdate.day())
                        .unwrap()
                        .signed_duration_since(
                            NaiveDate::from_ymd_opt(year, birthdate.month(), birthdate.day())
                                .unwrap(),
                        )
                        .num_days();
                expectancy_day_count += days_between_bday;
                for _ in 0..days_between_bday {
                    if current_date > birthdate + chrono::Duration::days(expectancy_day_count) {
                        break; // Stop if we exceed the life expectancy
                    }
                    self.events
                        .insert(current_date, format!("{}", current_date.weekday()));
                    current_date = match current_date.checked_add_signed(chrono::Duration::days(1))
                    {
                        Some(date) => date,
                        None => break, // Exit the loop if overflow occurs
                    };
                }
            }
        }
    }

    //Calculate to maximize unit size/spacing for the given screen space and spacing
    fn calculate_matrix(&self, ui: &mut Ui) -> (usize, usize, f32) {
        let event_no = self.events.len();
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
                    + 2.0 * self.config.border_spacing);
            //Work out the maximum number of rows given the unit size.
            let row_num = ((ui.available_height() - 2.0 * self.config.border_spacing * unit_size
                + self.config.row_spacing * unit_size)
                / (unit_size + self.config.row_spacing * unit_size))
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
    fn col_offset(&self, matrix: (usize, usize, f32)) -> f32 {
        let max_unit_num = matrix.0 * matrix.1 * 7;
        let unit_num = self.events.len()
            + self
                .config
                .birthdate
                .unwrap()
                .weekday()
                .num_days_from_monday() as usize;
        let col_capacity = matrix.1 * 7;

        //the line below crashes with overflow issue if left at usize
        if (max_unit_num as i32 - unit_num as i32) >= col_capacity as i32 {
            matrix.2 * (7.0 + self.config.col_spacing) / (matrix.1 as f32 - 2.0)
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
        offset: (f32, f32),
    ) -> Rect {
        let x = unit_size
            * (self.config.border_spacing
                + date.weekday().number_from_monday() as f32
                + 7.0 * col as f32
                + self.config.col_spacing * (col as f32 - 1.0))
            + col as f32 * col_offset
            + offset.0;
        let y = unit_size
            * (self.config.border_spacing
                + row as f32
                + self.config.row_spacing * (row as f32 - 1.0))
            + offset.1;
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
    }

    pub fn draw_lyfcal(&mut self, ui: &mut Ui) {
        //To allow exception to the first week shown where the week doesn't begin on a monday, extra day added for the birthday itself.
        let matrix_scale: (usize, usize, f32) = self.calculate_matrix(ui);
        let mut date_incre = self.config.birthdate.expect("No birthdate given.");

        for col in 0..(matrix_scale.0) {
            for row in 0..(matrix_scale.1) {
                for _ in date_incre.weekday().num_days_from_monday() as usize..7 {
                    if date_incre > *self.events.last_key_value().unwrap().0 {
                        break;
                    }
                    self.draw_unit(
                        ui,
                        self.calculate_pos(
                            col,
                            row,
                            matrix_scale.2,
                            date_incre,
                            self.col_offset(matrix_scale),
                            (0.0, 0.0),
                        ),
                        date_incre,
                        matrix_scale.2,
                    );
                    date_incre += chrono::Duration::days(1);
                }
            }
        }
    }
}

//================================================== MISC. FUNCTIONS ==================================================

fn is_weekday(date: NaiveDate) -> bool {
    !matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}
