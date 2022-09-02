// All modules, structs, funcs, impls based
// the transaction type of a transaction

// Transaction type may include for example:
// 1. Cash
// 2. ATM Card
// 3. GPay , etc

// TODO: Add remove transaction type

use crate::display_utils;
use colored::Colorize;
use std::fs;
use std::io;
use std::io::Write;
use text_io::read;

#[derive(Debug)]
struct TransactionTypeRecord {
    id: i32,
    type_name: String,
    emoticon: String,
}

impl TransactionTypeRecord {
    fn read_transaction_type_details(&mut self) {
        self.id = get_transaction_type_count() + 1;

        print!("{}: ", "Enter Transaction Type Name".yellow().bold());
        io::stdout().flush().unwrap();
        self.type_name = read!("{}\n");

        print!("{}: ", "Enter Emoticon".yellow().bold());
        io::stdout().flush().unwrap();
        self.emoticon = read!("{}\n");
    }
}

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

pub fn display_transaction_type_table() {
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
            println!("{}\n", "View all Transaction Types".cyan().bold());
            display_transaction_type_table();
            display_utils::display_go_back_message();
        }

        2 => {
            // Add new Transaction Type
            println!("{}\n", "Add new Transaction Type".cyan().bold());

            let mut db_transaction_type = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("./src/data/db_transaction_type.csv")
                .unwrap();

            let mut new_transaction_type: TransactionTypeRecord = TransactionTypeRecord {
                id: 0,
                type_name: String::from(""),
                emoticon: String::from(""),
            };

            new_transaction_type.read_transaction_type_details();

            write!(
                db_transaction_type,
                "{}",
                format!(
                    "\n{},{},{}",
                    new_transaction_type.id,
                    new_transaction_type.type_name,
                    new_transaction_type.emoticon
                )
            )
            .expect("Can't find db_transaction_type.csv");

            println!("\n{}", "Transaction Type successfully added".green().bold());
            display_utils::display_go_back_message();
        }

        _ => {
            println!("\n{}", "Please enter a valid option number".red());
            display_utils::display_go_back_message();
        }
    }
}

fn get_transaction_type_count() -> i32 {
    // Returns the number of transaction type records
    // in the transaction type database i.e db_transaction_type.csv
    let transaction_type_contents = fs::read_to_string("./src/data/db_transaction_type.csv")
        .expect("Can't open file db_transaction_type.csv");
    let transaction_type_lines: Vec<&str> = transaction_type_contents.lines().collect();

    return (transaction_type_lines.len() - 1).try_into().unwrap();
}

pub fn get_transaction_type_name(id: i32) -> Result<String, String> {
    // Returns the transaction type name for the corresponding
    // transaction type id
    // Ok(<transaction_type_name>)
    // Err("Transaction Type not found")

    if id <= 0 {
        return Err("Please enter a positive value".to_string());
    }

    let transaction_type_contents: String =
        fs::read_to_string("./src/data/db_transaction_type.csv")
            .expect("Can't find file db_transaction_type.csv");
    let transaction_type_lines: Vec<&str> = transaction_type_contents.lines().collect();
    let transaction_type_data: &[&str] = &transaction_type_lines[1..];

    for data in transaction_type_data {
        let data: Vec<&str> = data.split(",").collect();
        let data_id: i32 = data[0].parse().unwrap();

        if data_id == id {
            return Ok(data[1].to_string());
        }
    }

    Err("Transaction Type not found".to_string())
}

pub fn get_transaction_type_emoticon(id: i32) -> Result<String, String> {
    // Returns the transaction type emoticon for the corresponding
    // transaction type id
    // Ok(<transaction_type_emoticon>)
    // Err("Transaction Type not found")

    if id <= 0 {
        return Err("Please enter a positive value".to_string());
    }

    let transaction_type_contents: String =
        fs::read_to_string("./src/data/db_transaction_type.csv")
            .expect("Can't find file db_transaction_type.csv");
    let transaction_type_lines: Vec<&str> = transaction_type_contents.lines().collect();
    let transaction_type_data: &[&str] = &transaction_type_lines[1..];

    for data in transaction_type_data {
        let data: Vec<&str> = data.split(",").collect();
        let data_id: i32 = data[0].parse().unwrap();

        if data_id == id {
            return Ok(data[2].to_string());
        }
    }

    Err("Transaction Type not found".to_string())
}
