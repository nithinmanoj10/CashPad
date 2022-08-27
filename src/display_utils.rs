// This module contains functions that are used to print
// something important to the screen

use colored::Colorize;

pub fn print_gamepad() {
    println!("💰 {} 💰", "Welcome to CashPad".yellow().bold());
}

pub fn display_main_menu() {
    // Displays the main menu of CashApp
    print_gamepad();

    println!("\n{}. Add Transaction", "1".cyan());
    println!("{}. View Previous Transactions", "2".cyan());

    println!("\n{}. Go to Merchant Menu", "3".cyan());

    println!("\n{}. Exit", "0".red());
}

pub fn display_merchant_menu() {
    // Displays the menu screen for Merchant related
    // operations i.e View Merchant, Add Merchant,
    // Update Merchant

    println!("🧔🏽 {}\n", "Merchant Main Menu".yellow().bold());

    println!("{}. View All Merchants", "1".cyan());
    println!("{}. Add New Merchant", "2".cyan());
    println!("{}. Update Merchant Info", "3".cyan());

    println!("\n{}. Go back to Main Menu", "0".red());
}