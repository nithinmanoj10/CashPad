use std::fs;
use std::io;
use std::io::Write;

use colored::*;

fn print_table_header(table_header: Vec<&str>) {
    for header in table_header {
        print! {"{: <20}", header};
        io::stdout().flush().unwrap();
    }
    print!(
        "\n{}\n",
        "――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――"
            .yellow()
            .bold()
    );
    io::stdout().flush().unwrap();
}

fn print_financial_data(financial_data: &[&str]) {
    for data in financial_data {
        let row_info: Vec<&str> = data.split(",").collect();
        for info in row_info {
            print! {"{: <20}", info};
            io::stdout().flush().unwrap();
        }
        println!("");
    }
}

fn main() {
    let file_contents: String = fs::read_to_string("data.txt").expect("Can't open your data file");
    let data: Vec<&str> = file_contents.lines().collect();

    let table_header: Vec<&str> = data[0].split(",").collect(); // Contains the table headers
    let financial_data: &[&str] = &data[1..]; // Contains the financial records in csv format

    print_table_header(table_header);
    print_financial_data(financial_data);
}
