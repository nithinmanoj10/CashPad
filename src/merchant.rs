// This module contains structs, impl, functions
// for merchants

use colored::Colorize;
use std::io;
use std::io::Write;
use text_io::read;

pub struct MerchantRecord {
    pub merchant_id: i32,
    pub merchant_name: String,
}

pub fn merchant(option: u32) {
    match option {
        1 => {
            println!("View all merchants\n");

            print!("{}", "Hit enter to go back to Merchant Menu".green().bold());
            io::stdout().flush().unwrap();
            let dummy_enter: String = read!("{}\n");
        }

        2 => {
            println!("Add new merchant");
        }

        3 => {
            println!("Update merchant info");
        }

        _ => {

        }
    }
}