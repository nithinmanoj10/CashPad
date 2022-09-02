// Module containing structs, functions, enums, etc related
// to the database records used in this application

use crate::category;
use crate::display_utils;
use crate::merchant;
use crate::system_utils;
use crate::transaction_type;

use colored::Colorize;
use std::fs;
use std::io;
use std::io::Write;
use text_io::read;

#[derive(Debug)]
pub struct TransactionRecord {
    pub id: i32,
    pub date: String,
    pub merchant_id: i32,
    pub merchant_name: String,
    pub transaction_type_id: i32,
    pub transaction_type_name: String,
    pub transaction_amount: i32,
    pub is_debit_id: i32,
    pub is_debit: bool,
    pub category_ids: Vec<i32>,
    pub category_names: Vec<String>,
    pub description: String,
}

impl Default for TransactionRecord {
    fn default() -> TransactionRecord {
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
            category_ids: Vec::new(),
            category_names: Vec::new(),
            description: String::from(""),
        }
    }
}

impl TransactionRecord {
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

        loop {
            self.display_input_transaction_menu();

            println!(
                "{}\n",
                "Choose Categories (enter 0 once done)".yellow().bold()
            );

            category::display_category_table();

            print!("Enter Category Number: ");
            io::stdout().flush().unwrap();
            let category_number: i32 = read!("{}\n");

            // Once all categories are added
            if category_number == 0 {
                break;
            }

            match category::get_category_name(category_number) {
                Ok(name) => {
                    self.category_ids.push(category_number);
                    self.category_names.push(name);
                }
                Err(err) => {
                    println!("{}: {}", "Error".red().bold(), err);
                    display_utils::display_retry_operation_message();
                }
            }
        }

        self.display_input_transaction_menu();

        // Getting the transaction description
        println!("{}\n", "Transaction Description".yellow().bold());
        println!(
            "{}:",
            "Enter description in less than 20 words".cyan().bold()
        );

        self.description = read!("{}\n");

        self.display_input_transaction_menu();

        // adding date
        self.date = system_utils::get_current_date();

        let mut db_transactions = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("./src/data/db_transactions.csv")
            .unwrap();

        let mut category_ids_string: String = String::from("");
        let mut category_id_count: i32 = 0;
        for id in &self.category_ids {
            category_ids_string.push_str(&format!("{}", id));
            category_id_count += 1;

            if category_id_count < self.category_ids.len().try_into().unwrap() {
                category_ids_string.push_str("-");
            }
        }

        // setting transactions record id
        self.id = get_transactions_count() + 1;

        write!(
            db_transactions,
            "{}",
            format!(
                "\n{},{},{},{},{},{},{},{}",
                self.id,
                self.date,
                self.merchant_id,
                self.transaction_type_id,
                self.transaction_amount,
                self.is_debit,
                category_ids_string,
                self.description
            )
        )
        .expect("Can't open db_transactions.csv");

        println!("\n{}", "Transaction successfully added".green().bold());
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
                    "{} credited into account",
                    format!("₹{}", self.transaction_amount).green().bold()
                );
            }
        }

        // displaying transaction categories
        if self.category_ids.len() != 0 {
            print!("{}", "Categories:".yellow().bold());
            io::stdout().flush().unwrap();

            for category in &self.category_names {
                print!(" {}", category);
                io::stdout().flush().unwrap();
            }
            println!("");
        }

        if self.description != "" {
            println!("{}: {}", "Description".yellow().bold(), self.description);
        }

        println!("");
    }
}

pub fn display_transaction_table() {
    // Displays all the transaction records
    let transactions_content: String = fs::read_to_string("./src/data/db_transactions.csv")
        .expect("Can't open file db_transactions.csv");
    let transactions_lines: Vec<&str> = transactions_content.lines().collect();
    let transactions_data: &[&str] = &transactions_lines[1..];

    // printing the headers
    println!(
        "{0: <5} {1: <12} {2: <25} {3: <6} {4: <10} {5: <35} {6: <40}",
        "Id".dimmed(),
        "Date".dimmed(),
        "Merchant".dimmed(),
        "Type".dimmed(),
        "Amount".dimmed(),
        "Category".dimmed(),
        "Description".dimmed()
    );

    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            .yellow()
            .bold()
    );

    // displaying all the transaction data
    for data in transactions_data {
        let data: Vec<&str> = data.split(",").collect();

        // Transaction ID
        print!("{0: <5}", data[0]);
        io::stdout().flush().unwrap();

        // Date
        print!(" {0: <12}", data[1]);
        io::stdout().flush().unwrap();

        // Merchant Name
        let merchant_id: i32 = data[2].parse().unwrap();
        match merchant::get_merchant_name(merchant_id) {
            Ok(name) => {
                print!(" {0: <25}", name);
            }
            Err(_err) => {
                print!(" {0: <25}", "-");
            }
        }
        io::stdout().flush().unwrap();

        // Transaction Type Emoticon
        let transaction_type_id: i32 = data[3].parse().unwrap();
        match transaction_type::get_transaction_type_emoticon(transaction_type_id) {
            Ok(emoticon) => {
                print!(" {0: <6}", emoticon);
            }
            Err(_err) => {
                print!(" {0: <6}", "-");
            }
        }
        io::stdout().flush().unwrap();

        // Transaction Amount
        // checking if its a debited amount or not
        match data[5] {
            "true" => {
                print!("{0: <10}", data[4].red());
            }
            "false" => {
                print!("{0: <10}", data[4].green());
            }
            _ => {}
        }

        // Category
        let category_ids: Vec<&str> = data[6].split("-").collect();
        let mut category_string: String = String::from("");
        let mut category_count: i32 = 0;
        for id in &category_ids {
            let id: i32 = id.parse().unwrap();

            match category::get_category_name(id) {
                Ok(name) => {
                    category_string.push_str(&name);
                }
                Err(_err) => {
                    category_string.push_str("N/A");
                }
            }

            category_count += 1;

            if category_count < category_ids.len().try_into().unwrap() {
                category_string.push_str(", ");
            }
        }

        if category_string.len() > 30 {
            print!(" {0: <35}", format!("{} {}", &category_string[..29], "..."))
        } else {
            print!(" {0: <35}", category_string);
        }

        // display the transaction description
        print!(" {0: <40}", data[7]);

        println!("");
    }

    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            .yellow()
            .bold()
    );
}

fn get_transactions_count() -> i32 {
    // Returns the number of transaction records in the
    // database db_transactions.csv
    let transactions_contents: String = fs::read_to_string("./src/data/db_transactions.csv").expect("Can't open file db_transactions.csv");
    let transactions_lines: Vec<&str> = transactions_contents.lines().collect();

    return (transactions_lines.len() - 1).try_into().unwrap();
}