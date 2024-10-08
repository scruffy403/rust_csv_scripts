// src/bin/compare_csv.rs

use clap::{Arg, Command};
use std::collections::HashSet;
use rust_csv_scripts::{compare_csv}; // Import the compare_csv function from lib.rs
use rust_csv_scripts::encoding::Encoding; // Import the Encoding enum

fn main() {
    let matches = Command::new("compare_csv")
        .about("Compare two CSV files")
        .arg(Arg::new("file1")
            .help("The first CSV file")
            .required(true)
            .value_parser(clap::value_parser!(String)))
        .arg(Arg::new("file2")
            .help("The second CSV file")
            .required(true)
            .value_parser(clap::value_parser!(String)))
        .arg(Arg::new("ignore")
            .help("Comma-separated list of columns to ignore during comparison")
            .short('i')
            .long("ignore")
            .value_parser(clap::value_parser!(String)))
        .arg(Arg::new("encoding")
            .help("Output file encoding (utf8 or utf8bom)")
            .short('e')
            .long("encoding")
            .value_parser(clap::value_parser!(String)))
        .get_matches();

    let file1 = matches.get_one::<String>("file1").unwrap();
    let file2 = matches.get_one::<String>("file2").unwrap();

    // Safely unwrap the encoding argument or default to utf8
    let encoding = match matches.get_one::<String>("encoding").map(|s| s.as_str()).unwrap_or("utf8") {
        "utf8" => Encoding::Utf8,
        "utf8bom" => Encoding::Utf8Bom,
        _ => {
            eprintln!("Invalid encoding specified. Use 'utf8' or 'utf8bom'.");
            return;
        }
    };

    let ignore_columns: HashSet<String> = if let Some(cols) = matches.get_one::<String>("ignore") {
        cols.split(',')
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        HashSet::new()
    };

    // Call the CSV comparison function from lib.rs
    if let Err(e) = compare_csv(file1, file2, encoding, &ignore_columns.iter().map(|s| s.as_str()).collect::<Vec<_>>()) {
        eprintln!("Error comparing files: {}", e);
    }
}