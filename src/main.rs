#![allow(clippy::collapsible_else_if)]
#![allow(clippy::type_complexity)]
#![allow(clippy::single_char_pattern)]

mod settings;

use crate::settings::{CLASSES_TO_USE, ENUMS_ETC, FUNCTIONS_TO_USE, IGNORED_CLASSES, IGNORED_FUNCTIONS, NUMBER_OF_REPEATS, USE_PARENT_ITEMS, USE_TRAIT_ITEMS};
use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

const PATH_TO_GTK_RS: &str = "/home/rafal/Downloads/gtk4-rs/gtk4/src";
const PATH_TO_GTK_RS_AUTO: &str = "/home/rafal/Downloads/gtk4-rs/gtk4/src/auto";

const PATH_TO_PROJECT_FILE: &str = "/home/rafal/Projekty/Rust/gtk_rs_fuzzer/Project/src/ziemniak.rs";

fn main() {
    let (class_info, class_functions) = collect_things();
    create_project_file(class_info, class_functions)
}
fn create_project_file(_class_info: BTreeMap<String, Vec<String>>, class_functions: BTreeMap<String, BTreeMap<String, Vec<String>>>) {
    let _ = fs::remove_file(PATH_TO_PROJECT_FILE);

    let file = OpenOptions::new().write(true).truncate(true).create(true).open(PATH_TO_PROJECT_FILE).unwrap();
    let mut file = BufWriter::new(file);

    let start_text = r#####"
use crate::create_objects::*;
use crate::helpers::*;
use gtk4::prelude::*;
use gtk4::*;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn execute_things(){
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open("things.txt").unwrap();
"#####;

    // Basic function to
    // <<function_arguments>> - list of functions and its names
    // <<number_of_functions>> - number of functions
    let basic_function = r#####"
pub fn run_tests(check_all_things: bool, classes_to_check: Vec<String>, functions_to_check: Vec<String>) {
    let all_things: [(fn(&Vec<String>) -> (), &str); <<number_of_functions>>] = [<<function_arguments>>];

    if check_all_things {
        for (function, _name) in all_things {
            function(check_all_things, &functions_to_check);
        }
    } else {
        for (function, name_of_class) in all_things {
            if classes_to_check.iter().any(|e| e == name_of_class) {
                function(check_all_things, &functions_to_check);
            }
        }
    }
}
"#####;

    // <<type>> - type of used item
    // <<logic_to_execute>> - logic how to run this thing
    let unit_class = r#####"
pub fn things_on(check_all_things: bool, functions_to_check: &Vec<String>) {
    let functions: [(fn(&<<type>>) -> &<<type>>, &str); 1] = [(fct, "fct")];

    <<logic_to_execute>>

    println!("AA")
}
"#####;

    // <<type>> - type of used item
    // <<executed_functions>> - logic how to run this thing
    let unit_function = r#####"
pub fn fct(thing: &<<type>>) -> &<<type>> {
    <<executed_functions>>
    thing
}
"#####;

    // <<number>> - just any number
    // <<type>> - type of used item
    // <<method>> - used method
    // <<create_object>> - create_object_function
    let zero_things = r#####"
        println!("Creating object <<type>>");
        print_and_save_to_file(&mut file, "let object_<<number>> = <<create_object>>(); // <<type>>");
        let object_<<number>> = <<create_object>>(); // <<type>>
        print_and_save_to_file(&mut file, "object_<<number>>.<<method>>();");
        object_<<number>>.<<method>>();
"#####;

    // <<number>> - just any number
    // <<type>> - type of used item
    // <<method>> - used method
    // <<create_object>> - create_object_function
    // <<number_of_repeats>> - number of repeats
    let zero_things_old = r#####"
        for _i in 0..<<number_of_repeats>>{
            println!("Creating object <<type>>");
            print_and_save_to_file(&mut file, "let object_<<number>> = gget_<<create_object>>(); // <<type>>");
            let object_<<number>> = gget_<<create_object>>(); // <<type>>
            print_and_save_to_file(&mut file, "object_<<number>>.<<method>>();");
            object_<<number>>.<<method>>();
        }
"#####;

    // <<number>> - just any number
    // <<type>> - type of used item
    // <<method>> - used method
    // <<create_object>> - create_object_function
    // <<number_of_repeats>> - number of repeats
    // <<creating_arguments>> - creating arguments
    // <<argument_names>> - argument names like argument1, argument2, argument3 etc.
    // <<format_arguments>> - {},{},{} - exactly same number as arguments
    // <<argument_names_proper>> - argument names like argument1, argument2, argument3 etc.,but with proper formatting (e.g. with added Some() wrap)
    let multiple_things_old = r#####"
        for _i in 0..<<number_of_repeats>>{
            println!("Creating object <<type>>");
            print_and_save_to_file(&mut file, "let object_<<number>> = gget_<<create_object>>(); // <<type>>");
            let object_<<number>> = gget_<<create_object>>(); // <<type>>
            <<creating_arguments>>
			print_and_save_to_file(&mut file, &format!("object_<<number>>.<<method>>(<<format_arguments>>);",<<argument_names>>));
            object_<<number>>.<<method>>(<<argument_names_proper>>);
        }
"#####;

    writeln!(file, "{}", start_text).unwrap();

    let mut object_number = 0;

    let mut changed_text = "".to_string();
    for (_index, (name_of_class, function_list)) in class_functions.iter().enumerate() {
        // if name_of_class != "AboutDialog" {
        //     continue;
        // }
        if (0..35).contains(&_index) {
            println!("{}. {}", _index, name_of_class);
        } else {
            continue;
        }

        writeln!(file, "\t//{}", name_of_class).unwrap();
        writeln!(file, "\t{{").unwrap();
        for (function, arguments) in function_list {
            // TODO create here an object
            if function == "new" {
                continue;
            }
            // TODO Better object numbering(currently things are )
            // if !arguments.is_empty() {
            //     continue;
            // }

            if arguments.is_empty() {
                changed_text = zero_things_old
                    .replace("<<number>>", &object_number.to_string())
                    .replace("<<type>>", name_of_class)
                    .replace("<<method>>", function)
                    .replace("<<create_object>>", &name_of_class.to_ascii_lowercase())
                    .replace("<<number_of_repeats>>", &NUMBER_OF_REPEATS.to_string());
            } else {
                // println!("{:?}", arguments);
                let mut found_bad_thing: bool = false;
                // TODO support for all arguments
                for arg in arguments {
                    let mut arg = arg.clone();
                    if arg.starts_with("Option<") {
                        arg = arg.strip_prefix("Option<").unwrap().to_string();
                        arg = arg.strip_suffix(">").unwrap().to_string();
                    }
                    found_bad_thing = match arg.as_str() {
                        "bool" | "i32" | "u32" | "u64" | "i64" | "f32" | "f64" | "&str" => false,
                        thing => {
                            if IGNORED_CLASSES.contains(&thing) {
                                println!("NOT {}", thing);
                                true
                            } else {
                                if thing.chars().all(|e| e.is_alphabetic()) {
                                    println!("Supported {:?}", arg);
                                    false
                                } else {
                                    println!("NOT {:?}", arg);
                                    true
                                }
                            }
                        }
                    };
                    if found_bad_thing {
                        break;
                    }
                }
                if !found_bad_thing {
                    let mut result_arguments = "".to_string();
                    let mut creating_arguments = "".to_string();

                    let mut to_print_arguments = "".to_string();
                    let mut to_print_arguments_variable = "".to_string();

                    for arg_index in 0..arguments.len() {
                        let mut is_option_type = false;
                        let mut reference = "";
                        let mut arg = arguments[arg_index].clone();
                        if arg.starts_with("Option<") {
                            is_option_type = true;
                            arg = arg.strip_prefix("Option<").unwrap().to_string();
                            arg = arg.strip_suffix(">").unwrap().to_string();
                        }
                        if arg.starts_with("&") && arg != "&str" {
                            reference = "&";
                            arg = arg[1..].to_string();
                        }

                        let mut stek = "";
                        let help_function_name = match arg.as_str() {
                            "bool" => "take_bool".to_string(),
                            "i32" => "take_i32".to_string(),
                            "u32" => "take_u32".to_string(),
                            "u64" => "take_u64".to_string(),
                            "i64" => "take_i64".to_string(),
                            "f32" => "take_f32".to_string(),
                            "f64" => "take_f64".to_string(),
                            "&str" => "&take_string".to_string(),
                            thing => {
                                if !ENUMS_ETC.contains(&thing) {
                                    format!("gget_{}", thing.to_lowercase())
                                } else {
                                    stek = ".0";
                                    format!("stek_{}", thing.to_lowercase())
                                }
                            } // _ => panic!("Not supported {}", arg),
                        };
                        creating_arguments += &format!("let argument_{} = {}(){}; // {}", arg_index, help_function_name, stek, arg);
                        if arg_index != arguments.len() - 1 {
                            creating_arguments += "\n\t\t\t";
                        }

                        let comma_after = if arg_index == arguments.len() - 1 { "".to_string() } else { ",".to_string() };

                        if is_option_type {
                            result_arguments += &format!("Some({}argument_{}){}", reference, arg_index, comma_after);
                        } else {
                            result_arguments += &format!("{}argument_{}{}", reference, arg_index, comma_after);
                        }

                        let default_formatter = match is_option_type {
                            true => "Some({})",
                            false => "{}",
                        };
                        if arg == "&str" {
                            if is_option_type {
                                to_print_arguments += &format!("Some({}\\\"{{}}\\\"){}", reference, comma_after);
                            } else {
                                to_print_arguments += &format!("{}\\\"{{}}\\\"{}", reference, comma_after);
                            }
                        } else {
                            to_print_arguments += &format!("{}{}", default_formatter, comma_after);
                        }

                        to_print_arguments_variable += &format!("argument_{}{}", arg_index, comma_after);
                    }

                    changed_text = multiple_things_old
                        .replace("<<number>>", &object_number.to_string())
                        .replace("<<type>>", name_of_class)
                        .replace("<<method>>", function)
                        .replace("<<create_object>>", &name_of_class.to_ascii_lowercase())
                        .replace("<<number_of_repeats>>", &NUMBER_OF_REPEATS.to_string())
                        .replace("<<creating_arguments>>", &creating_arguments)
                        .replace("<<argument_names>>", &to_print_arguments_variable)
                        .replace("<<format_arguments>>", &to_print_arguments)
                        .replace("<<argument_names_proper>>", &result_arguments);
                }
            }

            object_number += 1;
            assert!(!changed_text.contains("<<"));
            writeln!(file, "{}", changed_text).unwrap();
        }
        writeln!(file, "\t}}").unwrap();
    }

    writeln!(file, "}}").unwrap();
    let end_text = r####"
    pub fn print_and_save_to_file(file: &mut File, what_to_save: &str) {
    writeln!(file, "{}", what_to_save);
    println!("\t{}", what_to_save);
}"####;
    writeln!(file, "{}", end_text).unwrap();
}

fn collect_things() -> (BTreeMap<String, Vec<String>>, BTreeMap<String, BTreeMap<String, Vec<String>>>) {
    // Do not modify result of this variable
    let mut class_info: BTreeMap<String, Vec<String>> = Default::default(); // Class + what extends e.g.   Label -> [Widget, LabelExt]
                                                                            // Can be removed
    let mut class_functions: BTreeMap<String, BTreeMap<String, Vec<String>>> = Default::default(); // Class + functions + arguments e.g. Label -> new -> [&str]

    let mut traits: BTreeMap<String, BTreeMap<String, Vec<String>>> = Default::default();

    for path_dir in [PATH_TO_GTK_RS, PATH_TO_GTK_RS_AUTO] {
        let dir = fs::read_dir(path_dir).unwrap_or_else(|_| panic!("Cannot open dir {}", path_dir));
        for entry in dir {
            let entry_data = match entry {
                Ok(t) => t,
                Err(_e) => {
                    println!("Cannot read entries of {}", path_dir);
                    continue;
                }
            };
            let name = entry_data.file_name().to_string_lossy().to_string();
            if !name.ends_with(".rs") {
                continue;
            }

            let mut full_path = PathBuf::new();
            full_path = full_path.join(&path_dir);
            full_path = full_path.join(&name);

            let all_text = fs::read_to_string(full_path).unwrap();
            let lines = all_text.split("\n");

            let mut function_name: String = "".to_string();
            let mut previous_arguments: String = "".to_string();
            let mut current_class: String = "".to_string();
            let mut current_trait: String = "".to_string();
            let mut continue_function_declaration = false;
            for line in lines {
                let old_line = line;
                let line = old_line.trim();
                // GENERAL - continue when found ending
                if continue_function_declaration {
                    if (!current_class.is_empty() && line.ends_with("{")) || (!current_trait.is_empty() && line.ends_with(";")) {
                        if line.contains(")") {
                            previous_arguments.push_str(&line[..line.find(")").unwrap()]);
                            if previous_arguments.contains("::") {
                                continue_function_declaration = false;
                                previous_arguments.clear();
                                function_name.clear();
                                continue; // Things like gio::Pango are not supported
                            }
                            let parts = previous_arguments
                                .split(",")
                                .map(|e| e.replace(" ", ""))
                                // .filter(|e| e != "mutself" && e != "self" && e != "&self")
                                .filter_map(|e| {
                                    let split = e.split(":").map(|e| e.to_string()).collect::<Vec<String>>();
                                    if split.len() == 2 {
                                        return Some(split[1].clone());
                                    }
                                    None
                                })
                                .collect::<Vec<String>>();

                            if !current_class.is_empty() {
                                class_functions.entry(current_class.clone()).or_insert_with(Default::default);
                                class_functions.get_mut(&current_class).unwrap().insert(function_name.clone(), parts.clone());
                            } else if !current_trait.is_empty() {
                                traits.entry(current_trait.clone()).or_insert_with(Default::default);
                                traits.get_mut(&current_trait).unwrap().insert(function_name.clone(), parts.clone());
                            } else {
                                panic!("")
                            }

                            // println!("Arguments for multiline function {}, {:?}", function_name, parts);
                            continue_function_declaration = false;
                            previous_arguments.clear();
                            function_name.clear();
                        }
                    } else {
                        previous_arguments.push_str(line);
                    }
                }
                // Finds objects function
                else if line.starts_with("pub fn ") && line.contains("(") && !current_class.is_empty() {
                    // println!("{}", line);
                    let text_to_check = "pub fn ";
                    let function_name_local = &line[text_to_check.len()..line.find("(").unwrap()];
                    if function_name_local.contains("connect_") || function_name_local.contains("<") {
                        continue; // Connect function are not supported
                    }

                    if line.ends_with("{") {
                        // Means that this is one line function
                        if line.contains("(") && line.contains(")") {
                            let text_to_check = &line[line.find("(").unwrap() + 1..line.find(")").unwrap()];
                            if text_to_check.contains("::") {
                                continue_function_declaration = false;
                                previous_arguments.clear();
                                function_name.clear();
                                continue; // Things like gio::Pango are not supported
                            }
                            if !(text_to_check.starts_with("&self") || text_to_check.starts_with("&mut self") || text_to_check.starts_with("self")) {
                                if !text_to_check.is_empty() {
                                    // println!("Ignoring {}", text_to_check);
                                }
                                continue;
                            }
                            let parts = text_to_check
                                .split(",")
                                .map(|e| e.replace(" ", ""))
                                // .filter(|e| e != "mutself" && e != "self" && e != "&self")
                                .filter_map(|e| {
                                    let split = e.split(":").map(|e| e.to_string()).collect::<Vec<String>>();
                                    if split.len() == 2 {
                                        return Some(split[1].clone());
                                    }
                                    None
                                })
                                .collect::<Vec<String>>();

                            class_functions.entry(current_class.clone()).or_insert_with(Default::default);
                            class_functions.get_mut(&current_class).unwrap().insert(function_name_local.to_string(), parts.clone());

                            // println!("Arguments for function {}, {:?}", function_name_local, parts);
                        } else {
                            panic!("HMMMMMMMMMMMMM");
                        }
                    } else {
                        if let Some(line_index) = line.find("(") {
                            previous_arguments.push_str(&line[line_index..]);
                        } else {
                            panic!("Big HMMMM");
                        }
                        function_name = function_name_local.to_string();
                        continue_function_declaration = true;
                    }

                    // println!("found function \"{}\" for \"{}\"", function_name_local, current_class);
                }
                // Finds
                else if line.contains("Object<ffi::") {
                    current_trait.clear();
                    let t_help = "pub struct ";
                    if let Some(found_item) = line.find(t_help) {
                        let new_temp_line = &line[found_item + t_help.len()..];
                        if let Some(found_space) = new_temp_line.find("(") {
                            current_class = new_temp_line[..found_space].to_string();
                            if !class_info.contains_key(&current_class) {
                                class_info.insert(current_class.clone(), Vec::new());
                            }

                            let e_text = "@extends ";
                            let i_text = "@implements ";
                            let e_index = line.find(e_text);
                            let i_index = line.find(i_text);
                            if let Some(e_index) = e_index {
                                if let Some(i_index) = i_index {
                                    let text_to_check = &line[e_index + e_text.len()..i_index];
                                    let parts = text_to_check
                                        .split(",")
                                        .map(|e| e.replace(" ", "").replace(",", ""))
                                        .filter(|e| !e.is_empty() && e.chars().next().unwrap().is_uppercase())
                                        .collect::<Vec<String>>();
                                    class_info.get_mut(&current_class).unwrap().extend(parts);

                                    let text_to_check = &line[i_index + i_text.len()..line.len() - 1];
                                    let parts = text_to_check
                                        .split(",")
                                        .map(|e| e.replace(" ", "").replace(",", ""))
                                        .filter(|e| !e.is_empty() && e.chars().next().unwrap().is_uppercase())
                                        .collect::<Vec<String>>();
                                    class_info.get_mut(&current_class).unwrap().extend(parts);
                                } else {
                                    let text_to_check = &line[e_index + e_text.len()..line.len() - 1];
                                    let parts = text_to_check
                                        .split(",")
                                        .map(|e| e.replace(" ", "").replace(",", ""))
                                        .filter(|e| !e.is_empty() && e.chars().next().unwrap().is_uppercase())
                                        .collect::<Vec<String>>();
                                    class_info.get_mut(&current_class).unwrap().extend(parts);
                                }
                            } else {
                                if let Some(i_index) = i_index {
                                    let text_to_check = &line[i_index + i_text.len()..line.len() - 1];
                                    let parts = text_to_check
                                        .split(",")
                                        .map(|e| e.replace(" ", "").replace(",", ""))
                                        .filter(|e| !e.is_empty() && e.chars().next().unwrap().is_uppercase())
                                        .collect::<Vec<String>>();
                                    class_info.get_mut(&current_class).unwrap().extend(parts);
                                }
                            }
                        }
                    }
                }
                // Finds Ext
                else if line.starts_with("pub trait ") && line.contains("Ext") && !line.contains("Manual") {
                    current_class.clear();
                    current_trait.clear();

                    let t_help = "pub trait ";
                    let temp_line = &line[t_help.len()..];
                    if let Some(s_index) = temp_line.find(" ") {
                        let ext_name = &temp_line[..s_index];
                        let end_name = if ext_name.ends_with(':') {
                            &ext_name[..ext_name.len() - 4]
                        } else {
                            &ext_name[..ext_name.len() - 3]
                        };
                        // println!("ZZZ {}, {}, {}, {}", line, temp_line, ext_name, end_name);
                        // println!("{}     ,,,,      {}", end_name, ext_name);
                        current_trait = end_name.to_string();
                        traits.insert(end_name.to_string(), Default::default());
                    }
                } else if line.starts_with("fn ") && line.contains("(") && !current_trait.is_empty() {
                    // println!("{}", line);
                    let text_to_check = "fn ";
                    let function_name_local = &line[text_to_check.len()..line.find("(").unwrap()];
                    if function_name_local.contains("connect_") || function_name_local.contains("<") {
                        continue; // Connect function are not supported
                    }

                    if line.ends_with(";") {
                        // Means that this is one line function
                        if line.contains("(") && line.contains(")") {
                            let text_to_check = &line[line.find("(").unwrap() + 1..line.find(")").unwrap()];
                            if text_to_check.contains("::") {
                                continue_function_declaration = false;
                                previous_arguments.clear();
                                function_name.clear();
                                continue; // Things like gio::Pango are not supported
                            }
                            if !(text_to_check.starts_with("&self") || text_to_check.starts_with("&mut self") || text_to_check.starts_with("self")) {
                                if !text_to_check.is_empty() {
                                    // println!("Ignoring {}", text_to_check);
                                }
                                continue;
                            }
                            let parts = text_to_check
                                .split(",")
                                .map(|e| e.replace(" ", ""))
                                // .filter(|e| e != "mutself" && e != "self" && e != "&self")
                                .filter_map(|e| {
                                    let split = e.split(":").map(|e| e.to_string()).collect::<Vec<String>>();
                                    if split.len() == 2 {
                                        return Some(split[1].clone());
                                    }
                                    None
                                })
                                .collect::<Vec<String>>();

                            traits.entry(current_trait.clone()).or_insert_with(Default::default);
                            traits.get_mut(&current_trait).unwrap().insert(function_name_local.to_string(), parts.clone());

                            // println!("Arguments for function {}, {:?}", function_name_local, parts);
                        } else {
                            panic!("HMMMMMMMMMMMMM");
                        }
                    } else {
                        if let Some(line_index) = line.find("(") {
                            previous_arguments.push_str(&line[line_index..]);
                        } else {
                            panic!("Big HMMMM");
                        }
                        function_name = function_name_local.to_string();
                        continue_function_declaration = true;
                    }
                } else if old_line.starts_with("impl ") && line.contains("Builder") {
                    // println!("{}", old_line);
                    current_class = "".to_string();
                    current_trait = "".to_string();
                }
            }

            // println!("Found proper {} file", name);
        }
    }

    count_objects(&class_functions, &traits, "At start            ");

    // Extend classes with parent functions
    if USE_PARENT_ITEMS {
        let base_functions = class_functions.clone(); // Needed to have same set of functions across all iterations

        for (name_of_class, parent_classes) in &class_info {
            if class_functions.contains_key(name_of_class) {
                for parent_class in parent_classes {
                    if class_functions.contains_key(parent_class) {
                        // println!("I'm in {}, {}", name_of_class, parent_class);
                        class_functions
                            .get_mut(name_of_class)
                            .unwrap()
                            .append(&mut base_functions.get(parent_class).unwrap().clone());
                    } else {
                        // println!("MISSING parent class: {}", parent_class); // TODO why is this missing?
                    }
                }
            } else {
                // println!("MISSING normal class: {}", name_of_class); // TODO why is this missing?
            }
        }
    }

    // Extend classes with traits functions
    if USE_TRAIT_ITEMS {
        for (name_of_class, used_traits) in &class_info {
            if class_functions.contains_key(name_of_class) {
                for used_trait in used_traits {
                    if traits.contains_key(used_trait) {
                        // println!("I'm in {}, {}", name_of_class, parent_class);
                        class_functions.get_mut(name_of_class).unwrap().append(&mut traits.get(used_trait).unwrap().clone());
                    } else {
                        // println!("MISSING parent class: {}", parent_class); // TODO why is this missing?
                    }
                }
            } else {
                // println!("MISSING normal class: {}", name_of_class); // TODO why is this missing?
            }
        }
    }

    count_objects(&class_functions, &traits, "After adding parents");

    // Remove classes which won't be used
    if !CLASSES_TO_USE.is_empty() {
        let keys = class_functions.clone().into_keys();
        for used_class in keys {
            if !CLASSES_TO_USE.iter().any(|e| *e == used_class) {
                class_functions.remove(&used_class);
            }
        }
    } else {
        for ignored in IGNORED_CLASSES {
            class_functions.remove(&ignored.to_string());
        }
    }

    // Remove functions which won't be used
    if !FUNCTIONS_TO_USE.is_empty() {
        let keys = class_functions.clone();
        for (name_of_class, function_list) in keys {
            for (function, _) in function_list {
                if !FUNCTIONS_TO_USE.iter().any(|e| *e == function) {
                    class_functions.get_mut(&name_of_class).unwrap().remove(&function);
                }
            }
        }
    } else {
        for ignored in IGNORED_FUNCTIONS {
            for functions in class_functions.values_mut() {
                functions.remove(&ignored.to_string());
            }
        }
    }

    // Print all classes + functions + arguments
    // {
    //     for (name_of_class, function_list) in &class_functions {
    //         for (name_of_function, arguments) in function_list {
    //             let mut what = format!("{}.{}(", name_of_class, name_of_function);
    //             for index in 0..arguments.len() {
    //                 what += "\"";
    //                 what += &arguments[index];
    //                 what += "\"";
    //                 if index != arguments.len() - 1 {
    //                     what += ",";
    //                 }
    //             }
    //             what += ")";
    //
    //             println!("{}", what);
    //         }
    //     }
    // }
    // {
    //     for (name_of_class, function_list) in &traits {
    //         for (name_of_function, arguments) in function_list {
    //             let mut what = format!("{}.{}(", name_of_class, name_of_function);
    //             for index in 0..arguments.len() {
    //                 what += "\"";
    //                 what += &arguments[index];
    //                 what += "\"";
    //                 if index != arguments.len() - 1 {
    //                     what += ",";
    //                 }
    //             }
    //             what += ")";
    //
    //             println!("{}", what);
    //         }
    //     }
    // }
    // {
    //     for (name_of_class, function_list) in &traits {
    //         for (name_of_function, _arguments) in function_list {
    //             println!("{}.{} --- Trait", name_of_class, name_of_function)
    //         }
    //     }
    // }
    // {
    //     for (name_of_class, function_list) in &class_functions {
    //         for (name_of_function, _arguments) in function_list {
    //             println!("{}.{}", name_of_class, name_of_function)
    //         }
    //     }
    // }
    // {
    //     for (name_of_class, similar_clases) in &class_info {
    //         println!("{}.{:?}", name_of_class, similar_clases)
    //     }
    // }

    count_objects(&class_functions, &traits, "End results         ");

    (class_info, class_functions)
}

fn count_objects(class_functions: &BTreeMap<String, BTreeMap<String, Vec<String>>>, traits: &BTreeMap<String, BTreeMap<String, Vec<String>>>, what: &str) {
    let mut counter_class = 0;
    let mut counter_methods = 0;
    let mut counter_arguments = 0;
    let traits_number = traits.len();
    for function_list in class_functions.values() {
        counter_class += 1;
        for arguments in function_list.values() {
            counter_methods += 1;
            counter_arguments += arguments.len();
        }
    }
    println!(
        "{} - Class: {}, Methods: {}, Arguments: {}, Traits: {}",
        what, counter_class, counter_methods, counter_arguments, traits_number
    );
}
