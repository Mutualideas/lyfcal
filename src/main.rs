extern crate image;
#[macro_use]
extern crate conrod;
extern crate piston_window;
extern crate find_folder;
extern crate chrono;

use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};
use chrono::{Datelike};

struct App {
    past_opacity: f32,
    future_opacity: f32,
    life_expectancy: f32,
    birthday: chrono::DateTime<chrono::Local>,
}


    ///// Constants /////

const MARGIN: f64 = 24.0;
const MAX_LIFE_EXPECTANCY: f32 = 100.0;
const BIRTH_LIMIT_OFFSET: f32 = MAX_LIFE_EXPECTANCY - 5.0;


fn main() {


    //////////////////
    ///// Window /////
    //////////////////


    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 512;

    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("lyfcal", [WIDTH, HEIGHT])
            .opengl(OpenGL::V3_2)
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();

    window.set_ups(60);

    let mut app = App {
        past_opacity: 0.0,
        future_opacity: 0.7,
        life_expectancy: 80.0,
        birthday: chrono::Local::now().with_month(1).unwrap().with_day(1).unwrap(),
    };

    //////////////////////////
    ///// User Interface /////
    //////////////////////////


    // Construct our `Ui`.
    let mut ui = conrod::UiBuilder::new().build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let notosans_id = ui.fonts.insert_from_file(font_path).unwrap();
    ui.theme.font_id = Some(notosans_id);
    let font_path = assets.join("fonts/bitstream-vera-sans-mono/Bitstream Vera Sans Mono Roman.ttf");
    let bitstream_id = ui.fonts.insert_from_file(font_path).unwrap();

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_texture_cache =
        conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();

    let ids = Ids::new(ui.widget_id_generator());

    /////////////////////
    ///// Main Loop /////
    /////////////////////


    // Poll events from the window.
    while let Some(event) = window.next() {

        // Convert the piston event to a conrod event.
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            let mut ui = ui.set_widgets();
            gui(&mut ui, &ids, &mut app, bitstream_id);
        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });
    }


    ///////////////////////////////
    ///// Set the background! /////
    ///////////////////////////////

    set_background();

}

///////////////
///// GUI /////
///////////////

widget_ids!{
    struct Ids {
        background,
        title,
        past_opacity,
        past_opacity_label,
        future_opacity,
        future_opacity_label,
        life_expectancy,
        life_expectancy_label,
        birthday_title,
        birthday_year,
        birthday_month,
        birthday_day,
    } 
}

fn gui(ui: &mut conrod::UiCell, ids: &Ids, app: &mut App, title_font: conrod::text::font::Id) {
    use conrod::{widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};
    use chrono::{Datelike};

    widget::Canvas::new()
        .border(0.0)
        .color(conrod::color::WHITE)
        .set(ids.background, ui);

    let mut title = widget::Text::new("lyfcal");
    title.style.font_id = Some(Some(title_font));
    title
        .font_size(38)
        .color(conrod::color::LIGHT_CHARCOAL)
        .mid_top_with_margin_on(ids.background, MARGIN)
        .set(ids.title, ui);

   //Birthday//

    widget::Text::new("when were you born?")
        .font_size(16)
        .color(conrod::color::LIGHT_CHARCOAL)
        .align_middle_x_of(ids.background)
        .down(32.0)
        .set(ids.birthday_title, ui);

    let current_year = chrono::Local::now().year() as f32;

    let date_column_w = (ui.w_of(ids.background).unwrap() - MARGIN * 2.0) / 3.0;
    for new_year in widget::NumberDialer::new(app.birthday.year() as f32, current_year - BIRTH_LIMIT_OFFSET, current_year, 0)
        .color(conrod::color::LIGHT_CHARCOAL)
        .mid_right_with_margin_on(ids.background, MARGIN)
        .down(12.0)
        .w_h(date_column_w, 24.0)
        .border(0.0)
        .label_font_size(12)
        .label_color(conrod::color::WHITE)
        .set(ids.birthday_year, ui)
    {
        if let Some(date_time) = app.birthday.with_year(new_year as i32) { 
            app.birthday = date_time;
        }
    }


    let months = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];

    for new_month in widget::DropDownList::new(&months, Some(app.birthday.month0() as usize))
        .color(conrod::color::LIGHT_CHARCOAL)
        .left(0.0)
        .h(24.0)
        .border(0.0)
        .label_font_size(12)
        .label_color(conrod::color::WHITE)
        .set(ids.birthday_month, ui)
    {
        if let Some(date_time) = app.birthday.with_month((new_month + 1) as u32) { 
            app.birthday = date_time;
        }
    }


    fn last_day_of_month(date: &chrono::DateTime<chrono::Local>) -> u32 {
        use chrono::NaiveDate;
        let year = date.year();
        let month = date.month();
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1)).pred().day()
    }
    let max_day = last_day_of_month(&app.birthday);

    for new_day in widget::NumberDialer::new(app.birthday.day() as f32, 1.0, max_day as f32, 0)
        .color(conrod::color::LIGHT_CHARCOAL)
        .left(0.0)
        .h(24.0)
        .border(0.0)
        .label_font_size(12)
        .label_color(conrod::color::WHITE)
        .set(ids.birthday_day, ui)
    {
        match app.birthday.with_day(new_day as u32) { 
            Some(valid_date) => app.birthday = valid_date,
            None => (),
        }
    }



   //Sliders//
   
    const SLIDER_H: f64 = 24.0;

    fn slider(
        value: &mut f32,
        min: f32,
        max: f32,
        (down_from, distance): (widget::Id, f64),
        label: &str,
        slider_id: widget::Id,
        label_id: widget::Id,
        background: widget::Id,
        ui: &mut conrod::UiCell,
    ) {

        for new_value in widget::Slider::new(*value, min, max)
            .padded_w_of(background, MARGIN)
            .middle_of(background)
            .down_from(down_from, distance)
            .h(SLIDER_H)
            .color(conrod::color::LIGHT_CHARCOAL)
            .border(0.0)
            .set(slider_id, ui)
        {
            *value = new_value;
        }
        
        let font_size = (SLIDER_H / 2.0) as u32;
        widget::Text::new(&label)
            .font_size(font_size)
            .color(conrod::color::WHITE)
            .mid_top_with_margin_on(slider_id, -6.0)
            .graphics_for(slider_id)
            .set(label_id, ui);
    }

    let min_life_expectancy = ((chrono::Local::now().year() - app.birthday.year()) as f32).max(5.0);
    let max_life_expectancy = MAX_LIFE_EXPECTANCY.max(min_life_expectancy);
    app.life_expectancy = app.life_expectancy.min(max_life_expectancy).max(min_life_expectancy);

    let label = format!("life expectancy {} years", app.life_expectancy.trunc());
    slider(&mut app.life_expectancy, min_life_expectancy, max_life_expectancy, (ids.birthday_year, MARGIN), &label, ids.life_expectancy, ids.life_expectancy_label, ids.background, ui);

    let label = format!("past opacity {}%", (app.past_opacity * 100.0).trunc());
    slider(&mut app.past_opacity, 0.0, 1.0, (ids.life_expectancy, MARGIN), &label, ids.past_opacity, ids.past_opacity_label, ids.background, ui);

    let label = format!("future opacity {}%", (app.future_opacity * 100.0).trunc());
    slider(&mut app.future_opacity, 0.0, 1.0, (ids.past_opacity, 4.0), &label, ids.future_opacity, ids.future_opacity_label, ids.background, ui);
  
}




#[cfg(target_os="macos")]
fn set_background() {
    println!("We're on macos!!!!!");
}

#[cfg(target_os="windows")]
fn set_background() {
    println!("We're on windows... yuck!!!!!");
}
