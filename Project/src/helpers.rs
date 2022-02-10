use gtk4::*;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

const DEBUG_PRINTING: bool = true;
pub fn debug_printing(what_to_print: &str) {
    if DEBUG_PRINTING {
        println!("Using argument \"{}\"(text length {})", what_to_print, what_to_print.len());
    }
}

pub fn take_string() -> String {
    let to_return;

    if rand::random::<bool>() {
        to_return = "".to_string();
    } else {
        to_return = thread_rng().gen_range(-100000..100000).to_string();
    }
    debug_printing(&to_return);
    to_return
}
pub fn take_i32() -> i32 {
    let to_return: i32 = thread_rng().gen_range(-100000..100000);

    debug_printing(&to_return.to_string());
    to_return
}
pub fn take_i64() -> i64 {
    let to_return: i64 = thread_rng().gen_range(-100000..100000);

    debug_printing(&to_return.to_string());
    to_return
}
pub fn take_u32() -> u32 {
    let to_return: u32 = thread_rng().gen_range(0..100000);

    debug_printing(&to_return.to_string());
    to_return
}
pub fn take_u64() -> u64 {
    let to_return: u64 = thread_rng().gen_range(0..100000);

    debug_printing(&to_return.to_string());
    to_return
}
pub fn take_f64() -> f64 {
    let to_return: f64 = thread_rng().gen_range(-100000.0..100000.0);

    debug_printing(&to_return.to_string());
    to_return
}
pub fn take_f32() -> f32 {
    let to_return: f32 = thread_rng().gen_range(-100000.0..100000.0);

    debug_printing(&to_return.to_string());
    to_return
}
pub fn take_usize() -> usize {
    let to_return: usize = thread_rng().gen_range(0..100000);

    debug_printing(&to_return.to_string());
    to_return
}
pub fn take_char() -> char {
    let to_return: char = thread_rng().gen_range(0..127) as u8 as char;

    debug_printing(&to_return.to_string());
    to_return
}

pub fn take_bool() -> bool {
    let to_return: bool = rand::random::<bool>();

    debug_printing(&to_return.to_string());
    to_return
}

const S_LIB_TYPE: [(glib::Type, &str); 22] = [
    (glib::Type::INVALID, "glib::Type::INVALID"),
    (glib::Type::UNIT, "glib::Type::UNIT"),
    (glib::Type::I8, "glib::Type::I8"),
    (glib::Type::U8, "glib::Type::U8"),
    (glib::Type::BOOL, "glib::Type::BOOL"),
    (glib::Type::I32, "glib::Type::I32"),
    (glib::Type::U32, "glib::Type::U32"),
    (glib::Type::I_LONG, "glib::Type::I_LONG"),
    (glib::Type::U_LONG, "glib::Type::U_LONG"),
    (glib::Type::I64, "glib::Type::I64"),
    (glib::Type::U64, "glib::Type::U64"),
    (glib::Type::F32, "glib::Type::F32"),
    (glib::Type::F64, "glib::Type::F64"),
    (glib::Type::STRING, "glib::Type::STRING"),
    (glib::Type::POINTER, "glib::Type::POINTER"),
    (glib::Type::VARIANT, "glib::Type::VARIANT"),
    (glib::Type::INTERFACE, "glib::Type::INTERFACE"),
    (glib::Type::ENUM, "glib::Type::ENUM"),
    (glib::Type::FLAGS, "glib::Type::FLAGS"),
    (glib::Type::BOXED, "glib::Type::BOXED"),
    (glib::Type::PARAM_SPEC, "glib::Type::PARAM_SPEC"),
    (glib::Type::OBJECT, "glib::Type::OBJECT"),
];

pub fn stek_glib_type() -> (glib::Type, String) {
    let to_return = S_LIB_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}
