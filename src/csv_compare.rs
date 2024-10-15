use std::error::Error;
use std::collections::HashSet;
use csv::{ReaderBuilder, WriterBuilder, StringRecord};
use std::io::Write;
use crate::encoding::Encoding;

/// Compares rows between two CSV files, filters out ignored columns, and writes
/// unique rows from `file1` (i.e., rows not found in `file2`) to a new CSV file 
/// with the same headers as `file1`.
///
/// # Arguments
///
/// * `file1` - The path to the first CSV file.
/// * `file2` - The path to the second CSV file.
/// * `ignore_columns` - A set of column names to ignore during comparison.
/// * `encoding` - The encoding to use for the output CSV file (e.g., UTF-8 or UTF-8 with BOM).
///
/// # Returns
///
/// Returns `Ok(())` if the operation succeeds, or an error if there are issues 
/// reading/writing the files or processing the data.
pub fn compare_and_write_unique_rows(file1: &str, file2: &str, ignore_columns: &HashSet<String>, encoding: Encoding) -> Result<(), Box<dyn Error>> {
    // Read rows from file2 into a HashSet for fast comparison (removes duplicates).
    let mut reader2 = ReaderBuilder::new().from_path(file2)?;
    let mut rows_in_file2: HashSet<Vec<String>> = HashSet::new();

    // Retrieve the headers from file2 for later use in filtering.
    let headers2 = reader2.headers()?.clone();

    // Iterate through the rows of file2 and filter the columns to be ignored.
    for result in reader2.records() {
        let record = result?;
        let filtered_record: Vec<String> = filter_columns(&record, &headers2, ignore_columns);
        rows_in_file2.insert(filtered_record); // Store filtered rows from file2
    }

    // Prepare the name for the output file by modifying the original file1 name.
    let output_file = format!("{}_modified.csv", file1.trim_end_matches(|c| c == '.' || c == 'c' || c == 's' || c == 'v' || c == 'C' || c == 'S' || c == 'V'));
    
    // Initialize a CSV writer for the output file.
    let mut writer = WriterBuilder::new().from_path(&output_file)?;

    // If the selected encoding is UTF-8 with BOM, write the BOM to the file.
    if let Encoding::Utf8Bom = encoding {
        writer.flush()?;  // Flush the buffer before writing the BOM.
        let mut file = writer.into_inner()?; // Access the underlying file.
        file.write_all(b"\xEF\xBB\xBF")?; // Write BOM bytes.
        writer = WriterBuilder::new().from_writer(file); // Reinitialize writer after BOM.
    }

    // Write the original headers (from file1) to the new output file.
    let mut reader1 = ReaderBuilder::new().from_path(file1)?;
    writer.write_record(reader1.headers()?)?;

    // Retrieve headers from file1 to be used in filtering.
    let headers1 = reader1.headers()?.clone();

    // Iterate through each row in file1.
    for result in reader1.records() {
        let record = result?;
        // Filter columns based on the ignore list.
        let filtered_record: Vec<String> = filter_columns(&record, &headers1, ignore_columns);

        // If the filtered row from file1 doesn't exist in file2, write it to the new file.
        if !rows_in_file2.contains(&filtered_record) {
            writer.write_record(&record)?; // Write the full row (not just filtered) to the output.
        }
    }

    writer.flush()?; // Ensure all data is written to the output file.
    println!("Unique rows from '{}' written to '{}'", file1, output_file);

    Ok(())
}

/// Filters out ignored columns from a CSV record based on the provided set of column names.
///
/// This function takes a CSV record and its associated headers and filters out the
/// columns that are present in the `ignore_columns` set. It returns a new `Vec<String>`
/// containing only the columns that were not ignored.
///
/// # Arguments
///
/// * `record` - The CSV record to filter.
/// * `headers` - The headers associated with the CSV file.
/// * `ignore_columns` - A set of column names to ignore during filtering.
///
/// # Returns
///
/// A `Vec<String>` containing only the columns that were not ignored.
fn filter_columns(record: &StringRecord, headers: &StringRecord, ignore_columns: &HashSet<String>) -> Vec<String> {
    let mut filtered_record = Vec::new();

    // Iterate through each column in the record by index.
    for (i, header) in headers.iter().enumerate() {
        // Only keep columns whose headers are not in the ignore list.
        if !ignore_columns.contains(header) {
            // Add the field from the record to the filtered list if it exists.
            if let Some(field) = record.get(i) {
                filtered_record.push(field.to_string());
            }
        }
    }

    filtered_record
}