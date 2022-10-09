// Contains modules and functions to filter search and print
// transactions

use crate::display_utils;
use crate::merchant;
use crate::system_utils;
use crate::database;
use colored::Colorize;
use std::fs;
use std::io;
use std::io::Write;
use text_io::read;

pub fn filter_search(option: i32) {
    match option {
        1 => {
            // Filter search by Time
        }

        2 => {
            // Filter search by Merchant
            loop {
                system_utils::clear_screen();

                println!("{}\n", "Filter Search by Merchant".yellow().bold());

                merchant::display_merchant_table();

                print!("\nEnter Merchant Number: ");
                io::stdout().flush().unwrap();
                let merchant_number: i32 = read!("{}\n");

                if merchant_number < 1
                    || merchant_number > merchant::get_merchant_count().try_into().unwrap()
                {
                    println!("{}", "Please enter a valid ID".red());
                    display_utils::display_retry_operation_message();
                } else {
                    system_utils::clear_screen();
                    display_merchant_transaction(merchant_number);
                    display_utils::display_go_back_message();
                    break;
                }
            }
        }

        3 => {
            // Filter search by Payment Type
        }

        4 => {
            // Filter search by Category
        }

        _ => {
            println!("\n{}", "Please enter a valid option number".red());
            display_utils::display_go_back_message();
        }
    }
}

fn display_merchant_transaction(merchant_number: i32) {
    // Displays all the transaction records of the merchant
    // with the corresponding merchant_number

    let transactions_content: String = fs::read_to_string("./src/data/db_transactions.csv")
        .expect("Can't open file db_transactions.csv");
    let transactions_lines: Vec<&str> = transactions_content.lines().collect();
    let transactions_data: &[&str] = &transactions_lines[1..];
    
    // merchant_transaction_records will contain all the
    // transaction records of the merchant we are searching
    let mut merchant_transaction_records: Vec<&str> = Vec::new();
    
    // iterating through all the transaction records and
    // adding the record of the merchant we are searching for
    // into merchant_transaction_records
    for csv_data in transactions_data {
        let data: Vec<&str> = csv_data.split(",").collect();
        let curr_merchant_id: i32 = data[2].parse().unwrap();
    
        // the merchant we are searching for
        if curr_merchant_id == merchant_number {
            merchant_transaction_records.push(csv_data);
        }
    }
    
    database::display_transaction_records(merchant_transaction_records.clone());
    merchant::display_merchant_transactions_stats(merchant_transaction_records.clone());
}
