// Contains the entire logic behind the app

use std::process;
use std::io;
use std::io::Write;
use colored::Colorize;
use text_io::read;

use crate::database;
use crate::system_utils;
use crate::display_utils;
use crate::merchant;

pub fn cashpad(option: u32) {
    match option {
        0 => {
            // Exit the app
            println!("\n💖 {} 💖\n", "Thank you, byee".red().bold());
            process::exit(0);
        }

        1 => {
            // Add a transaction record to the database
            let mut new_trans_record: database::TransactionRecord = database::TransactionRecord{..Default::default()};

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
                let merchant_option: u32 = read!("{}\n");
    
                if merchant_option == 0 {
                    break;
                }

                system_utils::clear_screen();
                merchant::merchant(merchant_option);
            }
        
        }

        // TODO:

        _ => {
            println!("Wooow");
        }
    }
}
