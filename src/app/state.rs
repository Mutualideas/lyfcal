use chrono::{Datelike, NaiveDate};
use core::f32;
use eframe::egui::*;

use crate::config::config::*;
use crate::ui::component::display::*;

#[derive(Default, Debug)]
pub struct LyfcalApp {
    pub config: Config,
    pub draw_data: DrawData,
    pub show_immediate_viewport: bool,
    //show_deferred_viewport: Arc<AtomicBool>,
}

impl LyfcalApp {
    //When initialized, data from the config is passed to draw_data.
    pub fn initialize(&mut self) {
        self.draw_data.initialize(self.config);
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
