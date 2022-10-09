use std::io;
use std::io::Write;
use text_io::read;

mod cashpad;
mod category;
mod database;
mod display_utils;
mod filter_search;
mod merchant;
mod system_utils;
mod transaction_type;

fn main() {
    loop {
        system_utils::clear_screen();
        display_utils::display_main_menu();

        print!("\nEnter option number: ");
        io::stdout().flush().unwrap();

        let menu_option: u32 = read!("{}\n"); // Menu option number entered by user

        cashpad::cashpad(menu_option);
    }
}
