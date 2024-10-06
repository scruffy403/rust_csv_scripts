
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
## Usage/Examples

### Run compare_csv script

```bash
cargo run --bin compare_csv
```

### Run check_encoding script

```bash
cargo run --bin check_encoding
```

## Installation

1. Install [Rust](https://www.rust-lang.org/).
2. Clone this repository:

```bash
  git clone https://github.com/scruffy403/rust_csv_scripts.git
```
    