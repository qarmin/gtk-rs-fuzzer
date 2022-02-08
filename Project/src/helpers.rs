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

pub fn take_bool() -> bool {
    let to_return: bool = rand::random::<bool>();

    debug_printing(&to_return.to_string());
    to_return
}
pub fn stek_orientation() -> Orientation {
    let to_return = if rand::random::<bool>() {
        (Orientation::Horizontal, "Orientation::Horizontal")
    } else {
        (Orientation::Vertical, "Orientation::Vertical")
    };
    debug_printing(to_return.1);
    to_return.0
}
pub fn stek_glib_type() -> glib::Type {
    let to_return = [
        (glib::Type::INVALID, "INVALID"),
        (glib::Type::UNIT, "UNIT"),
        (glib::Type::I8, "I8"),
        (glib::Type::U8, "U8"),
        (glib::Type::BOOL, "BOOL"),
        (glib::Type::I32, "I32"),
        (glib::Type::U32, "U32"),
        (glib::Type::I_LONG, "I_LONG"),
        (glib::Type::U_LONG, "U_LONG"),
        (glib::Type::I64, "I64"),
        (glib::Type::U64, "U64"),
        (glib::Type::F32, "F32"),
        (glib::Type::F64, "F64"),
        (glib::Type::STRING, "STRING"),
        (glib::Type::POINTER, "POINTER"),
        (glib::Type::VARIANT, "VARIANT"),
        (glib::Type::INTERFACE, "INTERFACE"),
        (glib::Type::ENUM, "ENUM"),
        (glib::Type::FLAGS, "FLAGS"),
        (glib::Type::BOXED, "BOXED"),
        (glib::Type::PARAM_SPEC, "PARAM_SPEC"),
        (glib::Type::OBJECT, "OBJECT"),
    ]
    .choose(&mut rand::thread_rng())
    .unwrap();

    debug_printing(to_return.1);
    return to_return.0;
}
