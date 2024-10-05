// src/csv_compare.rs

use std::error::Error;
use std::collections::HashSet;
use csv::{ReaderBuilder, WriterBuilder, StringRecord};

/// Enum to represent file encoding options.
#[derive(Debug)]
pub enum Encoding {
    Utf8,
    Utf8Bom,
}

/// Compares the headers of two CSV files and prints any mismatches.
///
/// # Arguments
///
/// * `file1` - The path to the first CSV file.
/// * `file2` - The path to the second CSV file.
///
/// # Errors
///
/// Returns an error if the files cannot be read or the headers are missing.
pub fn compare_file_headers(file1: &str, file2: &str) -> Result<(), Box<dyn Error>> {
    let mut reader1 = ReaderBuilder::new().from_path(file1)?;
    let mut reader2 = ReaderBuilder::new().from_path(file2)?;

    let headers1 = reader1.headers()?;
    let headers2 = reader2.headers()?;

    println!("Comparing headers:");

    // Check if the number of columns matches.
    if headers1.len() == headers2.len() {
        println!("Both files have the same number of columns: {}", headers1.len());
    } else {
        println!(
            "Files have different numbers of columns: {} vs {}",
            headers1.len(),
            headers2.len()
        );
    }

    // Compare column names and print only mismatches.
    let mut mismatch_found = false;
    for (i, header1) in headers1.iter().enumerate() {
        if let Some(header2) = headers2.get(i) {
            if header1 != header2 {
                println!("Column {} mismatch: '{}' does not match '{}'", i + 1, header1, header2);
                mismatch_found = true;
            }
        }
    }

    if !mismatch_found {
        println!("All column names match.");
    }

    Ok(())
}

/// Compares rows between two CSV files, filters out ignored columns, and writes
/// unique rows from `file1` to a new CSV file with the same headers.
///
/// # Arguments
///
/// * `file1` - The path to the first CSV file.
/// * `file2` - The path to the second CSV file.
/// * `ignore_columns` - A set of column names to ignore during comparison.
/// * `encoding` - The encoding to use for the output CSV file.
///
/// # Errors
///
/// Returns an error if there are issues reading/writing the files or processing the data.
pub fn compare_and_write_unique_rows(file1: &str, file2: &str, ignore_columns: &HashSet<String>, encoding: Encoding) -> Result<(), Box<dyn Error>> {
    // Read rows from file2 into a HashSet for quick lookups.
    let mut reader2 = ReaderBuilder::new().from_path(file2)?;
    let mut rows_in_file2: HashSet<Vec<String>> = HashSet::new();

    // Store headers for filtering ignored columns later.
    let headers2 = reader2.headers()?.clone();

    for result in reader2.records() {
        let record = result?;
        let filtered_record: Vec<String> = filter_columns(&record, &headers2, ignore_columns);
        rows_in_file2.insert(filtered_record);
    }

    // Create a writer for the new CSV file.
    let output_file = format!("{}_modified.csv", file1.trim_end_matches(".csv"));
    let mut writer = match encoding {
        Encoding::Utf8 => WriterBuilder::new().from_path(&output_file)?,
        Encoding::Utf8Bom => {
            let mut writer = WriterBuilder::new().from_path(&output_file)?;
            writer.flush()?;
            let mut file = writer.into_inner()?;
            use std::io::Write; // Ensure `Write` is in scope
            file.write_all(b"\xEF\xBB\xBF")?; // Write BOM
            WriterBuilder::new().from_writer(file)
        }
    };

    // Write the headers to the new CSV.
    let mut reader1 = ReaderBuilder::new().from_path(file1)?;
    writer.write_record(reader1.headers()?)?;

    // Read rows from file1 and check against file2.
    let headers1 = reader1.headers()?.clone();

    for result in reader1.records() {
        let record = result?;
        let filtered_record: Vec<String> = filter_columns(&record, &headers1, ignore_columns);
        if !rows_in_file2.contains(&filtered_record) {
            // Write the full row (with all columns) to the new CSV.
            writer.write_record(&record)?;
        }
    }

    writer.flush()?; // Ensure all data is written to the file.
    println!("Unique rows from '{}' written to '{}'", file1, output_file);

    Ok(())
}

/// Filters out columns from the record based on the columns to ignore.
///
/// # Arguments
///
/// * `record` - The CSV record to filter.
/// * `headers` - The headers associated with the record.
/// * `ignore_columns` - The set of column names to ignore.
///
/// # Returns
///
/// A `Vec<String>` containing only the columns that are not ignored.
fn filter_columns(record: &StringRecord, headers: &StringRecord, ignore_columns: &HashSet<String>) -> Vec<String> {
    let mut filtered_record = Vec::new();

    for (i, header) in headers.iter().enumerate() {
        if !ignore_columns.contains(header) {
            if let Some(field) = record.get(i) {
                filtered_record.push(field.to_string());
            }
        }
    }

    filtered_record
}
