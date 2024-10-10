use clap::{Command, Arg}; // Use Command instead of App

mod gui; // Import the GUI module

fn main() {
    let matches = Command::new("CSV Scripts") // Use Command::new instead of App::new
        .version("0.1.0")
        .about("Utility for comparing CSV files and checking encoding")
        .arg(
            Arg::new("use_gui")
                .short('g')
                .long("use_gui")
                .help("Launch the GUI for CSV comparison"), // Use .help() instead of .about()
        )
        .get_matches();

    // Check if the "use_gui" argument was provided
    if matches.get_one::<String>("use_gui").is_some() {
        gui::run_gui(); // Call the GUI function
    } else {
        println!("Please run the desired command-line program with the appropriate options.");
    }
}