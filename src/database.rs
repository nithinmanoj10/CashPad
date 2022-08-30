// Module containing structs, functions, enums, etc related
// to the database records used in this application

use crate::display_utils;
use crate::merchant;
use crate::system_utils;
use crate::transaction_type;

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
    pub transaction_amount: i32,
    pub is_debit_id: i32,
    pub is_debit: bool,
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
            is_debit_id: -1,
            is_debit: true,
            transaction_amount: -1,
            category: Vec::new(),
            description: String::from(""),
        }
    }
}

impl TransactionRecord<'_> {
    pub fn input_transaction_record(&mut self) {
        // for choosing merchant number
        loop {
            self.display_input_transaction_menu();
            println!("{}\n", "Choose Merchant:".yellow().bold());

            merchant::display_merchant_table();

            print!("Enter merchant number: ");
            io::stdout().flush().unwrap();
            self.merchant_id = read!("{}\n");

            match merchant::get_merchant_name(self.merchant_id) {
                Ok(merchant_name) => {
                    self.merchant_name = merchant_name;
                    break;
                }
                Err(err) => {
                    self.merchant_id = -1;
                    println!("{}: {}", "Error".red().bold(), err);
                    display_utils::display_retry_operation_message();
                }
            }
        }

        // for choosing transaction type
        loop {
            self.display_input_transaction_menu();

            println!("{}\n", "Choose Transaction Type:".yellow().bold());

            transaction_type::display_transaction_type_table();

            print!("Enter transaction type number: ");
            io::stdout().flush().unwrap();
            self.transaction_type_id = read!("{}\n");

            match transaction_type::get_transaction_type_name(self.transaction_type_id) {
                Ok(name) => {
                    self.transaction_type_name = name;
                    break;
                }
                Err(err) => {
                    self.transaction_type_id = -1;
                    println!("{}: {}", "Error".red().bold(), err);
                    display_utils::display_retry_operation_message();
                }
            }
        }

        // Getting the transaction amount
        self.display_input_transaction_menu();

        println!("{}\n", "Transaction Amount".yellow().bold());
        print!("Enter transaction amount: ");
        io::stdout().flush().unwrap();
        self.transaction_amount = read!("{}\n");

        // checking if money is debited or credited
        loop {
            self.display_input_transaction_menu();

            println!("{}\n", "Debit or Credit?".yellow().bold());
            println!(
                "{}. Debit\n{}. Credit\n",
                "1".cyan().bold(),
                "2".cyan().bold()
            );

            print!("Enter Option: ");
            io::stdout().flush().unwrap();
            let debit_option: i32 = read!("{}\n");

            if debit_option == 1 {
                self.is_debit = true;
                self.is_debit_id = 1;
                break;
            } else if debit_option == 2 {
                self.is_debit = false;
                self.is_debit_id = 1;
                break;
            } else {
                println!("Enter a valid option");
                display_utils::display_retry_operation_message();
            }
        }

        self.display_input_transaction_menu();

        // TODO: Rest of the values

        // Getting the transaction description
        println!("{}\n", "Transaction Description".yellow().bold());
        println!(
            "{}:",
            "Enter description in less than 20 words".cyan().bold()
        );

        self.description = read!("{}\n");

        self.display_input_transaction_menu();

        println!("{:#?}", self);
        display_utils::display_go_back_message();
    }

    pub fn display_input_transaction_menu(&mut self) {
        system_utils::clear_screen();
        println!("🏦 {}", "Add Transaction".cyan().bold());

        if self.merchant_id != -1 {
            println!(
                "\n{}: {}",
                "Merchant Name".yellow().bold(),
                self.merchant_name
            );
        }

        if self.transaction_type_id != -1 {
            println!(
                "{}: {}",
                "Transaction Type".yellow().bold(),
                self.transaction_type_name
            );
        }

        if self.transaction_amount != -1 {
            println!(
                "{}: {}",
                "Transaction Amount".yellow().bold(),
                self.transaction_amount
            );
        }

        if self.is_debit_id == 1 {
            if self.is_debit {
                println!(
                    "{} debited from account",
                    format!("₹{}", self.transaction_amount).red().bold()
                );
            } else {
                println!(
                    "{} credited from account",
                    format!("₹{}", self.transaction_amount).green().bold()
                );
            }
        }

        if self.description != "" {
            println!(
                "{}: {}",
                "Description".yellow().bold(),
                self.description
            );
        }

        println!("");
    }
}
