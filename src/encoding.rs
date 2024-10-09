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
pub fn is_valid_utf8(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok()
}

/// Detects the encoding of a file at the given path.
///
/// # Arguments
///
/// * `path` - The path to the file.
///
/// # Returns
///
/// A tuple containing the detected encoding as a `String` and the confidence level as a `f64`.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub fn detect_encoding(path: &Path) -> Result<(String, f64), io::Error> {
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found."));
    }

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    
    // Read the first few bytes to check for BOM
    let mut bom_check = [0; 3];
    let bom_bytes_read = reader.read(&mut bom_check)?;

    // Check if BOM is present (UTF-8 BOM is "\xEF\xBB\xBF")
    let is_utf8_bom = bom_bytes_read == 3 && &bom_check == b"\xEF\xBB\xBF";

    // Read the rest of the file into a buffer
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // Detect encoding using `chardet`
    let detection = detect(&buffer);
    let encoding = detection.0;  // Detected encoding as a string
    let confidence = detection.1; // Confidence level of detection as a float

    // If BOM is detected, specify that in the result
    let encoding = if is_utf8_bom {
        "UTF-8 with BOM".to_string()
    } else {
        encoding.to_string()
    };

    Ok((encoding, confidence.into()))
}
