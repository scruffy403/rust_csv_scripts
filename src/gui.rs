// src/gui.rs

use eframe::egui;
use rfd::FileDialog;
use rust_csv_scripts::encoding::{detect_encoding, Encoding};
use rust_csv_scripts::compare_csv;
use std::path::Path;

/// Entry point to run the graphical user interface (GUI) for the CSV comparison tool.
pub fn run_gui() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("CSV Compare", options, Box::new(|_cc| Box::new(MyApp::default())))
        .expect("Failed to launch the GUI");
}

/// The main application structure for the GUI.
struct MyApp {
    file1: String,              // File path for the first CSV file
    file2: String,              // File path for the second CSV file
    encoding: String,           // Encoding type for file comparison (e.g., UTF-8, UTF-8 with BOM)
    ignore_columns: String,     // Comma-separated list of columns to ignore during comparison
    encoding_file: String,      // File path for the file to check its encoding
    encoding_result: String,    // Result of encoding check (detected encoding and confidence level)
}

/// Default implementation for `MyApp` to initialize all fields with default values.
impl Default for MyApp {
    fn default() -> Self {
        Self {
            file1: String::new(),
            file2: String::new(),
            encoding: String::from("UTF-8"),
            ignore_columns: String::new(),
            encoding_file: String::new(),
            encoding_result: String::new(),
        }
    }
}

/// Implementation of the `eframe::App` trait to define how the GUI is rendered and updated.
impl eframe::App for MyApp {
    /// The main update loop for the GUI. It defines the layout and behavior of the interface.
    ///
    /// # Parameters
    /// * `ctx` - The UI context used to draw and update elements.
    /// * `_frame` - The frame passed by `eframe` (unused here).
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            // File 1 selection
            ui.label("File 1:");

            // Button to open a file dialog for selecting the first CSV file
            if ui.button("Browse File 1").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.file1 = path.display().to_string(); // Store selected path
                }
            }
            // Text field displaying the file path for the first file
            ui.text_edit_singleline(&mut self.file1);

            // File 2 selection
            ui.label("File 2:");

            // Button to open a file dialog for selecting the second CSV file
            if ui.button("Browse File 2").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.file2 = path.display().to_string(); // Store selected path
                }
            }
            // Text field displaying the file path for the second file
            ui.text_edit_singleline(&mut self.file2);

            // Encoding selection
            ui.label("Encoding:");
            // Text field for specifying the encoding type (e.g., UTF-8, UTF-8-BOM)
            ui.text_edit_singleline(&mut self.encoding);

            // Ignore columns input
            ui.label("Ignore Columns (comma-separated):");
            // Text field for specifying columns to ignore during CSV comparison
            ui.text_edit_singleline(&mut self.ignore_columns);

            // CSV comparison button
            if ui.button("Compare CSVs").clicked() {
                // Parse the ignore columns string into a vector of strings
                let ignore_columns: Vec<&str> = self.ignore_columns.split(',')
                    .map(|s| s.trim())
                    .collect();

                // Match the encoding string to the appropriate enum variant
                let encoding_enum = match self.encoding.as_str() {
                    "UTF-8" => Encoding::Utf8,
                    "UTF-8 with BOM" => Encoding::Utf8Bom,
                    _ => {
                        eprintln!("Unsupported encoding: {}", self.encoding);
                        return;
                    }
                };

                // Call the CSV comparison function with the selected files and parameters
                if let Err(e) = compare_csv(&self.file1, &self.file2, encoding_enum, &ignore_columns) {
                    eprintln!("Error comparing files: {}", e); // Log error if comparison fails
                }
            }

            // Separator for a new section to check file encoding
            ui.separator();
            ui.label("Check File Encoding:");

            // Button to open a file dialog for selecting a file to check its encoding
            if ui.button("Browse File to Check Encoding").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.encoding_file = path.display().to_string(); // Store selected file path
                }
            }
            // Text field displaying the file path for encoding check
            ui.text_edit_singleline(&mut self.encoding_file);

            // Button to detect the encoding of the selected file
            if ui.button("Check Encoding").clicked() {
                let path = Path::new(&self.encoding_file);
                match detect_encoding(path) {
                    // If encoding detection succeeds, store the result in `encoding_result`
                    Ok((encoding, confidence)) => {
                        self.encoding_result = format!("Encoding: {}, Confidence: {:.2}%", encoding, confidence * 100.0);
                    }
                    // If encoding detection fails, store the error message
                    Err(e) => {
                        self.encoding_result = format!("Error detecting encoding: {}", e);
                    }
                }
            }

            // Display the encoding detection result if available
            if !self.encoding_result.is_empty() {
                ui.label(&self.encoding_result); // Show the result in the UI
            }
        });
    }
}