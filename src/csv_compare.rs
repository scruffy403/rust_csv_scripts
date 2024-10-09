use std::error::Error;
use std::collections::HashSet;
use csv::{ReaderBuilder, WriterBuilder, StringRecord};
use std::io::Write;
use crate::encoding::Encoding;

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

    // Prepare output file name and open writer
    let output_file = format!("{}_modified.csv", file1.trim_end_matches(|c| c == '.' || c == 'c' || c == 's' || c == 'v' || c == 'C' || c == 'S' || c == 'V'));
    
    let mut writer = WriterBuilder::new().from_path(&output_file)?;
    if let Encoding::Utf8Bom = encoding {
        // Write BOM for UTF-8 with BOM
        writer.flush()?;  // Ensure we have access to the writer
        let mut file = writer.into_inner()?;
        file.write_all(b"\xEF\xBB\xBF")?;
        writer = WriterBuilder::new().from_writer(file);
    }

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
