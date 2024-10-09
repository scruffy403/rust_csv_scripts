use clap::{Arg, Command};
use std::path::Path;
use rust_csv_scripts::encoding::detect_encoding;


fn main() {
    let matches = Command::new("check_encoding")
        .about("Check the encoding of a file")
        .arg(Arg::new("file")
            .help("The file to check")
            .required(true)
            .value_parser(clap::value_parser!(String)))
        .get_matches();

    // Use `get_one` instead of `value_of`
    let file_path = matches.get_one::<String>("file").unwrap();
    
    let path = Path::new(file_path);
    
    match detect_encoding(path) {
        Ok((encoding, confidence)) => {
            println!("Detected encoding: {}", encoding);
            println!("Confidence: {:.2}%", confidence * 100.0);
        }
        Err(e) => eprintln!("Error detecting encoding: {}", e),
    }
}
