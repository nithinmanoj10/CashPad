// Contains the entire logic behind the app

use colored::Colorize;
use std::io;
use std::io::Write;
use std::process;
use text_io::read;

use crate::database;
use crate::display_utils;
use crate::merchant;
use crate::system_utils;
use crate::transaction_type;

pub fn cashpad(option: u32) {
    match option {
        0 => {
            // Exit the app
            println!("\n💖 {} 💖\n", "Thank you, byee".red().bold());
            process::exit(0);
        }

        1 => {
            // Add a transaction record to the database
            let mut new_trans_record: database::TransactionRecord = database::TransactionRecord {
                ..Default::default()
            };

            system_utils::clear_screen();
            new_trans_record.input_transaction_record();
            println!("{:?}", new_trans_record);
        }

        3 => {
            // Go to Merchant Main Menu

            loop {
                system_utils::clear_screen();
                display_utils::display_merchant_menu();

                print!("\nEnter Option Number: ");
                io::stdout().flush().unwrap();
                let merchant_option: i32 = read!("{}\n");

                if merchant_option == 0 {
                    break;
                }

                system_utils::clear_screen();
                merchant::merchant(merchant_option);
            }
        }

        4 => {
            // Go to Transaction Type Main Menu

            loop {
                system_utils::clear_screen();
                transaction_type::display_transaction_type_menu();

                print!("\nEnter Option Number: ");
                io::stdout().flush().unwrap();
                let transaction_type_option: i32 = read!("{}\n");

                if transaction_type_option == 0 {
                    break;
                }

                system_utils::clear_screen();
                transaction_type::transaction(transaction_type_option);
            }
        }

        // TODO:
        _ => {
            println!("Wooow");
        }
    }
}
