use clap::{Arg, Command};
use std::collections::HashSet;
use rust_csv_scripts::{compare_csv}; // Import the compare_csv function from lib.rs
use rust_csv_scripts::encoding::Encoding; // Import the Encoding enum

/// Main function that sets up and parses command-line arguments, then calls the
/// CSV comparison function to compare two CSV files.
///
/// This CLI tool allows users to compare two CSV files and optionally ignore
/// specified columns during comparison. It also supports output file encoding 
/// (UTF-8 or UTF-8 with BOM).
fn main() {
    // Define and parse the command-line arguments using Clap.
    let matches = Command::new("compare_csv")
        .about("Compare two CSV files")
        .arg(Arg::new("file1")
            .help("The first CSV file")
            .required(true) // file1 is required
            .value_parser(clap::value_parser!(String)))
        .arg(Arg::new("file2")
            .help("The second CSV file")
            .required(true) // file2 is required
            .value_parser(clap::value_parser!(String)))
        .arg(Arg::new("ignore")
            .help("Comma-separated list of columns to ignore during comparison")
            .short('i')
            .long("ignore")
            .value_parser(clap::value_parser!(String))) // Optional argument for columns to ignore
        .arg(Arg::new("encoding")
            .help("Output file encoding (utf8 or utf8bom)")
            .short('e')
            .long("encoding")
            .value_parser(clap::value_parser!(String))) // Optional encoding argument
        .get_matches();

    // Extract file paths from the parsed arguments.
    let file1 = matches.get_one::<String>("file1").unwrap(); // Unwrap is safe due to `.required(true)`
    let file2 = matches.get_one::<String>("file2").unwrap();

    // Extract and map the encoding argument to the Encoding enum, or default to UTF-8.
    let encoding = match matches.get_one::<String>("encoding").map(|s| s.as_str()).unwrap_or("utf8") {
        "utf8" => Encoding::Utf8,
        "utf8bom" => Encoding::Utf8Bom,
        _ => {
            // Print an error message if an unsupported encoding is provided.
            eprintln!("Invalid encoding specified. Use 'utf8' or 'utf8bom'.");
            return; // Exit the program in case of an invalid encoding.
        }
    };

    // Parse the ignore columns argument if provided, and convert it into a HashSet.
    let ignore_columns: HashSet<String> = if let Some(cols) = matches.get_one::<String>("ignore") {
        cols.split(',') // Split the string by commas
            .map(|s| s.trim().to_string()) // Trim and convert each column name to a String
            .collect() // Collect into a HashSet to remove duplicates
    } else {
        HashSet::new() // Return an empty HashSet if no columns to ignore are provided
    };

    // Call the CSV comparison function from the library (lib.rs).
    // The function takes two file paths, the encoding type, and the columns to ignore.
    if let Err(e) = compare_csv(
        file1, 
        file2, 
        encoding, 
        &ignore_columns.iter().map(|s| s.as_str()).collect::<Vec<_>>() // Convert HashSet<String> to Vec<&str>
    ) {
        // Print any errors that occur during the comparison process.
        eprintln!("Error comparing files: {}", e);
    }
}