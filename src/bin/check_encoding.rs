use clap::{Arg, Command};
use std::path::Path;
use rust_csv_scripts::encoding::detect_encoding;

/// Main entry point for the `check_encoding` command-line tool.
///
/// This tool checks the encoding of a specified file by reading its content
/// and using the `detect_encoding` function from the `encoding` module.
/// It prints the detected encoding and confidence level to the console.
fn main() {
    // Set up command-line argument parsing using Clap.
    let matches = Command::new("check_encoding")
        .about("Check the encoding of a file") // Brief description of the tool's purpose.
        .arg(Arg::new("file") // Define the "file" argument.
            .help("The file to check") // Help text for the argument.
            .required(true) // This argument is mandatory.
            .value_parser(clap::value_parser!(String))) // Specify that the argument should be a String.
        .get_matches(); // Parse the command-line arguments.

    // Retrieve the file path argument from the matches, safely unwrapping since it's required.
    let file_path = matches.get_one::<String>("file").unwrap();
    
    // Create a Path object from the file path string.
    let path = Path::new(file_path);
    
    // Call the detect_encoding function to check the encoding of the specified file.
    match detect_encoding(path) {
        Ok((encoding, confidence)) => { // If successful, unpack the results.
            // Print the detected encoding to the console.
            println!("Detected encoding: {}", encoding);
            // Print the confidence level as a percentage.
            println!("Confidence: {:.2}%", confidence * 100.0);
        }
        Err(e) => { // If an error occurs, print an error message to the standard error stream.
            eprintln!("Error detecting encoding: {}", e);
        }
    }
}