// Module containing structs, functions, enums, etc related
// to the database records used in this application

use crate::system_utils;
use colored::Colorize;
use std::io;
use std::io::Write;
use text_io::read;

#[derive(Debug)]
pub struct TransactionRecord<'a> {
    pub id: i32,
    pub date: String,
    pub merchant_id: i32,
    pub merchant_name: String,
    pub transaction_type_id: i32,
    pub transaction_type_name: String,
    pub transaction_medium_id: i32,
    pub transaction_medium_name: String,
    pub transaction_amount: i32,
    pub category: Vec<&'a str>,
    pub description: String,
}

impl Default for TransactionRecord<'_> {
    fn default() -> TransactionRecord<'static> {
        TransactionRecord {
            id: -1,
            date: String::from(""),
            merchant_id: -1,
            merchant_name: String::from(""),
            transaction_type_id: -1,
            transaction_type_name: String::from(""),
            transaction_medium_id: -1,
            transaction_medium_name: String::from(""),
            transaction_amount: -1,
            category: Vec::new(),
            description: String::from(""),
        }
    }
}

impl TransactionRecord<'_> {
    pub fn input_transaction_record(&mut self) {
        self.display_input_transaction_menu();

        println!("{}\n", "Choose Merchant:".yellow().bold());
        // TODO: Print out all the merchants added so far
        print!("Enter merchant number: ");
        io::stdout().flush().unwrap();
        self.merchant_id = read!("{}\n");

        self.display_input_transaction_menu();

        println!("{}\n", "Choose Transaction Type:".yellow().bold());
        // TODO: Print out all the transactions types added so far
        print!("Enter transaction type number: ");
        io::stdout().flush().unwrap();
        self.transaction_type_id = read!("{}\n");

        self.display_input_transaction_menu();

        println!("{}\n", "Choose Transaction Medium:".yellow().bold());
        // TODO: Print out all the transaction medium added so far
        print!("Enter transaction medium number: ");
        io::stdout().flush().unwrap();
        self.transaction_medium_id = read!("{}\n");

        self.display_input_transaction_menu();

        println!("{}\n", "Transaction Amount:".yellow().bold());
        // TODO: Print out all the transaction medium added so far
        print!("Enter transaction amount: ");
        io::stdout().flush().unwrap();
        self.transaction_amount = read!("{}\n");

        self.display_input_transaction_menu();

        // TODO: Rest of the values
    }

    pub fn display_input_transaction_menu(&mut self) {
        system_utils::clear_screen();
        println!("🏦 {}", "Add Transaction".cyan().bold());

        if self.merchant_id != -1 {
            println!("\n{}: {}", "Merchant Id".yellow().bold(), self.merchant_id)
        }

        if self.transaction_type_id != -1 {
            println!(
                "{}: {}",
                "Transaction Type ID".yellow().bold(),
                self.transaction_type_id
            )
        }

        if self.transaction_medium_id != -1 {
            println!(
                "{}: {}",
                "Transaction Medium ID".yellow().bold(),
                self.transaction_medium_id
            )
        }

        if self.transaction_amount != -1 {
            println!(
                "{}: {}",
                "Transaction Amount".yellow().bold(),
                self.transaction_amount
            )
        }

        println!("");
    }
}
