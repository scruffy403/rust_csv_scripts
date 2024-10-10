
# File Scripts

This repository contains two Rust scripts for working with CSV files and detecting file encoding.

## Scripts

1. **compare_csv.rs**
   - Compares two CSV files, checks for unique rows, and writes them to a new CSV.
   - Allows ignoring specified columns during comparison.
   - Supports output encoding in UTF-8 or UTF-8 with BOM.

2. **check_encoding.rs**
   - Detects the encoding of a file and checks if it's valid UTF-8.
   - Provides detailed information about any non-UTF-8 characters.

    ## Installation

1. Install [Rust](https://www.rust-lang.org/).
2. Clone this repository:

```bash
  git clone https://github.com/scruffy403/rust_csv_scripts.git
```
Alternatively you can copy and paste all of the relevant code into your project. Ensure you copy the contents of `Cargo.toml` and copy the structure and code of the `src` folder and its sub folder `bin`. Then configure you `.ignore` file to ignore the `target`  and `debug` folders.

### Run compare_csv script

```bash
cargo run --bin compare_csv -- <path_to_file1> <path_to_file2> --ignore <ignored_column_names> --encoding <encoding_type>
```

### Run check_encoding script

```bash
cargo run --bin check_encoding -- <path_to_file>
```

### Run the GUI

```bash
cargo run --bin rust_csv_scripts -- --use_gui gui
```