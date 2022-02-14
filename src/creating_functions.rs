use crate::settings::*;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub fn create_project_file(
    _class_info: BTreeMap<String, Vec<String>>,
    class_functions: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    _traits: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    enums: BTreeMap<String, Vec<String>>,
    children_of_class: BTreeMap<String, Vec<String>>,
) {
    let _ = fs::remove_file(PATH_TO_PROJECT_FILE);

    let file = OpenOptions::new().write(true).truncate(true).create(true).open(PATH_TO_PROJECT_FILE).unwrap();
    let mut file = BufWriter::new(file);

    let start_text = r#####"
use crate::create_objects::*;
use crate::helpers::*;
use crate::enum_things::*;
use crate::implementations::*;
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
pub struct SettingsTaker {
    pub(crate) ignored_functions: Vec<String>,
    pub(crate) allowed_functions: Vec<String>,
    pub(crate) ignored_classes: Vec<String>,
    pub(crate) allowed_classes: Vec<String>,
    pub(crate) repeating_number: u32,
    pub(crate) all_repeating_number: u32,
    pub(crate) number_of_max_executed_function: i32,
}

pub fn run_tests(st: SettingsTaker) {
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open("things.txt").unwrap();

    let all_classes: [(fn(&mut File, &SettingsTaker) -> (), &str); <<number_of_functions>>] = [<<function_arguments>>];
    
    for _i in 0..st.all_repeating_number {
        if st.allowed_classes.is_empty() {
            for (function, name) in all_classes {
                if !st.ignored_classes.contains(&name.to_string()) {
                    function(&mut file, &st);
                }
            }
        } else {
            for (function, name) in all_classes {
                if st.allowed_classes.contains(&name.to_string()) {
                    function(&mut file, &st);
                }
            }
        }
    }
}
"#####;

    // <<type>> - type of used item
    // <<type_lowercase>> - type of used item
    // <<function_list>> - list of functions
    // <<function_number>> - number of functions
    // <<function_class_name>> - number of functions
    let unit_class = r#####"
pub fn <<function_class_name>>(file: &mut File, st: &SettingsTaker) {
    let functions: [(fn(&mut File, &<<type>>) -> (), &str, &str); <<function_number>>] = [<<function_list>>]; // function, function_name_in_rust, function_name 


    println!("////////// Creating object <<type>>");
    let (object,get_string_todo) = gget_<<type_lowercase>>();
    print_and_save_to_file(file, &format!("\nlet thing = {}; // <<type>>",get_string_todo));
    let object_ref = &object;

    let mut functions_to_check: Vec<(fn(&mut File, &<<type>>) -> (), &str, &str)> = Vec::new();    
    if !st.allowed_functions.is_empty() {
        functions
            .into_iter()
            .filter(|e| st.allowed_functions.contains(&e.2.to_string()))
            .for_each(|e| functions_to_check.push(e));
    } else {
        functions
            .into_iter()
            .filter(|e| !st.ignored_functions.contains(&e.2.to_string()))
            .for_each(|e| functions_to_check.push(e));
    }
     
    let number_of_function = if st.number_of_max_executed_function > 0{
        st.number_of_max_executed_function as usize
    } else {
        functions_to_check.len() * st.repeating_number as usize
    };
    
    // Random by default
    for _i in 0..number_of_function {
        // Missing some random functions
        if rand::random::<bool>() {
            let function = functions_to_check.choose(&mut rand::thread_rng()).unwrap().0;
            function(file, object_ref);
        }
    }
}
"#####;

    // <<function_name>> - function name
    // <<type>> - type of used item
    // <<method>> - used method
    let unit_function = r#####"
pub fn <<function_name>>(file: &mut File, thing: &<<type>>) {
    print_and_save_to_file(file, "thing.<<method>>();");
    thing.<<method>>();
}
"#####;

    // <<function_name>> - function name
    // <<type>> - type of used item
    // <<method>> - used method
    // <<println_brackets>> - {} in println to pretty print results
    // <<argument_names>> - arguments names
    // <<argument_names_proper>> - {} in println to pretty print results
    // <<creating_arguments>> - arguments creation
    let unit_multiple_function = r#####"
pub fn <<function_name>>(file: &mut File, thing: &<<type>>) {
    <<creating_arguments>>
    print_and_save_to_file(file, &format!("thing.<<method>>(<<println_brackets>>);",<<argument_names>>));
    thing.<<method>>(<<argument_names_proper>>);
}
"#####;

    writeln!(file, "{}", start_text).unwrap();

    struct ClassUnit {
        a_method: String,
        a_function_name: String,
        a_println_brackets: String,
        a_argument_names: String,
        a_creating_arguments: String,
        a_argument_names_proper: String,
    }
    let mut cu: BTreeMap<String, Vec<ClassUnit>> = Default::default();

    let mut ignored_arguments: BTreeMap<String, u32> = Default::default();

    let mut produced_functions = 0;
    let mut produced_empty_functions = 0;
    let mut produced_multiple_functions = 0;
    for (_index, (name_of_class, function_list)) in class_functions.iter().enumerate() {
        // if name_of_class != "AboutDialog" {
        //     continue;
        // }
        if (RANGE_OF_USED_CLASSES).contains(&_index) {
            // println!("{}. {}", _index, name_of_class);
        } else {
            continue;
        }

        'all: for (function, arguments) in function_list {
            // TODO create here an object
            if function == "new" {
                continue;
            }
            // if !arguments.is_empty() {
            //     continue;
            // }

            let function_name = format!("fct_{}_{}", name_of_class.to_lowercase(), function);

            let mut new_cu: ClassUnit = ClassUnit {
                a_method: function.clone(),
                a_function_name: function_name,
                a_creating_arguments: "".to_string(),
                a_argument_names: "".to_string(),
                a_println_brackets: "".to_string(),
                a_argument_names_proper: "".to_string(),
            };

            if arguments.is_empty() {
            } else {
                // println!("{:?}", arguments);
                let mut found_bad_thing: bool = false;
                // TODO support for all arguments
                for arg in arguments {
                    let mut arg = arg.clone();
                    if arg.starts_with("Option<") {
                        arg = arg.strip_prefix("Option<").unwrap().strip_suffix(">").unwrap().to_string();
                    }

                    found_bad_thing = match arg.as_str() {
                        "bool" | "i32" | "u32" | "u64" | "i64" | "f32" | "f64" | "usize" | "char" | "&str" | "&[&str]" => false,
                        thing => {
                            if IGNORED_CLASSES.contains(&thing) || IGNORED_ENUMS.contains(&thing) || IGNORED_CLASSES.contains(&&thing[1..thing.len()]) {
                                // println!("NOT {}", thing);
                                true
                            } else {
                                // Normal classes like Label
                                if thing.chars().all(|e| e.is_alphabetic()) {
                                    // println!("Supported {:?}", arg);
                                    false
                                } else if thing.starts_with("&implIsA<") && thing.ends_with(">") {
                                    let class = thing.strip_prefix("&implIsA<").unwrap().strip_suffix(">").unwrap();

                                    // TODO maybe support e.g. Widget when using ImplAs<Widget>(currently only all children are supported)
                                    if children_of_class.contains_key(class) && !children_of_class.get(class).unwrap().is_empty() {
                                        // println!("Supported {:?}", arg);
                                        false
                                    } else {
                                        // println!("NOT {:?}", arg);
                                        true
                                    }
                                }
                                //
                                else if thing.contains("<") || thing.contains("[") {
                                    // println!("NOT {:?}", arg);
                                    ignored_arguments.entry(arg.clone()).or_insert(0);
                                    *ignored_arguments.get_mut(&arg).unwrap() += 1;
                                    true
                                }
                                // Supports e.g. &Label
                                else {
                                    // println!("YES {:?}", arg);
                                    false
                                }
                            }
                        }
                    };
                    if found_bad_thing {
                        continue 'all; // Not supported argument
                    }
                }
                if !found_bad_thing {
                    let mut result_arguments = "".to_string();
                    let mut creating_arguments = "".to_string();

                    let mut to_print_arguments = "".to_string();
                    let mut to_print_arguments_variable = "".to_string();

                    for arg_index in 0..arguments.len() {
                        let mut is_implementation = false;
                        let mut is_object = false;
                        let mut is_option_type = false;
                        let mut reference = "";
                        let mut arg = arguments[arg_index].clone();
                        if arg.starts_with("Option<") {
                            is_option_type = true;
                            arg = arg.strip_prefix("Option<").unwrap().to_string();
                            arg = arg.strip_suffix(">").unwrap().to_string();
                        }

                        if arg.starts_with("&") && arg != "&str" && arg != "&[&str]" {
                            reference = "&";
                            arg = arg[1..].to_string();
                        }

                        if arg.starts_with("implIsA<") && arg.ends_with(">") {
                            is_implementation = true;
                            arg = arg.strip_prefix("implIsA<").unwrap().strip_suffix(">").unwrap().to_string();
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
                            "&[&str]" => "take_vec_string".to_string(),
                            thing => {
                                if !enums.contains_key(thing) {
                                    is_object = true;
                                    if is_implementation {
                                        format!("imple_{}", thing.to_lowercase())
                                    } else {
                                        format!("gget_{}", thing.to_lowercase())
                                    }
                                } else {
                                    assert!(!is_implementation, "Enum cannot be implemented as.");
                                    stek = ".0";
                                    format!("stek_{}", thing.to_lowercase())
                                }
                            } // _ => panic!("Not supported {}", arg),
                        };

                        if is_object {
                            creating_arguments += &format!("let (argument_{},object_string_{}) = {}(){}; // {}", arg_index, arg_index, help_function_name, stek, arg);
                        } else {
                            creating_arguments += &format!("let argument_{} = {}(){}; // {}", arg_index, help_function_name, stek, arg);
                        }

                        if arg == "&[&str]" {
                            reference = "&";
                            creating_arguments += "\n\t";
                            creating_arguments += &format!("let argument_{} = get_vector_str_from_string(&argument_{}); // {}", arg_index, arg_index, arg);
                            creating_arguments += "\n\t";
                            creating_arguments += &format!("let argument_{} = argument_{}.as_slice(); // {}", arg_index, arg_index, arg);
                        }
                        if arg_index != arguments.len() - 1 {
                            creating_arguments += "\n\t";
                        }

                        let comma_after = if arg_index == arguments.len() - 1 { "".to_string() } else { ",".to_string() };

                        if is_option_type {
                            result_arguments += &format!("Some({}argument_{}){}", reference, arg_index, comma_after);
                        } else {
                            result_arguments += &format!("{}argument_{}{}", reference, arg_index, comma_after);
                        }

                        let default_formatter = match is_option_type {
                            true => {
                                if reference == "&" {
                                    reference = "";
                                    "Some({}).as_ref()"
                                } else {
                                    "Some({})"
                                }
                            }
                            false => "{}",
                        };

                        // TODO missing printing
                        let add_to_print;
                        if arg == "&str" {
                            if is_option_type {
                                add_to_print = format!("Some({}\\\"{{}}\\\"){}", reference, comma_after);
                            } else {
                                add_to_print = format!("{}\\\"{{}}\\\"{}", reference, comma_after);
                            }
                        } else {
                            add_to_print = format!("{}{}{}", reference, default_formatter, comma_after);
                        }

                        if arg == "&[&str]" {
                            to_print_arguments += &add_to_print.replace("{}", "{:?}");
                        } else {
                            to_print_arguments += &add_to_print;
                        }

                        if is_object {
                            to_print_arguments_variable += &format!("object_string_{}{}", arg_index, comma_after);
                        } else {
                            to_print_arguments_variable += &format!("argument_{}{}", arg_index, comma_after);
                        }
                    }

                    new_cu.a_creating_arguments = creating_arguments;
                    new_cu.a_argument_names = to_print_arguments_variable;
                    new_cu.a_println_brackets = to_print_arguments;
                    new_cu.a_argument_names_proper = result_arguments;
                }
            }
            cu.entry(name_of_class.clone()).or_insert_with(Default::default);
            cu.get_mut(name_of_class).unwrap().push(new_cu);
        }
    }
    produced_functions += 1;

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
            arguments += &format!("({},\"{}\",\"{}\")", f.a_function_name, f.a_function_name, f.a_method);
            if index != functions.len() - 1 {
                arguments += ",";
            }
        }

        let fcn = format!("cls_{}", class_name.to_lowercase());

        let end_class = unit_class
            .replace("<<type>>", &class_name)
            .replace("<<type_lowercase>>", &class_name.to_lowercase())
            .replace("<<function_list>>", &arguments)
            .replace("<<function_class_name>>", &fcn)
            .replace("<<function_number>>", &functions.len().to_string());
        assert!(!end_class.contains("<<"));

        write!(file, "{}", end_class).unwrap();
        for i in functions {
            produced_functions += 1;
            // <<function_name>> - function name
            // <<type>> - type of used item
            // <<method>> - used method
            let single_function = match i.a_creating_arguments.is_empty() {
                true => {
                    produced_empty_functions += 1;
                    unit_function
                        .replace("<<function_name>>", &i.a_function_name)
                        .replace("<<type>>", &class_name)
                        .replace("<<method>>", &i.a_method)
                }
                false => {
                    produced_multiple_functions += 1;
                    unit_multiple_function
                        .replace("<<function_name>>", &i.a_function_name)
                        .replace("<<type>>", &class_name)
                        .replace("<<method>>", &i.a_method)
                        .replace("<<creating_arguments>>", &i.a_creating_arguments)
                        .replace("<<argument_names>>", &i.a_argument_names)
                        .replace("<<println_brackets>>", &i.a_println_brackets)
                        .replace("<<argument_names_proper>>", &i.a_argument_names_proper)
                }
            };
            assert!(!single_function.contains("<<"));
            write!(file, "{}", single_function).unwrap();
        }
    }
    println!(
        "Produced {} functions({} empty, {} multiple)",
        produced_functions, produced_empty_functions, produced_multiple_functions
    );

    if !ignored_arguments.is_empty() {
        let mut vec_results: Vec<(_, _)> = Vec::new();
        for i in ignored_arguments {
            vec_results.push(i);
        }
        vec_results.sort_by(|e, f| if e.1 > f.1 { Ordering::Greater } else { Ordering::Less });
        // for (argument, counter) in vec_results {
        //     println!("{} was used {} times", argument, counter);
        // }
    }

    // Implementation functions
    {}

    let end_text = r####"
    pub fn print_and_save_to_file(file: &mut File, what_to_save: &str) {
    writeln!(file, "{}", what_to_save);
    println!("\t{}", what_to_save);
}"####;
    writeln!(file, "{}", end_text).unwrap();
}
