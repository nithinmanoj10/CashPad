// This module contains functions, impl, etc
// for category

use crate::display_utils;
use colored::Colorize;
use std::fs;
use std::io;
use std::io::Write;
use text_io::read;

struct CategoryRecord {
    id: i32,
    category_name: String,
}

impl CategoryRecord {
    fn read_category_details(&mut self) {
        self.id = get_category_count() + 1;

        print!("{}: ", "Enter Category Name".yellow().bold());
        io::stdout().flush().unwrap();
        self.category_name = read!("{}\n");
    }
}

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

pub fn display_category_table() {
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
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".yellow().bold()
    );

    for data in category_data {
        let data: Vec<&str> = data.split(",").collect();
        println!("{0: <5} {1: <20}", data[0], data[1]);
    }

    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".yellow().bold()
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

        2 => {
            // Add new Category
            println!("{}\n", "Add new Category".cyan().bold());

            let mut db_category = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("./src/data/db_category.csv")
                .unwrap();

            let mut new_category: CategoryRecord = CategoryRecord {
                id: -1,
                category_name: String::from(""),
            };

            new_category.read_category_details();

            write!(
                db_category,
                "{}",
                format!("\n{},{}", new_category.id, new_category.category_name)
            )
            .expect("Can't open db_category.csv");

            println!("\n{}", "Category successfully added".green().bold());
            display_utils::display_go_back_message();
        }

        _ => {
            println!("\n{}", "Please enter a valid option number".red());
            display_utils::display_go_back_message();
        }
    }
}

fn get_category_count() -> i32 {
    // Returns the number of category records
    // in the database db_category.csv

    let category_contents: String =
        fs::read_to_string("./src/data/db_category.csv").expect("Can't open file db_category.csv");

    let category_lines: Vec<&str> = category_contents.lines().collect();
    return (category_lines.len() - 1).try_into().unwrap();
}

pub fn get_category_name(id: i32) -> Result<String, String> {
    // Returns the category name for the corresponding
    // category id
    // Ok(<category_name>)
    // Err("Category not found")

    if id <= 0 {
        return Err("Please enter a positive value".to_string());
    }

    let category_contents: String =
        fs::read_to_string("./src/data/db_category.csv").expect("Can't find file db_category.csv");
    let category_lines: Vec<&str> = category_contents.lines().collect();
    let category_data: &[&str] = &category_lines[1..];

    for data in category_data {
        let data: Vec<&str> = data.split(",").collect();
        let data_id: i32 = data[0].parse().unwrap();

        if data_id == id {
            return Ok(data[1].to_string());
        }
    }

    Err("Category not found".to_string())
}
