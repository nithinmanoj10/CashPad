use std::process::Command;

pub fn clear_screen() {
    let mut clr_scr = Command::new("clear");
    clr_scr.status().expect("Sorry, couldn't clear the screen");
}
