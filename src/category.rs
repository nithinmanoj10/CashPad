// This module contains functions, impl, etc
// for category

use std::fs;
use colored::Colorize;
use crate::display_utils;

pub fn display_category_menu() {
    // Displays the following category menu:
    //
    // 1. View all Categories
    // 2. Add new Category

    println!("🏷️ {}\n", "Category Main Menu".yellow().bold());

    println!("{}. View all Categories", "1".cyan());
    println!("{}. Add new Category", "2".cyan());

    println!("\n{}. Go back to Main Menu", "0".red());
}

fn display_category_table() {
    let category_contents: String =
        fs::read_to_string("./src/data/db_category.csv").expect("Can't open file db_category.csv");
    let category_table: Vec<&str> = category_contents.lines().collect();
    let category_header: Vec<&str> = category_table[0].split(",").collect();
    let category_data: &[&str] = &category_table[1..];

    println!(
        "{0: <5} {1: <20}",
        category_header[0].white().dimmed(),
        category_header[1].white().dimmed()
    );

    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            .yellow()
            .bold()
    );

    for data in category_data {
        let data: Vec<&str> = data.split(",").collect();
        println!("{0: <5} {1: <20}", data[0], data[1]);
    }

    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            .yellow()
            .bold()
    );
}

pub fn category(option: i32) {
    match option {
        1 => {
            // View all Categories
            println!("{}\n", "View all Categories".cyan().bold());
            display_category_table();
            display_utils::display_go_back_message();
        }

        _ => {
            println!("\n{}", "Please enter a valid option number".red());
            display_utils::display_go_back_message();
        }
    }
}
