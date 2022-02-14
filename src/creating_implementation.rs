use crate::settings::*;
use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub fn create_implementation_file(
    _class_info: &BTreeMap<String, Vec<String>>,
    _class_functions: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    _traits: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    _enums: &BTreeMap<String, Vec<String>>,
    children_of_class: &BTreeMap<String, Vec<String>>,
) {
    let _ = fs::remove_file(PATH_TO_IMPLEMENTATIONS);

    let file = OpenOptions::new().write(true).truncate(true).create(true).open(PATH_TO_IMPLEMENTATIONS).unwrap();
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

    // type - base type e.g. Widget (ImplAs<Widget>)
    // type_lowercase - base type lowercase
    // number_of_records - number of records
    // items -
    let single_impl_template = r#####"
pub fn imple_<<type_lowercase>>() -> (<<type>>, &'static str) {
    let number_of_records = <<number_of_records>>;

    match thread_rng().gen_range(0 as usize..number_of_records as usize) {
        <<items>>
    }
}
"#####;

    for (name_of_class, children_list) in children_of_class {
        println!("TRYING {}", name_of_class);
        if children_list.is_empty() || IGNORED_IMPLEMENTATIONS.contains(&name_of_class.as_str()) {
            continue;
        }

        let mut to_write = single_impl_template
            .replace("<<type>>", name_of_class)
            .replace("<<type_lowercase>>", &name_of_class.to_lowercase())
            .replace("<<number_of_records>>", &children_list.len().to_string());
        let mut arguments = "".to_string();
        for (index, child_of_item) in children_list.iter().enumerate() {
            arguments += &format!(
                "{} => (gget_{}().0.upcast::<{}>(), \"{}\"),",
                index,
                child_of_item.to_lowercase(),
                name_of_class,
                child_of_item
            );
            arguments += "\n";
            if index != child_of_item.len() - 1 {
                arguments += "        ";
            }
        }
        arguments += "_ => panic!()";
        to_write = to_write.replace("<<items>>", &arguments);
        assert!(!to_write.contains("<<"));
        writeln!(file, "{}", to_write).unwrap();
    }
}
