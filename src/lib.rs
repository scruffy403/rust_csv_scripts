// src/lib.rs

//! # File Scripts Library
//! 
//! This library provides functionalities for checking file encodings and comparing CSV files.
//! 
//! ## Modules
//! 
//! - `encoding`: Contains functions to detect file encoding and validate UTF-8.
//! - `csv_compare`: Contains functions for comparing CSV files and writing unique rows.

pub mod encoding; // Module for file encoding functions.
pub mod csv_compare; // Module for CSV comparison logic.

use std::collections::HashSet; // Import HashSet for managing ignored columns.
use crate::csv_compare::compare_and_write_unique_rows; // Import the function to compare and write unique CSV rows.
use crate::encoding::Encoding; // Import the Encoding enum to specify output encoding.

/// Compares two CSV files and writes unique rows from the first file to a new CSV file.
///
/// # Arguments
///
/// * `file1` - The path to the first CSV file.
/// * `file2` - The path to the second CSV file.
/// * `encoding` - The encoding format to use for the output CSV file.
/// * `ignore_columns` - A slice of column names to ignore during the comparison.
///
/// # Returns
///
/// A `Result` that is `Ok(())` if the comparison and writing are successful, or
/// an `Err(String)` containing an error message if there are issues during the process.
pub fn compare_csv(file1: &str, file2: &str, encoding: Encoding, ignore_columns: &[&str]) -> Result<(), String> {
    // Create a HashSet of ignored column names for efficient lookups.
    let ignore_set: HashSet<String> = ignore_columns.iter()
        .map(|s| s.to_string())
        .collect();

    // Call the CSV comparison function and handle any potential errors.
    compare_and_write_unique_rows(file1, file2, &ignore_set, encoding)
        .map_err(|e| e.to_string()) // Convert the error to a String if it occurs.
}