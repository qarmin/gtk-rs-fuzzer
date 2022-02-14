use crate::settings::*;
use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub fn create_enums_file(
    _class_info: &BTreeMap<String, Vec<String>>,
    _class_functions: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    _traits: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    enums: &BTreeMap<String, Vec<String>>,
    _children_of_class: &BTreeMap<String, Vec<String>>,
) {
    let _ = fs::remove_file(PATH_TO_ENUM_FILE);

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
