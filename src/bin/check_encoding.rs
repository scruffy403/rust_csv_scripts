// src/bin/check_encoding.rs

//! Binary to check the encoding of a given file.

use std::path::Path;
use std::error::Error;
use my_file_scripts::encoding::{detect_encoding, is_valid_utf8};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter the path to the file you want to check:");
    
    // Read the file path from the user input.
    let mut path_input = String::new();
    std::io::stdin().read_line(&mut path_input)?;
    let path = Path::new(path_input.trim());

    // Detect encoding.
    let (encoding, confidence) = match detect_encoding(path) {
        Ok(result) => result,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    println!("Detected Encoding: {}", encoding);
    println!("Confidence: {:.2}%", confidence * 100.0);
    
    // Additional checks based on the detected encoding.
    if encoding == "ASCII" {
        // Check for the presence of non-ASCII characters in an ASCII-detected file.
        let mut non_ascii_count = 0;
        for byte in std::fs::read(path)? {
            if byte > 127 {
                non_ascii_count += 1;
            }
        }
        if non_ascii_count > 0 {
            println!("Warning: The file contains non-ASCII characters but is detected as ASCII.");
        } else {
            println!("The file contains only ASCII characters.");
        }
    } else if encoding == "UTF-8" {
        // Assume UTF-8 with BOM if BOM was detected.
        if std::fs::read(path)?.starts_with(&[0xEF, 0xBB, 0xBF]) {
            println!("The file uses UTF-8 with BOM.");
        } else {
            println!("The file uses UTF-8 without BOM.");
        }
    }

    // Custom validation for UTF-8 content.
    let buffer = std::fs::read(path)?;
    if is_valid_utf8(&buffer) {
        println!("The file is valid UTF-8.");
    } else {
        println!("The file is not valid UTF-8.");
    }

    Ok(())
}
