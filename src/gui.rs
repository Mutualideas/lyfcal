use chrono::{Datelike, Local, NaiveDate};
use core::f32;
use eframe::egui::*;

#[derive(Default, Debug)]
pub struct LyfcalApp {
    config: super::config::Config,
    draw_data: super::draw::DrawData,
    show_immediate_viewport: bool,
    //show_deferred_viewport: Arc<AtomicBool>,
}

impl LyfcalApp {
    //When initialized, data from the config is passed to draw_data.
    fn initialize(&mut self) {
        self.draw_data.initialize(&self.config);
    }

    #[cfg(debug_assertions)]
    fn debug_println(&mut self, ui: &mut Ui) {
        if cfg!(debug_assertions) {
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

            ui.label(format!("event number: {}", self.draw_data.events.len()));
        }
    }

    fn draw_config_ui(&mut self, ui: &mut egui::Ui) {
        let mut style = (*ui.ctx().style()).clone();
        style.spacing.item_spacing.x = 4.0;
        ui.ctx().set_style(style);

        egui::Grid::new("lyfcalconfiggrid")
            .min_col_width(grid_col_width(ui, 2))
            .max_col_width(grid_col_width(ui, 2))
            .striped(true)
            .spacing([0.0, 8.0])
            .show(ui, |ui| {
                self.ui_lyfcal_config_heading(ui);
                ui.end_row();
                self.ui_birthdate_picker(ui);
                ui.end_row();
                self.ui_life_expectancy_input(ui);
                ui.end_row();
                self.ui_elapsed_date_picker(ui);
            });
        ui.add_space(8.0);
        egui::Grid::new("displayconfiggrid")
            .min_col_width(grid_col_width(ui, 2))
            .max_col_width(grid_col_width(ui, 2))
            .striped(true)
            .spacing([0.0, 8.0])
            .show(ui, |ui| {
                self.ui_display_config_heading(ui);
                ui.end_row();
                self.ui_weekday_colorpicker(ui);
                ui.end_row();
                self.ui_weekend_colorpicker(ui);
                ui.end_row();
                self.ui_birthday_colorpicker(ui);
                ui.end_row();
                self.ui_today_colorpicker(ui);
                ui.end_row();
                self.ui_unit_ratio_slider(ui);
                ui.end_row();
                self.ui_column_spacing_slider(ui);
                ui.end_row();
                self.ui_row_spacing_slider(ui);
                ui.end_row();
                self.ui_border_spacing_slider(ui);
            });
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(8.0);
        self.draw_initialize_button(ui);
    }

    fn draw_initialize_button(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            if ui.button("initialize").clicked() {
                self.initialize();
                self.draw_data.populate_events();
                self.show_immediate_viewport = true;
                /* == DEFFERRED VIEWPORT ==
                let current_value = self.show_deferred_viewport.load(Ordering::Relaxed);
                self.show_deferred_viewport
                    .store(!current_value, Ordering::Relaxed);
                */
            }
        });
    }

    fn ui_birthdate_picker(&mut self, ui: &mut egui::Ui) {
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
            let date = chrono::NaiveDate::from_ymd_opt(2000, 1, 1); // or some other default
            if ui
                .add(egui_extras::DatePickerButton::new(&mut date.unwrap()))
                .changed()
            {
                self.config.birthdate = Some(date.unwrap());
            }
        }
    }

    fn ui_life_expectancy_input(&mut self, ui: &mut egui::Ui) {
        ui.label("life expectancy:");
        egui::Grid::new("expectancy_grid")
            .min_col_width(ui.available_width())
            .max_col_width(ui.available_width())
            .show(ui, |ui| {
                ui.add_sized(
                    [ui.available_width(), ui.spacing().interact_size.y],
                    egui::DragValue::new(&mut self.config.life_expectancy)
                        .range(1..=120)
                        .suffix(" years"),
                );
                ui.end_row();
                ui.scope(|ui| {
                    let style = ui.style_mut();
                    if let Some(text_style) = style.text_styles.get_mut(&egui::TextStyle::Button) {
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
            });
    }

    fn ui_elapsed_date_picker(&mut self, ui: &mut egui::Ui) {
        ui.label("elapsed date:");
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.config.elapsed_date_bool, "now ")
                .on_hover_text("set current date date to date elapsed");
            ui.add_enabled_ui(!self.config.elapsed_date_bool, |ui| {
                let mut date = Local::now().date_naive();
                if self.config.elapsed_date_bool {
                    self.config.elapsed_date = Local::now().date_naive();
                    if ui
                        .add_sized(
                            [ui.available_width(), ui.spacing().interact_size.y],
                            egui_extras::DatePickerButton::new(&mut date),
                        )
                        .changed()
                    {}
                } else if ui
                    .add_sized(
                        [ui.available_width(), ui.spacing().interact_size.y],
                        egui_extras::DatePickerButton::new(&mut date),
                    )
                    .changed()
                {
                    self.config.elapsed_date = date
                }
            });
        });
    }

    fn ui_lyfcal_config_heading(&mut self, ui: &mut egui::Ui) {
        ui.heading("lyfcal config");
        egui::Grid::new("lyfcalconfigheading")
            .min_col_width(ui.available_width())
            .show(ui, |ui| {
                ui.label("");
                ui.end_row();
                ui.label("");
            });
    }

    fn ui_display_config_heading(&mut self, ui: &mut egui::Ui) {
        ui.heading("display");
        egui::Grid::new("displayheading")
            .min_col_width(grid_col_width(ui, 2))
            .show(ui, |ui| {
                ui.label("");
                ui.end_row();
                ui.label("projected");
                ui.label("elapsed")
            });
    }

    fn ui_weekday_colorpicker(&mut self, ui: &mut egui::Ui) {
        ui.label("weekday colour:");
        egui::Grid::new("weekdaycolorpicker")
            .min_col_width(grid_col_width(ui, 2))
            .show(ui, |ui| {
                ui.color_edit_button_srgba(&mut self.config.color_weekday);
                ui.color_edit_button_srgba(&mut self.config.color_weekday_elapsed);
            });
    }

    fn ui_weekend_colorpicker(&mut self, ui: &mut egui::Ui) {
        ui.label("weekend colour:");
        egui::Grid::new("weekendcolorpicker")
            .min_col_width(grid_col_width(ui, 2))
            .show(ui, |ui| {
                ui.color_edit_button_srgba(&mut self.config.color_weekend);
                ui.color_edit_button_srgba(&mut self.config.color_weekend_elapsed);
            });
    }

    fn ui_birthday_colorpicker(&mut self, ui: &mut egui::Ui) {
        ui.label("birthday colour:");
        egui::Grid::new("birthdaycolorpicker")
            .min_col_width(grid_col_width(ui, 2))
            .show(ui, |ui| {
                ui.color_edit_button_srgba(&mut self.config.color_birthday);
                ui.color_edit_button_srgba(&mut self.config.color_birthday_elapsed);
            });
    }

    fn ui_today_colorpicker(&mut self, ui: &mut egui::Ui) {
        ui.label("current date colour:");
        egui::Grid::new("todaycolorpicker")
            .min_col_width(grid_col_width(ui, 2))
            .show(ui, |ui| {
                ui.color_edit_button_srgba(&mut self.config.color_today);
            });
    }

    fn ui_unit_ratio_slider(&mut self, ui: &mut egui::Ui) {
        ui.label("unit ratio:")
            .on_hover_text("does not affect spacing");
        ui.add_sized(
            [ui.available_width(), ui.spacing().interact_size.y],
            egui::DragValue::new(&mut self.config.unit_ratio)
                .range(0.1..=1.0)
                .speed(0.05)
                .fixed_decimals(2)
                .suffix("u"),
        );
    }

    fn ui_column_spacing_slider(&mut self, ui: &mut egui::Ui) {
        ui.label("column spacing:")
            .on_hover_text("a column represents 7 days/units");
        ui.add_sized(
            [ui.available_width(), ui.spacing().interact_size.y],
            egui::DragValue::new(&mut self.config.col_spacing)
                .range(0.0..=10.0)
                .speed(0.05)
                .fixed_decimals(2)
                .suffix("u"),
        );
    }

    fn ui_row_spacing_slider(&mut self, ui: &mut egui::Ui) {
        ui.label("row spacing:")
            .on_hover_text("spacing between each unit rows");
        ui.add_sized(
            [ui.available_width(), ui.spacing().interact_size.y],
            egui::DragValue::new(&mut self.config.row_spacing)
                .range(0.0..=10.0)
                .speed(0.05)
                .fixed_decimals(2)
                .suffix("u"),
        );
    }

    fn ui_border_spacing_slider(&mut self, ui: &mut egui::Ui) {
        ui.label("border spacing:")
            .on_hover_text("spacing between edge of screen");
        ui.add_sized(
            [ui.available_width(), ui.spacing().interact_size.y],
            egui::DragValue::new(&mut self.config.border_spacing)
                .range(0.0..=20.0)
                .speed(0.05)
                .fixed_decimals(2)
                .suffix("u"),
        );
    }
}

//================================================== EFRAME IMPLEMENTATION ==================================================

impl eframe::App for LyfcalApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            //.frame(egui::Frame::none())
            .show(ctx, |ui| {
                self.draw_config_ui(ui);
                ui.add_space(20.0);
                self.debug_println(ui)
            });

        if self.show_immediate_viewport {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("lyfcal")
                    .with_min_inner_size([480.0, 320.0])
                    .with_transparent(true)
                    .with_maximized(true)
                    .with_decorations(false)
                    //.with_mouse_passthrough(true)
                    .with_fullsize_content_view(true),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default()
                        .frame(egui::Frame::none())
                        .show(ctx, |ui| self.draw_data.draw_lyfcal(ui));

                    //close viewport
                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_immediate_viewport = false;
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
                    .with_min_inner_size([480.0, 320.0])
                    .with_transparent(true)
                    .with_maximized(true)
                    .with_decorations(false)
                    //.with_mouse_passthrough(true)
                    .with_fullsize_content_view(true),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        for days in self.draw_data.events {
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

//================================================== UI FUNCTIONS ==================================================

fn grid_col_width(ui: &egui::Ui, n_col: usize) -> f32 {
    let gap_space = ui.spacing().item_spacing.x * (n_col as f32 - 1.0);
    let grid_w = ui.available_width();
    (grid_w - gap_space) / n_col as f32
}
