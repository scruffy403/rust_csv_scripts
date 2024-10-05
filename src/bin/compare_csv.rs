// src/bin/compare_csv.rs

//! Binary to compare two CSV files and write unique rows from the first file to a new CSV.

use std::collections::HashSet;
use std::error::Error;
use std::time::Instant;
use my_file_scripts::csv_compare::{compare_file_headers, compare_and_write_unique_rows, Encoding};

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now(); // Start the timer for the whole script.

    let file1 = prompt_for_file_path("Enter the path to the first CSV file:")?;
    let file2 = prompt_for_file_path("Enter the path to the second CSV file:")?;

    // Check headers without ignoring any columns.
    compare_file_headers(&file1, &file2)?;

    // Ask if columns need to be ignored for subsequent comparisons.
    let ignore_columns = prompt_for_ignore_columns()?;

    // Prompt for output encoding.
    let encoding = prompt_for_encoding()?;

    // Time the file comparison and row writing process.
    let comparison_start_time = Instant::now();

    // Compare rows and write unique rows from file1 to a new CSV.
    compare_and_write_unique_rows(&file1, &file2, &ignore_columns, encoding)?;

    let comparison_duration = comparison_start_time.elapsed();
    println!(
        "File comparison completed in: {} seconds ({} ms)",
        comparison_duration.as_secs(),
        comparison_duration.as_millis()
    );

    let total_duration = start_time.elapsed();
    println!(
        "Script executed in: {} seconds ({} ms)",
        total_duration.as_secs(),
        total_duration.as_millis()
    );

    Ok(())
}

/// Prompts the user for a file path.
///
/// # Arguments
///
/// * `prompt` - The message to display when asking for the file path.
///
/// # Returns
///
/// The trimmed file path as a `String` or an error if input fails.
fn prompt_for_file_path(prompt: &str) -> Result<String, Box<dyn Error>> {
    println!("{}", prompt);
    let mut path = String::new();
    std::io::stdin().read_line(&mut path)?;
    let path = path.trim().to_string(); // Remove any trailing newline or spaces.
    Ok(path)
}

/// Prompts the user to specify columns to ignore during the file comparison.
///
/// # Returns
///
/// A set of column names to be ignored, or an empty set if no columns are ignored.
fn prompt_for_ignore_columns() -> Result<HashSet<String>, Box<dyn Error>> {
    println!("Would you like to ignore any columns in future comparisons? (y/n)");
    let mut response = String::new();
    std::io::stdin().read_line(&mut response)?;
    let response = response.trim().to_lowercase();

    let mut columns_to_ignore = HashSet::new();
    
    if response == "y" {
        println!("How many columns would you like to ignore?");
        let mut num_columns = String::new();
        std::io::stdin().read_line(&mut num_columns)?;
        let num_columns: usize = num_columns.trim().parse()?;
        
        for _ in 0..num_columns {
            println!("Enter the name of the column to ignore:");
            let mut column_name = String::new();
            std::io::stdin().read_line(&mut column_name)?;
            columns_to_ignore.insert(column_name.trim().to_string());
        }
    }

    Ok(columns_to_ignore)
}

/// Prompts the user to select the output file encoding (UTF-8 or UTF-8-BOM).
///
/// # Returns
///
/// The chosen encoding as the `Encoding` enum.
fn prompt_for_encoding() -> Result<Encoding, Box<dyn Error>> {
    println!("Select output file encoding:");
    println!("1. UTF-8");
    println!("2. UTF-8 with BOM");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    match choice {
        "1" => Ok(Encoding::Utf8),
        "2" => Ok(Encoding::Utf8Bom),
        _ => {
            println!("Invalid choice. Defaulting to UTF-8.");
            Ok(Encoding::Utf8)
        }
    }
}
