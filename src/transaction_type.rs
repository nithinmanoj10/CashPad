// All modules, structs, funcs, impls based
// the transaction type of a transaction

// Transaction type may include for example:
// 1. Cash
// 2. ATM Card
// 3. GPay , etc

use crate::display_utils;
use colored::Colorize;
use std::fs;

pub fn display_transaction_type_menu() {
    // Displays the main menu screen for the
    // transaction type i.e
    // 1. View Transaction Types
    // 2. Add New Transaction Type
    println!("💱 {}\n", "Transaction Type Main Menu".yellow().bold());

    println!("{}. View all Transaction Types", "1".cyan());
    println!("{}. Add new Transaction Type", "2".cyan());

    println!("\n{}. Go back to Main Menu", "0".red());
}

fn display_transaction_type_table() {
    let transaction_type_contents: String =
        fs::read_to_string("./src/data/db_transaction_type.csv")
            .expect("Can't open file db_transaction_type.csv");
    let transaction_type_table: Vec<&str> = transaction_type_contents.lines().collect();
    let transaction_type_header: Vec<&str> = transaction_type_table[0].split(",").collect();
    let transaction_type_data: &[&str] = &transaction_type_table[1..];

    println!(
        "{0: <5} {1: <30} {2: <20}",
        transaction_type_header[0].white().dimmed(),
        transaction_type_header[1].white().dimmed(),
        transaction_type_header[2].white().dimmed()
    );

    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            .yellow()
            .bold()
    );

    for data in transaction_type_data {
        let data: Vec<&str> = data.split(",").collect();
        println!("{0: <5} {1: <30} {2: <20}", data[0], data[1], data[2]);
    }

    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            .yellow()
            .bold()
    );
}

pub fn transaction(option: i32) {
    match option {
        1 => {
            // View all Transaction Types
            println!("View all Transaction Types\n");
            display_transaction_type_table();
            display_utils::display_go_back_message();
        }

        _ => {
            println!("\n{}", "Please enter a valid option number".red());
            display_utils::display_go_back_message();
        }
    }
}
