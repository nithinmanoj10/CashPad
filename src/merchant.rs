// This module contains structs, impl, functions
// for merchants

use colored::Colorize;
use std::fs;
use std::io;
use std::io::Write;
use text_io::read;

use crate::display_utils;

#[derive(Debug)]
pub struct MerchantRecord {
    pub merchant_id: u32,
    pub merchant_name: String,
}

impl MerchantRecord {
    pub fn read_merchant_details(&mut self) {
        self.merchant_id = get_merchant_count() + 1;

        print!("\n{}: ", "Enter Merchant Name".yellow().bold());
        io::stdout().flush().unwrap();
        self.merchant_name = read!("{}\n");
    }
}

pub fn display_merchant_table() {
    // Getting all merchant data from merchant.csv
    let merchant_file_contents: String =
        fs::read_to_string("./src/data/db_merchant.csv").expect("Can't find file merchant.csv");
    let merchant_table: Vec<&str> = merchant_file_contents.lines().collect();
    let merchant_header: Vec<&str> = merchant_table[0].split(",").collect();
    let merchant_data: &[&str] = &merchant_table[1..];

    // println!("{:#?}", merchant_table);
    // println!("{:#?}", merchant_header);
    // println!("{:#?}", merchant_data);

    println!(
        "{0: <20} {1: <20}",
        merchant_header[0].white().dimmed(),
        merchant_header[1].white().dimmed()
    );
    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            .yellow()
            .bold()
    );

    for data in merchant_data {
        let data: Vec<&str> = data.split(",").collect();
        println!("{0: <20} {1: <20}", data[0], data[1]);
    }

    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            .yellow()
            .bold()
    );
}

pub fn merchant(option: i32) {
    match option {
        1 => {
            println!("View all merchants\n");
            display_merchant_table();
            display_utils::display_go_back_message();
        }

        2 => {
            println!("🧔🏽{}", "Add New Merchant".cyan().bold());

            let mut db_merchant = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("./src/data/db_merchant.csv")
                .unwrap();

            let mut new_merchant: MerchantRecord = MerchantRecord {
                merchant_id: 0,
                merchant_name: String::from(""),
            };

            new_merchant.read_merchant_details();

            write!(
                db_merchant,
                "{}",
                format!("\n{},{}", new_merchant.merchant_id, new_merchant.merchant_name)
            )
            .expect("Can't find db_merchant.csv");

            println!("\n{}", "Merchant successfully added".green().bold());
            display_utils::display_go_back_message();
        }

        _ => {
            println!("\n{}", "Please enter a valid option number".red());
            display_utils::display_go_back_message();
        }
    }
}

fn get_merchant_count() -> u32 {
    // Returns the number of merchant records
    // in the Merchant database i.e db_merchant.csv
    let merchant_contents =
        fs::read_to_string("./src/data/db_merchant.csv").expect("Can't find file db_merchant.rs");
    let merchant_lines: Vec<&str> = merchant_contents.lines().collect();

    return (merchant_lines.len() - 1).try_into().unwrap();
}
