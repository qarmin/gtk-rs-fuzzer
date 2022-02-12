# Gtk Rust Fuzzer
Simple GTK Rust Fuzzer which aims to test all available classes and functions in GTK.  
It finds bugs inside GTK functions, GTK exported  and also GTK-rs

## What was fuzzed/done
- Every function without arguments was tested
- Tested also parent functions without arguments
- Fuzz also functions from traits
- Test arguments(enums, objects, primitive objects)
- Ability to copy paste code from output file, to reproduce crash
- Execute functions in random orders
- Fast compilation(Compilation 1000 functions with 20 lines in Rust is faster than compiling 20000 line function, did you know that?)

## TODO
- Ability to use `IsA<Type>` arguments
- 100% crash free setting(needs to find and ignore invalid settings)

## How to use it
- Install GTK 4(can be probably easily changed to GTK4 if needed)
- Override project, to use nightly compiler to be able to use sanitizers `rustup override set nightly`
- Run `cargo run` inside main folder, this will create `ziemniak.rs` file with 
- Run project with sanitizers(shows crashes and errors a lot of faster and more precise than `cargo run`) - `RUST_BACKTRACE=full RUSTFLAGS=-Zsanitizer=address RUSTDOCFLAGS=-Zsanitizer=address cargo run  -Zbuild-std --target x86_64-unknown-linux-gnu`