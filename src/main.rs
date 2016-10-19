extern crate image; 
extern crate conrod;
extern crate piston_window;
extern crate find_folder;

fn main() {
    let img = image::open()
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
