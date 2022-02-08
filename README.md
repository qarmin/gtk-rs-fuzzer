# Gtk Rust Fuzzer
Simple GTK Rust Fuzzer which aims to test all available classes and functions in GTK

## What was fuzzed/done
- Every function without arguments was tested
- Tested also parent functions without arguments

## TODO
- Fuzz also functions from traits
- Test also arguments of functions
- Execute functions in random orders
- Improve logging to be able to use copy pasted code from logs

## How to use it
- Install GTK 4(can be probably easily changed to GTK4 if needed)
- Override project, to use l`rustup override set nightly`
- Run `cargo run` inside main folder, this will create `ziemniak.rs` file with 
- Run project with sanitizers(shows crashes and errors a lot of faster and more precise than `cargo run`) - `RUST_BACKTRACE=full RUSTFLAGS=-Zsanitizer=address RUSTDOCFLAGS=-Zsanitizer=address cargo run  -Zbuild-std --target x86_64-unknown-linux-gnu`