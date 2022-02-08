#![allow(clippy::collapsible_else_if)]
#![allow(clippy::type_complexity)]
#![allow(clippy::single_char_pattern)]

mod settings;

use crate::settings::{CLASSES_TO_USE, IGNORED_CLASSES, IGNORED_FUNCTIONS};
use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

const PATH_TO_GTK_RS: &str = "/home/rafal/Downloads/gtk4-rs/gtk4/src";
const PATH_TO_GTK_RS_AUTO: &str = "/home/rafal/Downloads/gtk4-rs/gtk4/src/auto";

const PATH_TO_PROJECT_FILE: &str = "/home/rafal/Projekty/Rust/gtk_rs_fuzzer/Project/src/ziemniak.rs";

const NUMBER_OF_REPEATS: u32 = 10; // How many time repeat function executing to be sure that this function cause problems

fn main() {
    let (class_info, class_functions) = collect_things();
    create_project_file(class_info, class_functions)
}
fn create_project_file(_class_info: BTreeMap<String, Vec<String>>, class_functions: BTreeMap<String, BTreeMap<String, Vec<String>>>) {
    let _ = fs::remove_file(PATH_TO_PROJECT_FILE);

    let file = OpenOptions::new().write(true).create(true).open(PATH_TO_PROJECT_FILE).unwrap();
    let mut file = BufWriter::new(file);

    writeln!(
        file,
        "use crate::create_objects::*;
use crate::helpers::*;
use gtk4::prelude::*;
use gtk4::*;

pub fn execute_things(){{

"
    )
    .unwrap();

    // TODO here are full logic
    let mut object_number = 0;

    let mut st_save: Vec<String> = Vec::new();
    for (_index, (name_of_class, function_list)) in class_functions.iter().enumerate() {
        // if name_of_class != "Label" {
        //     continue;
        // }
        // if index > 10 {
        //     continue;
        // }
        st_save.push(format!("\t// {}", name_of_class));
        st_save.push("\t{".to_string());
        for (function, arguments) in function_list {
            // TODO create here an object
            if function == "build" || function == "new" || function.contains("new_") || function == "builder" {
                continue; // TODO check why
            }
            // TODO
            if arguments.is_empty() {
                st_save.push(format!("\t\tfor _i in 0..{}{{", NUMBER_OF_REPEATS));
                st_save.push(format!("\t\t\tprintln!(\"Creating object {} - gget_{} - \");", name_of_class, name_of_class.to_ascii_lowercase()));
                st_save.push(format!("\t\t\tlet _object{} = gget_{}();", object_number, name_of_class.to_ascii_lowercase()));
                st_save.push(format!("\t\t\tprintln!(\"Trying to execute {}.{}()\");", name_of_class, function));
                st_save.push(format!("\t\t\t_object{}.{}();", object_number, function));
                st_save.push(format!("\t\t\tprintln!(\"Executed {}.{}()\");", name_of_class, function));
                st_save.push("\t\t}".to_string());

                object_number += 1;
            }
        }
        st_save.push("\t}".to_string());
        let output = st_save.join("\n");
        writeln!(file, "{}", output).unwrap();

        st_save.clear();
    }

    writeln!(file, "}}").unwrap();
}

fn collect_things() -> (BTreeMap<String, Vec<String>>, BTreeMap<String, BTreeMap<String, Vec<String>>>) {
    // Do not modify result of this variable
    let mut class_info: BTreeMap<String, Vec<String>> = Default::default(); // Class + what extends e.g.   Label -> [Widget, LabelExt]
                                                                            // Can be removed
    let mut class_functions: BTreeMap<String, BTreeMap<String, Vec<String>>> = Default::default(); // Class + functions + arguments e.g. Label -> new -> [&str]

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
            let mut continue_function_declaration = false;
            for line in lines {
                let line = line.trim();
                if continue_function_declaration {
                    if line.ends_with("{") {
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
                                .filter(|e| e != "mutself" && e != "self" && e != "&self")
                                .filter_map(|e| {
                                    let split = e.split(":").map(|e| e.to_string()).collect::<Vec<String>>();
                                    if split.len() == 2 {
                                        return Some(split[1].clone());
                                    }
                                    None
                                })
                                .collect::<Vec<String>>();

                            class_functions.entry(current_class.clone()).or_insert_with(Default::default);
                            class_functions.get_mut(&current_class).unwrap().insert(function_name.clone(), parts.clone());

                            // println!("Arguments for multiline function {}, {:?}", function_name, parts);
                        } else {
                            panic!("HMMMMMMMMMMMMM");
                        }

                        continue_function_declaration = false;
                        previous_arguments.clear();
                        function_name.clear();
                    } else {
                        previous_arguments.push_str(line);
                        if line.contains("{") {
                            panic!("HHHHHHHHHHHHHHHHHHHHh");
                        }
                    }
                } else if line.starts_with("pub fn ") && line.contains("(") && !current_class.is_empty() {
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
                            let parts = text_to_check
                                .split(",")
                                .map(|e| e.replace(" ", ""))
                                .filter(|e| e != "mutself" && e != "self" && e != "&self")
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
                } else if line.contains("Object<ffi::") {
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
            }

            // println!("Found proper {} file", name);
        }
    }

    let mut counter_class;
    let mut counter_methods;
    let mut counter_arguments;
    let all_class_number = class_info.len();

    //// DEBUG
    {
        counter_class = 0;
        counter_methods = 0;
        counter_arguments = 0;
        for function_list in class_functions.values() {
            counter_class += 1;
            for arguments in function_list.values() {
                counter_methods += 1;
                counter_arguments += arguments.len();
            }
        }
        println!("All classes {}", all_class_number);
        println!("Class: {}, Methods: {}, Arguments: {}", counter_class, counter_methods, counter_arguments);
    }
    //// DEBUG END

    // Remove classes which won't be used
    if !CLASSES_TO_USE.is_empty() {
        for used_class in class_info.keys() {
            if !CLASSES_TO_USE.iter().any(|e| e == used_class) {
                class_functions.remove(used_class);
            }
        }
    } else {
        for ignored in IGNORED_CLASSES {
            class_functions.remove(&ignored.to_string());
        }
    }

    // Remove ignored functions
    for ignored in IGNORED_FUNCTIONS {
        for functions in class_functions.values_mut() {
            functions.remove(&ignored.to_string());
        }
    }

    //// DEBUG
    {
        counter_class = 0;
        counter_methods = 0;
        counter_arguments = 0;
        for function_list in class_functions.values() {
            counter_class += 1;
            for arguments in function_list.values() {
                counter_methods += 1;
                counter_arguments += arguments.len();
            }
        }
        println!("After ignoring - Class: {}, Methods: {}, Arguments: {}", counter_class, counter_methods, counter_arguments);
    }
    //// DEBUG END

    (class_info, class_functions)
}
