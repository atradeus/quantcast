### Description
The quantcast command line utility for finding he most active cookie in a CSV file

### Install Rust
https://www.rust-lang.org/tools/install

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build, Test and Run
```
git clone https://github.com/atradeus/quantcast.git
cd quantcast 
cargo test && \
cargo build && \
target/debug/quantcast -d 2018-12-09 -f ./data.csv
```

### Help
Use -h or --help flags for help menu
```
target/debug/quantcast --help

Quantcast App

Usage: quantcast --date <date> --file <file>

Options:
  -d, --date <date>  Date in format YYYY-MM-DD
  -f, --file <file>  Input file
  -h, --help         Print help
  -V, --version      Print version

```