#![allow(clippy::collapsible_else_if)]
#![allow(clippy::type_complexity)]
#![allow(clippy::single_char_pattern)]

mod settings;

use crate::settings::*;
use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

const PATH_TO_GTK_RS: &str = "/home/rafal/Downloads/gtk4-rs/gtk4/src";
const PATH_TO_GTK_RS_AUTO: &str = "/home/rafal/Downloads/gtk4-rs/gtk4/src/auto";

const PATH_TO_PROJECT_FILE: &str = "/home/rafal/Projekty/Rust/gtk_rs_fuzzer/Project/src/ziemniak.rs";
const PATH_TO_ENUM_FILE: &str = "/home/rafal/Projekty/Rust/gtk_rs_fuzzer/Project/src/enum_things.rs";

fn main() {
    let (class_info, class_functions, traits, enums) = collect_things();
    create_enums_file(&class_info, &class_functions, &traits, &enums);
    create_project_file(class_info, class_functions, traits, enums)
}
fn create_enums_file(
    _class_info: &BTreeMap<String, Vec<String>>,
    _class_functions: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    _traits: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    enums: &BTreeMap<String, Vec<String>>,
) {
    let _ = fs::remove_file(PATH_TO_PROJECT_FILE);

    let file = OpenOptions::new().write(true).truncate(true).create(true).open(PATH_TO_ENUM_FILE).unwrap();
    let mut file = BufWriter::new(file);

    let enum_start = r#####"
use crate::create_objects::*;
use crate::helpers::*;
use gtk4::prelude::*;
use gtk4::*;
use std::fs;
use std::fs::{File, OpenOptions};
use rand::prelude::*;
use std::io::Write;"#####;
    writeln!(file, "{}", enum_start).unwrap();

    // enum_arguments - arguments
    // type - type
    // type_lowercase - type in lowercase
    // type_uppercase - type in uppercase
    // number - number of constants
    let single_enum_template = r#####"
const ENUM_<<type_uppercase>>_TYPE: [(<<type>>, &str); <<number>>] = [
    <<enum_arguments>>
];

pub fn stek_<<type_lowercase>>() -> (<<type>>, String) {
    let to_return = ENUM_<<type_uppercase>>_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}
"#####;

    for (name_of_enum, constant_list) in enums {
        if IGNORED_ENUMS.contains(&name_of_enum.as_str()) {
            continue;
        }

        let mut to_write = single_enum_template
            .replace("<<type>>", name_of_enum)
            .replace("<<number>>", &constant_list.len().to_string())
            .replace("<<type_lowercase>>", &name_of_enum.to_lowercase())
            .replace("<<type_uppercase>>", &name_of_enum.to_uppercase());
        let mut arguments = "".to_string();
        for (index, constant) in constant_list.iter().enumerate() {
            let th = format!("{}::{}", name_of_enum, constant);
            arguments += &format!("({},\"{}\")", th, th);
            if index != constant_list.len() - 1 {
                arguments += ",";
            }
            arguments += "\n";
            if index != constant_list.len() - 1 {
                arguments += "    ";
            }
        }
        to_write = to_write.replace("<<enum_arguments>>", &arguments);
        assert!(!to_write.contains("<<"));
        writeln!(file, "{}", to_write).unwrap();
    }
}

fn create_project_file(
    _class_info: BTreeMap<String, Vec<String>>,
    class_functions: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    _traits: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    enums: BTreeMap<String, Vec<String>>,
) {
    let _ = fs::remove_file(PATH_TO_PROJECT_FILE);

    let file = OpenOptions::new().write(true).truncate(true).create(true).open(PATH_TO_PROJECT_FILE).unwrap();
    let mut file = BufWriter::new(file);

    let start_text = r#####"
use crate::create_objects::*;
use crate::helpers::*;
use crate::enum_things::*;
use gtk4::prelude::*;
use gtk4::*;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use rand::prelude::*;

"#####;

    // Basic function to
    // <<function_arguments>> - list of functions and its names
    // <<number_of_functions>> - number of functions
    let basic_function = r#####"
pub fn run_tests(check_all_things: bool, classes_to_check: Vec<String>, functions_to_check: Vec<String>) {
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open("things.txt").unwrap();

    let all_classes: [(fn(&mut File, bool, &Vec<String>) -> (), &str); <<number_of_functions>>] = [<<function_arguments>>];
    
    // if check_all_things {
    for (function, _name) in all_classes {
        function(&mut file, check_all_things, &functions_to_check);
    }
    // } else {
    //     for (function, name_of_class) in all_classes {
    //         if classes_to_check.iter().any(|e| e == name_of_class) {
    //             function(&mut file, check_all_things, &functions_to_check);
    //         }
    //     }
    // }
}
"#####;

    // <<type>> - type of used item
    // <<type_lowercase>> - type of used item
    // <<logic_to_execute>> - logic how to run this thing
    // <<function_list>> - list of functions
    // <<function_number>> - number of functions
    // <<function_class_name>> - number of functions
    let unit_class = r#####"
pub fn <<function_class_name>>(file: &mut File,check_all_things: bool, functions_to_check: &Vec<String>) {
    let functions: [(for<'a, 'b> fn(&'a mut File, &'b <<type>>) -> &'b <<type>>, &str); <<function_number>>] = [<<function_list>>];


    println!("Creating object <<type>>");
    print_and_save_to_file(file, "let thing = gget_<<type_lowercase>>(); // <<type>>");
    let object = gget_<<type_lowercase>>();
    let mut object_ref = &object;

    if check_all_things {
        println!("C");
        for (function, _name_of_func) in functions {
            object_ref = function(file, object_ref);
        }
    } else {
        if !functions_to_check.is_empty() {
            println!("B");
            for i in functions_to_check {
                for (func, name) in functions {
                    if name == i {
                        object_ref = func(file, object_ref);
                        break;
                    }
                }
            }
        } else {
            println!("A");
            for _i in 0..(functions.len()*5) {
                let fnc = functions.choose(&mut rand::thread_rng()).unwrap().0;
                object_ref = fnc(file, object_ref);
            }
        }
    }

    println!("AA")
}
"#####;

    // <<function_name>> - function name
    // <<type>> - type of used item
    // <<method>> - used method
    let unit_function = r#####"
pub fn <<function_name>><'a,'b>(file: &'a mut File, thing: &'b <<type>>) -> &'b <<type>> {
    print_and_save_to_file(file, "thing.<<method>>();");
    thing.<<method>>();
    thing
}
"#####;

    // <<create_object>> - create_object_function
    let _zero_things = r#####"
        println!("Creating object <<type>>");
        print_and_save_to_file(file, "let object_<<number>> = <<create_object>>(); // <<type>>");
        let object_<<number>> = <<create_object>>(); // <<type>>
"#####;

    writeln!(file, "{}", start_text).unwrap();

    struct ClassUnit {
        a_method: String,
        a_function_name: String,
    }
    let mut cu: BTreeMap<String, Vec<ClassUnit>> = Default::default();

    for (_index, (name_of_class, function_list)) in class_functions.iter().enumerate() {
        // if name_of_class != "AboutDialog" {
        //     continue;
        // }
        if (RANGE_OF_USED_CLASSES).contains(&_index) {
            // println!("{}. {}", _index, name_of_class);
        } else {
            continue;
        }

        for (function, arguments) in function_list {
            // TODO create here an object
            if function == "new" {
                continue;
            }
            if !arguments.is_empty() {
                continue;
            }

            if arguments.is_empty() {
                let function_name = format!("fct_{}_{}", name_of_class.to_lowercase(), function);

                let new_cu: ClassUnit = ClassUnit {
                    a_method: function.clone(),
                    a_function_name: function_name,
                };
                cu.entry(name_of_class.clone()).or_insert_with(Default::default);
                cu.get_mut(name_of_class).unwrap().push(new_cu);
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
                        "bool" | "i32" | "u32" | "u64" | "i64" | "f32" | "f64" | "usize" | "char" | "&str" => false,
                        thing => {
                            if IGNORED_CLASSES.contains(&thing) || IGNORED_ENUMS.contains(&thing) {
                                // println!("NOT {}", thing);
                                true
                            } else {
                                if thing.chars().all(|e| e.is_alphabetic()) {
                                    // println!("Supported {:?}", arg);
                                    false
                                } else {
                                    // println!("NOT {:?}", arg);
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
                            "usize" => "take_usize".to_string(),
                            "char" => "take_char".to_string(),
                            "&str" => "&take_string".to_string(),
                            thing => {
                                if !enums.contains_key(thing) {
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

                    // changed_text = multiple_things_old
                    //     .replace("<<number>>", &object_number.to_string())
                    //     .replace("<<type>>", name_of_class)
                    //     .replace("<<method>>", function)
                    //     .replace("<<create_object>>", &name_of_class.to_ascii_lowercase())
                    //     .replace("<<number_of_repeats>>", &NUMBER_OF_REPEATS.to_string())
                    //     .replace("<<creating_arguments>>", &creating_arguments)
                    //     .replace("<<argument_names>>", &to_print_arguments_variable)
                    //     .replace("<<format_arguments>>", &to_print_arguments)
                    //     .replace("<<argument_names_proper>>", &result_arguments);
                }
            }
        }
    }

    let mut list_of_function_classes = "".to_string();
    for (index, (name_of_class, _functions)) in cu.iter().enumerate() {
        list_of_function_classes += &format!("(cls_{},\"{}\")", name_of_class.to_lowercase(), name_of_class);
        if index != cu.len() - 1 {
            list_of_function_classes += ",";
        }
    }

    let base_functions = basic_function
        .replace("<<function_arguments>>", &list_of_function_classes)
        .replace("<<number_of_functions>>", &cu.len().to_string());
    writeln!(file, "{}", base_functions).unwrap();

    for (class_name, functions) in cu {
        let mut arguments = "".to_string();
        for (index, f) in functions.iter().enumerate() {
            arguments += &format!("({},\"{}\")", f.a_function_name, f.a_function_name);
            if index != functions.len() - 1 {
                arguments += ",";
            }
        }

        let fcn = format!("cls_{}", class_name.to_lowercase());

        let end_class = unit_class
            .replace("<<type>>", &class_name)
            .replace("<<type_lowercase>>", &class_name.to_lowercase())
            .replace("<<logic_to_execute>>", "//TODODODODODODODODODODODODOD")
            .replace("<<function_list>>", &arguments)
            .replace("<<function_class_name>>", &fcn)
            .replace("<<function_number>>", &functions.len().to_string());
        assert!(!end_class.contains("<<"));

        write!(file, "{}", end_class).unwrap();
        for i in functions {
            // <<function_name>> - function name
            // <<type>> - type of used item
            // <<method>> - used method
            let single_function = unit_function
                .replace("<<function_name>>", &i.a_function_name)
                .replace("<<type>>", &class_name)
                .replace("<<method>>", &i.a_method);
            assert!(!single_function.contains("<<"));
            write!(file, "{}", single_function).unwrap();
        }
    }

    let end_text = r####"
    pub fn print_and_save_to_file(file: &mut File, what_to_save: &str) {
    writeln!(file, "{}", what_to_save);
    println!("\t{}", what_to_save);
}"####;
    writeln!(file, "{}", end_text).unwrap();
}

fn collect_things() -> (
    BTreeMap<String, Vec<String>>,
    BTreeMap<String, BTreeMap<String, Vec<String>>>,
    BTreeMap<String, BTreeMap<String, Vec<String>>>,
    BTreeMap<String, Vec<String>>,
) {
    // Do not modify result of this variable
    let mut class_info: BTreeMap<String, Vec<String>> = Default::default(); // Class + what extends e.g.   Label -> [Widget, LabelExt]
                                                                            // Can be removed
    let mut class_functions: BTreeMap<String, BTreeMap<String, Vec<String>>> = Default::default(); // Class + functions + arguments e.g. Label -> new -> [&str]

    let mut traits: BTreeMap<String, BTreeMap<String, Vec<String>>> = Default::default();
    let mut enums: BTreeMap<String, Vec<String>> = Default::default();

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
                                continue; // Things like gio::Pango are not supported
                            }
                            if !(text_to_check.starts_with("&self") || text_to_check.starts_with("&mut self") || text_to_check.starts_with("self")) {
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

    (class_info, class_functions, traits, enums)
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
