use clap::{Command, Arg}; // Use Command instead of App for argument parsing

mod gui; // Import the GUI module for graphical user interface functionality

/// The main entry point of the program. 
/// 
/// This function initializes the command-line interface (CLI) for the CSV comparison utility,
/// and it optionally launches the graphical user interface (GUI) based on user input.
fn main() {
    // Define command-line arguments using the `Command` struct from the `clap` crate.
    let matches = Command::new("CSV Scripts") // Use Command::new instead of App::new
        .version("0.1.0") // Set version of the program
        .about("Utility for comparing CSV files and checking encoding") // Brief description of the program
        .arg(
            Arg::new("use_gui") // Define an argument for launching the GUI
                .short('g') // Short form of the argument (-g)
                .long("use_gui") // Long form of the argument (--use_gui)
                .help("Launch the GUI for CSV comparison"), // Help message for this argument
        )
        .get_matches(); // Parse the command-line arguments

    // Check if the "use_gui" argument was provided by the user.
    if matches.get_one::<String>("use_gui").is_some() {
        gui::run_gui(); // If the "use_gui" argument is present, launch the GUI.
    } else {
        // If no arguments are provided, display a message with instructions.
        println!("Please run the desired command-line program with the appropriate options.");
    }
}