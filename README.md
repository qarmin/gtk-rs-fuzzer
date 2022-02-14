# Gtk Rust Fuzzer
Simple GTK Rust Fuzzer which aims to test all available classes and functions in GTK.  
It finds bugs inside GTK functions, GTK exported  and also GTK-rs.

Code from execution is saved to `things.txt` file to be able to easily reproduce crashes.

For now support fuzzing GTK 4.4+(prebuilt binary supports only functions available in 4.4).

`settings.txt` allows to customize size of searched things

## State at 14.02.2022
Work on this fuzzer is done and I expect to do only simple changes, fixes and updates.  
Fuzzer tests 12225 functions(7253 empty and 4971 with multiple arguments).

## How it works?
In opposite to my other fuzzer - [Qarminer](https://github.com/qarmin/Qarminer) (used to fuzz Godot Engine) which used dynamic language - GDScript, I couldn't find and execute in runtime random functions and arguments.  
In this project I need to create files with source code of each function and its arguments, because Rust needs to check if all arguments are with valid type and lifetime.  
So at first, main project needs to be run which parse gtk4 files and produce information about classes, its functions and arguments, traits and enums.  
Next, generated are functions to get random enum or objects which implements specific trait.  
At the end generated is code responsible for random executing of every not ignored(or allowed) function and class.

After generating source code to `Project` folder, compilation of this code is required.  
There are 3 manually generated files `main.rs`(contains basic code to run fuzzer), `helpers.rs`(implementation of basic types like `u64`), `create_objects.rs`(functions to return object of any type).

## Maybe TODO
- 100% crash free default settings(this require to find each possible crash, ignore specific functions and classes and also report it to `gtk-rs`/`gtk`)
- Test also static methods
- Support for builders
- Random object generation

## How to use it
- Install GTK 4.4+ on OS
- Go to main folder of this repository
- Optional: Override project, to use nightly compiler to be able to use sanitizers `rustup override set nightly`
- Run `cargo run` inside main folder, this will create `ziemniak.rs`, `enum_things.rs` and `implementations.rs` files in `Project` folder
- Go to `Project` project folder
- Create/modify `settings.txt` file if needed
- Optional: To use sanitizers run this command - `RUST_BACKTRACE=full RUSTFLAGS=-Zsanitizer=address RUSTDOCFLAGS=-Zsanitizer=address cargo run  -Zbuild-std --target x86_64-unknown-linux-gnu`
- To run normal build, run this command `cargo run`
- Get crash and look at the end of file `things.txt` to find exact code to reproduce crash.