// This module contains structs, impl, functions
// for merchants

use colored::Colorize;
use std::io;
use std::io::Write;
use text_io::read;
use std::fs;

pub struct MerchantRecord {
    pub merchant_id: i32,
    pub merchant_name: String,
}

pub fn display_merchant_table() {
    // Getting all merchant data from merchant.csv
    let merchant_file_contents: String = fs::read_to_string("./src/data/db_merchant.csv").expect("Can't find file merchant.csv");
    let merchant_table: Vec<&str> = merchant_file_contents.lines().collect();
    let merchant_header: Vec<&str> = merchant_table[0].split(",").collect();
    let merchant_data: &[&str] = &merchant_table[1..];
    
    // println!("{:#?}", merchant_table);
    // println!("{:#?}", merchant_header);
    // println!("{:#?}", merchant_data);

    println!("{0: <20} {1: <20}", merchant_header[0].white().dimmed(), merchant_header[1].white().dimmed());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".yellow().bold());

    for data in merchant_data {
        let data: Vec<&str> = data.split(",").collect();
        println!("{0: <20} {1: <20}", data[0], data[1]);
    }
    
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".yellow().bold());
}

pub fn merchant(option: u32) {
    match option {
        1 => {
            println!("View all merchants\n");

            display_merchant_table();

            print!("{}", "Hit enter to go back to Merchant Menu".green().blink());
            io::stdout().flush().unwrap();
            let _dummy_enter: String = read!("{}\n");
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