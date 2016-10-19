extern crate image; 
extern crate conrod;
extern crate piston_window;
extern crate find_folder;

use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};

fn main() {


    //////////////////
    ///// Window /////
    //////////////////


    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 728;

    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("lyfcal", [WIDTH, HEIGHT])
            .opengl(OpenGL::V3_2)
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();

    window.set_ups(60);


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


#[cfg(target_os="macos")]
fn set_background() {
    println!("We're on macos!!!!!");
}

#[cfg(target_os="windows")]
fn set_background() {
    println!("We're on windows... yuck!!!!!");
}
