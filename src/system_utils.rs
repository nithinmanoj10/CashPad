use chrono::Local;
use std::process::Command;

pub fn clear_screen() {
    let mut clr_scr = Command::new("clear");
    clr_scr.status().expect("Sorry, couldn't clear the screen");
}

pub fn get_current_date() -> String {
    let curr_time: &str = &format!("{}", Local::now());
    let date: &str = &curr_time[..10];

    let mut final_date = String::from("");
    let date_numbers: Vec<&str> = date.split("-").collect();
    let mut count: i32 = 0;

    for num in date_numbers.iter().rev() {
        final_date.push_str(&format!("{}", num));
        count += 1;

        if count != 3 {
            final_date.push('-')
        }
    }

    final_date
}
