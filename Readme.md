


### Install Rust
https://www.rust-lang.org/tools/install

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build, Test and Run
```
git clone https://github.com/atradeus/quantcast.git
cd quantcast
cargo build
cargo test
target/debug/quantcast -d 2018-12-09 -f ./data.csv
```