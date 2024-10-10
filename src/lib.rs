// src/lib.rs

//! # File Scripts Library
//! 
//! This library provides functionalities for checking file encodings and comparing CSV files.

pub mod encoding; // Keep the encoding module if you have it
pub mod csv_compare; // Keep your CSV comparison logic

use std::collections::HashSet;
use crate::csv_compare::compare_and_write_unique_rows; // Use `crate::` to refer to the current crate
use crate::encoding::Encoding; // Use `crate::` to refer to the current crate

pub fn compare_csv(file1: &str, file2: &str, encoding: Encoding, ignore_columns: &[&str]) -> Result<(), String> {
    let ignore_set: HashSet<String> = ignore_columns.iter()
        .map(|s| s.to_string())
        .collect();

    compare_and_write_unique_rows(file1, file2, &ignore_set, encoding)
        .map_err(|e| e.to_string())
}