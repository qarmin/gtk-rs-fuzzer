#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

mod create_objects;
mod enum_things;
mod helpers;
mod implementations;
mod ziemniak;

use crate::create_objects::*;
use crate::ziemniak::{run_tests, SettingsTaker};
use gtk4::prelude::*;
use gtk4::*;
use std::fs;
use std::fs::File;

fn main() {
    let application = gtk4::Application::builder().build();
    application.connect_activate(move |application| {
        let window = gtk4::ApplicationWindow::new(application);
        window.set_title(Some("Fuzzer gtk-rs"));

        window.show();

        let sf = read_from_file();

        if TEST == 0 {
            crashes();
        } else {
            // execute_things();
            run_tests(sf);
        }
    });

    application.run();
}

const TEST: u64 = 1;
fn crashes() {
    println!("TESTSTTSTSTSTSTSTST");
}

fn read_from_file() -> SettingsTaker {
    let string: String = match fs::read_to_string("settings.txt") {
        Ok(t) => t,
        Err(_) => {
            println!("Missing settings.txt file");
            return SettingsTaker {
                ignored_functions: vec![],
                allowed_functions: vec![],
                ignored_classes: vec![],
                allowed_classes: vec![],
                repeating_number: 3,
                all_repeating_number: 1,
                number_of_max_executed_function: -1,
            };
        }
    };

    let mut st: SettingsTaker = SettingsTaker {
        ignored_functions: vec![],
        allowed_functions: vec![],
        ignored_classes: vec![],
        allowed_classes: vec![],
        repeating_number: 3,
        all_repeating_number: 1,
        number_of_max_executed_function: -1,
    };

    enum MODES {
        None,
        IgnoredFunctions,
        AllowedFunctions,
        IgnoredClasses,
        AllowedClasses,
        Repeating,
        AllRepeating,
        MaxExecutedFunction,
    }

    let mut current_mode: MODES = MODES::None;
    for line in string.split('\n').map(|e| e.to_string()).collect::<Vec<String>>() {
        let new_line = line.trim().to_string();
        if new_line.starts_with("//") {
            continue; // Comment
        }

        if new_line == "ignored_functions:" {
            current_mode = MODES::IgnoredFunctions;
        } else if new_line == "allowed_functions:" {
            current_mode = MODES::AllowedFunctions;
        } else if new_line == "ignored_classes:" {
            current_mode = MODES::IgnoredClasses;
        } else if new_line == "allowed_classes:" {
            current_mode = MODES::AllowedClasses;
        } else if new_line == "repeating_number:" {
            current_mode = MODES::Repeating;
        } else if new_line == "all_repeating_number:" {
            current_mode = MODES::AllRepeating;
        } else if new_line == "number_of_max_executed_function:" {
            current_mode = MODES::MaxExecutedFunction;
        } else {
            if !new_line.is_empty() {
                match current_mode {
                    MODES::IgnoredFunctions => st.ignored_functions.push(new_line),
                    MODES::AllowedFunctions => st.allowed_functions.push(new_line),
                    MODES::IgnoredClasses => st.ignored_classes.push(new_line),
                    MODES::AllowedClasses => st.allowed_classes.push(new_line),
                    MODES::Repeating => {
                        if let Ok(number) = new_line.parse() {
                            st.repeating_number = number;
                        }
                    }
                    MODES::AllRepeating => {
                        if let Ok(number) = new_line.parse() {
                            st.all_repeating_number = number;
                        }
                    }
                    MODES::MaxExecutedFunction => {
                        if let Ok(number) = new_line.parse() {
                            st.number_of_max_executed_function = number;
                        }
                    }
                    MODES::None => println!("SETTING: Missing mode for {}", new_line),
                }
            }
        }
    }
    // Print found data
    {
        println!("Start settings loading");

        if !st.ignored_classes.is_empty() {
            println!("Ignored classes:");
            for i in &st.ignored_classes {
                println!("{}", i);
            }
        }
        if !st.allowed_classes.is_empty() {
            println!("Allowed classes:");
            for i in &st.allowed_classes {
                println!("{}", i);
            }
        }
        if !st.allowed_functions.is_empty() {
            println!("Allowed functions:");
            for i in &st.allowed_functions {
                println!("{}", i);
            }
        }
        if !st.ignored_functions.is_empty() {
            println!("Ignored functions:");
            for i in &st.ignored_functions {
                println!("{}", i);
            }
        }
        println!("Repeating - {}", st.repeating_number);
        println!("All Repeating - {}", st.all_repeating_number);
        println!("Max Executed Functions - {}", st.number_of_max_executed_function);
        println!("End settings loading");
    }

    st
}
