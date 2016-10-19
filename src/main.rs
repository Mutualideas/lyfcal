fn main() {
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
