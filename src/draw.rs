use chrono::{Datelike, NaiveDate, Weekday};
use eframe::egui::*;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
struct Matrix {
    //Each column contains 7 units for each day of the week.
    col: usize,
    row: usize,
    unit_size: f32,
}

#[derive(Default, Debug)]
pub struct DrawData {
    pub config: super::config::Config,
    pub events: BTreeMap<NaiveDate, String>,
}

impl DrawData {
    pub fn initialize(&mut self, config: &super::config::Config) {
        self.config = config.clone();
    }

    //Function to populate events with every day from the birthdate up to the end of the life expectancy
    pub fn populate_events(&mut self) {
        self.events.clear();

        let birth_year = self.config.birthdate.unwrap().year();
        let end_year = birth_year + self.config.life_expectancy;
        let duration = NaiveDate::from_ymd_opt(
            end_year,
            self.config.birthdate.unwrap().month(),
            self.config.birthdate.unwrap().day(),
        )
        .unwrap()
        .signed_duration_since(self.config.birthdate.unwrap())
        .num_days() as i32;
        let mut date_counter = self.config.birthdate.unwrap();
        //Extra day to account of birthday
        for _ in 0..(duration + 1) {
            self.events
                .insert(date_counter, format!("{}", date_counter.weekday()));
            date_counter += chrono::Duration::days(1)
        }
    }

    //Calculate to maximize unit size/spacing for the given screen space and spacing
    fn calculate_matrix(&self, ui: &mut Ui) -> Matrix {
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
            if (col_num * 7) * row_num >= (self.events.len() - birthday_offset) {
                col_output = col_num;
                row_output = row_num;
                size_output = unit_size;
                break;
            }
        }
        Matrix {
            col: col_output,
            row: row_output,
            unit_size: size_output,
        }
    }

    //Offset unit body to account for empty column
    fn col_offset(&self, matrix: &Matrix) -> f32 {
        let max_unit_num = matrix.col * matrix.row * 7;
        let unit_num = self.events.len()
            + self
                .config
                .birthdate
                .unwrap()
                .weekday()
                .num_days_from_monday() as usize;
        let col_capacity = matrix.row * 7;
        let mut offset = 0.0;

        //
        //Line below occasionally causes crashes during window resizing due to overflow issue if left at usize

        if (max_unit_num as i32 - unit_num as i32) >= col_capacity as i32 {
            offset = matrix.unit_size * (7.0 + self.config.col_spacing) / (matrix.col as f32 - 2.0);
        };
        offset
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
                + self.config.col_spacing * (col as f32)
                - 1.0)
            + col as f32 * col_offset
            + offset.0;
        let y = unit_size
            * (self.config.border_spacing + row as f32 + self.config.row_spacing * (row as f32))
            + offset.1;
        Rect::from_min_size(
            egui::pos2(
                x + ((1.0 - self.config.unit_ratio) / 2.0) * unit_size,
                y + ((1.0 - self.config.unit_ratio) / 2.0) * unit_size,
            ),
            egui::vec2(
                self.config.unit_ratio * unit_size,
                self.config.unit_ratio * unit_size,
            ),
        )
    }

    //Draw logic
    fn draw_unit(&self, ui: &mut Ui, rect: Rect, date: NaiveDate, unit_size: f32) {
        // Determine the date type based on whether it's a weekday or weekend.
        let date_type = if is_weekday(date) {
            DateType::Weekday
        } else {
            DateType::Weekend
        };

        let rounding = unit_size * self.config.unit_ratio / 16.0;

        // Determine if the date is elapsed or in the future.
        let is_elapsed = date <= self.config.elapsed_date;

        // Use a match statement to determine the color based on the tuple of (is_elapsed, date_type).
        let color = match (is_elapsed, date_type) {
            (false, DateType::Weekday) => self.config.color_weekday,
            (true, DateType::Weekday) => self.config.color_weekday_elapsed,
            (false, DateType::Weekend) => self.config.color_weekend,
            (true, DateType::Weekend) => self.config.color_weekend_elapsed,
        };

        // Draw the rectangle with the determined color.
        ui.painter().rect_filled(rect, rounding, color);

        if is_birthday(date, self.config.birthdate.unwrap()) && !is_elapsed {
            ui.painter()
                .rect_filled(rect, rounding, self.config.color_birthday);
        } else if is_birthday(date, self.config.birthdate.unwrap()) && is_elapsed {
            ui.painter()
                .rect_filled(rect, rounding, self.config.color_birthday_elapsed);
        };

        if date == self.config.elapsed_date {
            ui.painter().rect_stroke(
                rect,
                rounding,
                Stroke::new(unit_size * 0.1 + 0.5, self.config.color_today),
            );
        };
    }

    pub fn draw_lyfcal(&mut self, ui: &mut Ui) {
        //To allow exception to the first week shown where the week doesn't begin on a monday, extra day added for the birthday itself.
        let matrix = self.calculate_matrix(ui);
        let mut date_counter = self.config.birthdate.expect("No birthdate given.");

        for col in 0..(matrix.col) {
            for row in 0..(matrix.row) {
                for _ in date_counter.weekday().num_days_from_monday() as usize..7 {
                    if date_counter > *self.events.last_key_value().unwrap().0 {
                        break;
                    }
                    self.draw_unit(
                        ui,
                        self.calculate_pos(
                            col,
                            row,
                            matrix.unit_size,
                            date_counter,
                            self.col_offset(&matrix),
                            (0.0, 0.0),
                        ),
                        date_counter,
                        matrix.unit_size,
                    );
                    date_counter += chrono::Duration::days(1);
                }
            }
        }
    }
}

//================================================== MISC. FUNCTIONS ==================================================

fn is_birthday(date: NaiveDate, birthdate: NaiveDate) -> bool {
    date.month() == birthdate.month() && date.day() == birthdate.day()
}

fn is_weekday(date: NaiveDate) -> bool {
    !matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

enum DateType {
    Weekday,
    Weekend,
}
