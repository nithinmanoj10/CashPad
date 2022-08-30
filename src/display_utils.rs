// This module contains functions that are used to print
// something important to the screen

use colored::Colorize;
use std::io;
use std::io::Write;
use text_io::read;

pub fn print_gamepad() {
    println!("💰 {} 💰", "Welcome to CashPad".yellow().bold());
}

pub fn display_main_menu() {
    // Displays the main menu of CashApp
    print_gamepad();

    println!("\n{}. Add Transaction", "1".cyan());
    println!("{}. View Previous Transactions", "2".cyan());

    println!("\n{}. Go to Merchant Menu", "3".cyan());
    println!("{}. Go to Transaction Type Menu", "4".cyan());

    println!("\n{}. Exit", "0".red());
}

pub fn display_merchant_menu() {
    // Displays the menu screen for Merchant related
    // operations i.e View Merchant, Add Merchant,
    // Update Merchant

    println!("🧔🏽 {}\n", "Merchant Main Menu".yellow().bold());

    println!("{}. View All Merchants", "1".cyan());
    println!("{}. Add New Merchant", "2".cyan());

    println!("\n{}. Go back to Main Menu", "0".red());
}

pub fn display_go_back_message() {
    print!("{}", "Hit enter to go back".green().blink());
    io::stdout().flush().unwrap();
    let _dummy_enter: String = read!("{}\n");
}

pub fn display_retry_operation_message() {
    print!("{}", "Hit enter to retry the operation".yellow().blink());
    io::stdout().flush().unwrap();
    let _dummy_enter: String = read!("{}\n");
}
