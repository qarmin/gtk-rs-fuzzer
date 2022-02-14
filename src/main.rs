#![allow(clippy::collapsible_else_if)]
#![allow(clippy::type_complexity)]
#![allow(clippy::single_char_pattern)]

mod creating_enum;
mod creating_functions;
mod creating_implementation;
mod settings;

use crate::creating_enum::*;
use crate::creating_functions::*;
use crate::creating_implementation::*;
use crate::settings::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

fn main() {
    let (class_info, class_functions, traits, enums, children_of_class) = collect_things();
    create_enums_file(&class_info, &class_functions, &traits, &enums, &children_of_class);
    create_implementation_file(&class_info, &class_functions, &traits, &enums, &children_of_class);
    create_project_file(class_info, class_functions, traits, enums, children_of_class)
}

fn collect_things() -> (
    BTreeMap<String, Vec<String>>,
    BTreeMap<String, BTreeMap<String, Vec<String>>>,
    BTreeMap<String, BTreeMap<String, Vec<String>>>,
    BTreeMap<String, Vec<String>>,
    BTreeMap<String, Vec<String>>,
) {
    // Do not modify result of this variable
    let mut class_info: BTreeMap<String, Vec<String>> = Default::default(); // Class + what extends e.g.   Label -> [Widget, LabelExt]
                                                                            // Can be removed
    let mut class_functions: BTreeMap<String, BTreeMap<String, Vec<String>>> = Default::default(); // Class + functions + arguments e.g. Label -> new -> [&str]

    let mut traits: BTreeMap<String, BTreeMap<String, Vec<String>>> = Default::default();
    let mut enums: BTreeMap<String, Vec<String>> = Default::default();

    let mut children_of_class: BTreeMap<String, Vec<String>> = Default::default();

    let mut number_of_ignored_functions: u32 = 0;
    let mut number_of_ignored_gio_etc_functions: u32 = 0;

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
            // if name != "label.rs" {
            //     continue;
            // }

            let mut full_path = PathBuf::new();
            full_path = full_path.join(&path_dir);
            full_path = full_path.join(&name);

            let all_text = fs::read_to_string(full_path).unwrap();
            let lines = all_text.split("\n");

            let mut function_name: String = "".to_string();
            let mut previous_arguments: String = "".to_string();
            let mut current_class: String = "".to_string();
            let mut current_trait: String = "".to_string();
            let mut current_enum: String = "".to_string();
            let mut continue_function_declaration = false;
            let mut counter = 0; // If equal to 0, can be cleared, used because Object<ffi is inside different library
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
                                number_of_ignored_gio_etc_functions += 1;
                                continue; // Things like gio::Pango are not supported
                            }
                            if !(text_to_check.starts_with("&self") || text_to_check.starts_with("&mut self") || text_to_check.starts_with("self")) {
                                number_of_ignored_functions += 1;
                                continue;
                            }
                            let parts = text_to_check
                                .split(",")
                                .map(|e| e.replace(" ", ""))
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
                    current_class.clear();
                    current_trait.clear();
                    current_enum.clear();
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
                            counter = 2;
                        }
                    }
                }
                // Finds Ext
                else if line.starts_with("pub trait ") && line.contains("Ext") && !line.contains("Manual") {
                    current_class.clear();
                    current_trait.clear();
                    current_enum.clear();

                    let t_help = "pub trait ";
                    let temp_line = &line[t_help.len()..];
                    if let Some(s_index) = temp_line.find(" ") {
                        let ext_name = &temp_line[..s_index];
                        let end_name = if ext_name.ends_with(':') {
                            &ext_name[..ext_name.len() - 4]
                        } else {
                            &ext_name[..ext_name.len() - 3]
                        };
                        current_trait = end_name.to_string();
                        traits.insert(end_name.to_string(), Default::default());
                        counter = 1;
                    }
                } else if line.starts_with("fn ") && line.contains("(") && !current_trait.is_empty() {
                    // println!("{}", line);
                    let text_to_check = "fn ";
                    let function_name_local = &line[text_to_check.len()..line.find("(").unwrap()];
                    if function_name_local.contains("connect_") || function_name_local.contains("<") {
                        number_of_ignored_functions += 1;
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
                                number_of_ignored_gio_etc_functions += 1;
                                continue; // Things like gio::Pango are not supported
                            }
                            if !(text_to_check.starts_with("&self") || text_to_check.starts_with("&mut self") || text_to_check.starts_with("self")) {
                                number_of_ignored_functions += 1;
                                continue;
                            }
                            let parts = text_to_check
                                .split(",")
                                .map(|e| e.replace(" ", ""))
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
                } else if old_line.starts_with("pub enum ") {
                    current_class.clear();
                    current_trait.clear();
                    current_enum.clear();

                    let t_help = "pub enum ";
                    let end_help = " {";
                    let end_name = &line[t_help.len()..line.len() - end_help.len()];

                    if !end_name.contains("<") {
                        current_enum = end_name.to_string();
                        enums.insert(end_name.to_string(), Default::default());
                        counter = 1;
                    }
                } else if old_line.starts_with("}") {
                    counter -= 1;
                    if counter == 0 {
                        current_class = "".to_string();
                        current_trait = "".to_string();
                        current_enum = "".to_string();
                    }
                } else if !current_enum.is_empty() {
                    // println!("Name - {}, line {}", name, line);
                    if !line.starts_with("#") {
                        let mut thing = line.to_string();
                        thing.pop(); // Remove last comma
                        if !thing.contains("(") {
                            enums.get_mut(&current_enum).unwrap().push(thing.to_string());
                            counter = 1;
                        }
                    }
                }
            }

            // println!("Found proper {} file", name);
        }
    }

    for (class, parents) in &class_info {
        if !IGNORED_CLASSES.contains(&class.as_str()) {
            for parent in parents {
                children_of_class.entry(parent.clone()).or_insert_with(Default::default);
                children_of_class.get_mut(parent).unwrap().push(class.to_string());
            }
        }
    }
    // TODO adds self class to implemented by things
    // for (class, children) in &mut children_of_class {
    //     if !IGNORED_CLASSES.contains(&class.as_str()) {
    //         children.push(class.clone());
    //     }
    // }
    // for (parent, classes_begin) in children_of_class {
    //     for begin in classes_begin {
    //         println!("{}.{}", parent, begin);
    //     }
    // }

    println!(
        "Ignored functions(connect, static methods etc.) - {}, Ignored functions(gdk, gio etc. arguments) - {}",
        number_of_ignored_functions, number_of_ignored_gio_etc_functions
    );

    count_objects(&class_functions, &traits, &enums, "At start            ");

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

    count_objects(&class_functions, &traits, &enums, "After adding parents");

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

    count_objects(&class_functions, &traits, &enums, "End results         ");

    (class_info, class_functions, traits, enums, children_of_class)
}

fn count_objects(
    class_functions: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    traits: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    enums: &BTreeMap<String, Vec<String>>,
    what: &str,
) {
    let mut counter_class = 0;
    let mut counter_methods = 0;
    let mut counter_arguments = 0;
    let traits_number = traits.len();
    let all_traits: usize = traits.iter().map(|(_e, b)| b.len()).sum();
    let enums_number = enums.len();
    let all_enums: usize = enums.iter().map(|(_e, b)| b.len()).sum();
    for function_list in class_functions.values() {
        counter_class += 1;
        for arguments in function_list.values() {
            counter_methods += 1;
            counter_arguments += arguments.len();
        }
    }
    println!(
        "{} - Class: {}, Methods: {}, Arguments: {}, Traits: {}({}), Enums: {}({})",
        what, counter_class, counter_methods, counter_arguments, traits_number, all_traits, enums_number, all_enums
    );
}
