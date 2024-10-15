use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};
use chardet::detect;

/// Enum to represent file encoding options.
#[derive(Debug)]
pub enum Encoding {
    Utf8,
    Utf8Bom,
}

/// Checks if the provided byte slice is valid UTF-8.
///
/// # Arguments
///
/// * `bytes` - A byte slice containing the file content.
///
/// # Returns
///
/// A boolean value indicating whether the byte slice is valid UTF-8.
///
/// This function attempts to convert the byte slice into a valid UTF-8 string.
/// If the conversion succeeds, it returns `true`, indicating that the byte slice
/// is valid UTF-8. If the conversion fails, it returns `false`.
pub fn is_valid_utf8(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok()
}

/// Detects the encoding of a file at the given path.
///
/// This function first checks if the file exists, then reads the file into a buffer.
/// It attempts to detect the file encoding using the `chardet` library and also checks
/// for a UTF-8 BOM. The function returns both the detected encoding and a confidence score.
///
/// # Arguments
///
/// * `path` - The path to the file whose encoding is to be detected.
///
/// # Returns
///
/// A tuple containing the detected encoding as a `String` and the confidence level as a `f64`.
///
/// * `"UTF-8"` or `"UTF-8 with BOM"` if the file is detected as UTF-8 with or without BOM.
/// * Other encodings (e.g., `"ISO-8859-1"`, `"Windows-1252"`) based on the content.
///
/// # Errors
///
/// Returns an `io::Error` if the file cannot be opened or read.
///
/// This function can fail for several reasons, such as:
/// * The file does not exist.
/// * The file cannot be read due to permission issues or I/O errors.
/// * The file is empty or corrupt.
pub fn detect_encoding(path: &Path) -> Result<(String, f64), io::Error> {
    // Check if the file exists before attempting to open it.
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found."));
    }

    // Open the file for reading.
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    
    // Buffer to store the first few bytes for BOM detection.
    let mut bom_check = [0; 3];
    let bom_bytes_read = reader.read(&mut bom_check)?;

    // Check if the first three bytes match the UTF-8 BOM signature (0xEF, 0xBB, 0xBF).
    let is_utf8_bom = bom_bytes_read == 3 && &bom_check == b"\xEF\xBB\xBF";

    // Read the rest of the file into a buffer for encoding detection.
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // Detect encoding using the `chardet` library.
    let detection = detect(&buffer);
    let encoding = detection.0;  // Detected encoding as a string.
    let confidence = detection.1; // Confidence level of detection as a float.

    // If the BOM is detected, explicitly specify "UTF-8 with BOM".
    let encoding = if is_utf8_bom {
        "UTF-8 with BOM".to_string()
    } else {
        encoding.to_string()
    };

    // Return the detected encoding and the confidence level.
    Ok((encoding, confidence.into()))
}