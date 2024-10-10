use eframe::egui;
use rust_csv_scripts::encoding::Encoding; 
use rust_csv_scripts::compare_csv;

pub fn run_gui() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("CSV Compare", options, Box::new(|_cc| Box::new(MyApp::default())))
        .expect("Failed to launch the GUI");
}

struct MyApp {
    file1: String,
    file2: String,
    encoding: String,
    ignore_columns: String, // Field for ignoring columns
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file1: String::new(),
            file2: String::new(),
            encoding: String::from("UTF-8"),
            ignore_columns: String::new(), // Initialize the new field
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { // Prefix with underscore
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("File 1:");
            ui.text_edit_singleline(&mut self.file1);

            ui.label("File 2:");
            ui.text_edit_singleline(&mut self.file2);

            ui.label("Encoding:");
            ui.text_edit_singleline(&mut self.encoding);

            ui.label("Ignore Columns (comma-separated):");
            ui.text_edit_singleline(&mut self.ignore_columns); // Input for ignored columns

            if ui.button("Compare CSVs").clicked() {
                let ignore_columns: Vec<&str> = self.ignore_columns.split(',')
                    .map(|s| s.trim())
                    .collect();

                // Determine encoding
                let encoding = match self.encoding.as_str() {
                    "UTF-8" => Encoding::Utf8,
                    "UTF-8-BOM" => Encoding::Utf8Bom,
                    _ => {
                        eprintln!("Invalid encoding specified. Defaulting to UTF-8.");
                        Encoding::Utf8
                    }
                };

                // Call compare_csv function with provided parameters
                if let Err(e) = compare_csv(&self.file1, &self.file2, encoding, &ignore_columns) {
                    eprintln!("Error comparing files: {}", e);
                }
            }
        });
    }
}