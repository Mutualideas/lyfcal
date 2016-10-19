extern crate image;
#[macro_use]
extern crate conrod;
extern crate piston_window;
extern crate find_folder;

use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};


struct App {
    past_opacity: f32,
    future_opacity: f32,
}


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
    };

    //////////////////////////
    ///// User Interface /////
    //////////////////////////


    // Construct our `Ui`.
    let mut ui = conrod::UiBuilder::new().build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

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
            gui(&mut ui, &ids, &mut app);
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

widget_ids!{
    struct Ids {
        background,
        title,
        past_opacity,
        past_opacity_label,
        future_opacity,
        future_opacity_label,
    } 
}

fn gui(ui: &mut conrod::UiCell, ids: &Ids, app: &mut App) {
    use conrod::{widget, Borderable, Colorable, Positionable, Sizeable, Widget};

    widget::Canvas::new()
        .color(conrod::color::WHITE)
        .set(ids.background, ui);

    widget::Text::new("lyfcal")
        .font_size(46)
        .color(conrod::color::LIGHT_CHARCOAL)
        .mid_top_with_margin_on(ids.background, 24.0)
        .set(ids.title, ui);

   //Sliders
   
   
    const SLIDER_H: f64 = 24.0;

    fn opacity_slider(
        opacity: &mut f32,
        label: &str,
        slider_id: widget::Id,
        label_id: widget::Id,
        background: widget::Id,
        ui: &mut conrod::UiCell,
    ) {

        for new_opacity in widget::Slider::new(*opacity, 0.0, 1.0)
            .padded_w_of(background, 24.0)
            .middle_of(background)
            .down(16.0)
            .h(SLIDER_H)
            .color(conrod::color::LIGHT_CHARCOAL)
            .border(0.0)
            .set(slider_id, ui)
        {
            *opacity = new_opacity;
        }
        
        let font_size = (SLIDER_H / 2.0) as u32;
        let label = format!("{} opacity {}%", label, (*opacity * 100.0).trunc());
        widget::Text::new(&label)
            .font_size(font_size)
            .color(conrod::color::WHITE)
            .mid_top_with_margin_on(slider_id, -6.0)
            .graphics_for(slider_id)
            .set(label_id, ui);
    }

    opacity_slider(&mut app.past_opacity, "past", ids.past_opacity, ids.past_opacity_label, ids.background, ui);

    opacity_slider(&mut app.future_opacity, "future", ids.future_opacity, ids.future_opacity_label, ids.background, ui);
  
}




#[cfg(target_os="macos")]
fn set_background() {
    println!("We're on macos!!!!!");
}

#[cfg(target_os="windows")]
fn set_background() {
    println!("We're on windows... yuck!!!!!");
}
