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
pub fn stek_orientation() -> (Orientation, String) {
    let to_return = if rand::random::<bool>() {
        (Orientation::Horizontal, "Orientation::Horizontal".to_string())
    } else {
        (Orientation::Vertical, "Orientation::Vertical".to_string())
    };
    debug_printing(&to_return.1);
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

const CONSTRAINT_STRENGTH_TYPE: [(ConstraintStrength, &str); 4] = [
    (ConstraintStrength::Strong, "ConstraintStrength::Strong"),
    (ConstraintStrength::Medium, "ConstraintStrength::Medium"),
    (ConstraintStrength::Weak, "ConstraintStrength::Weak"),
    (ConstraintStrength::Required, "ConstraintStrength::Required"),
];

pub fn stek_constraintstrength() -> (ConstraintStrength, String) {
    let to_return = CONSTRAINT_STRENGTH_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}

const LICENSE_TYPE: [(License, &str); 18] = [
    (License::Unknown, "License::Unknown"),
    (License::Custom, "License::Custom"),
    (License::Gpl20, "License::Gpl20"),
    (License::Gpl30, "License::Gpl30"),
    (License::Lgpl21, "License::Lgpl21"),
    (License::Lgpl30, "License::Lgpl30"),
    (License::Bsd, "License::Bsd"),
    (License::MitX11, "License::MitX11"),
    (License::Artistic, "License::Artistic"),
    (License::Gpl20Only, "License::Gpl20Only"),
    (License::Gpl30Only, "License::Gpl30Only"),
    (License::Lgpl21Only, "License::Lgpl21Only"),
    (License::Lgpl30Only, "License::Lgpl30Only"),
    (License::Agpl30, "License::Agpl30"),
    (License::Agpl30Only, "License::Agpl30Only"),
    (License::Bsd3, "License::Bsd3"),
    (License::Apache20, "License::Apache20"),
    (License::Mpl20, "License::Mpl20"),
];

pub fn stek_license() -> (License, String) {
    let to_return = LICENSE_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}
const BASELINE_TYPE: [(BaselinePosition, &str); 3] = [
    (BaselinePosition::Top, "BaselinePosition::Top"),
    (BaselinePosition::Bottom, "BaselinePosition::Bottom"),
    (BaselinePosition::Center, "BaselinePosition::Center"),
];

pub fn stek_baselineposition() -> (BaselinePosition, String) {
    let to_return = BASELINE_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}

const CellRendererAccelMode_TYPE: [(CellRendererAccelMode, &str); 2] = [(CellRendererAccelMode::Gtk, "CellRendererAccelMode::Gtk"), (CellRendererAccelMode::Other, "CellRendererAccelMode::Other")];

pub fn stek_cellrendereraccelmode() -> (CellRendererAccelMode, String) {
    let to_return = CellRendererAccelMode_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}

const iconsize_TYPE: [(IconSize, &str); 3] = [(IconSize::Inherit, "IconSize::Inherit"), (IconSize::Normal, "IconSize::Normal"), (IconSize::Large, "IconSize::Large")];

pub fn stek_iconsize() -> (IconSize, String) {
    let to_return = iconsize_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}

const pickflags_TYPE: [(PickFlags, &str); 3] = [
    (PickFlags::DEFAULT, "PickFlags::DEFAULT"),
    (PickFlags::INSENSITIVE, "PickFlags::INSENSITIVE"),
    (PickFlags::NON_TARGETABLE, "PickFlags::NON_TARGETABLE"),
];

pub fn stek_pickflags() -> (PickFlags, String) {
    let to_return = pickflags_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}

const directiontype_TYPE: [(DirectionType, &str); 6] = [
    (DirectionType::TabForward, "DirectionType::TabForward"),
    (DirectionType::TabBackward, "DirectionType::TabBackward"),
    (DirectionType::Up, "DirectionType::Up"),
    (DirectionType::Down, "DirectionType::Down"),
    (DirectionType::Left, "DirectionType::Left"),
    (DirectionType::Right, "DirectionType::Right"),
];

pub fn stek_directiontype() -> (DirectionType, String) {
    let to_return = directiontype_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}

// TODO
const accessibleproperty_TYPE: [(AccessibleProperty, &str); 1] = [(AccessibleProperty::Autocomplete, "AccessibleProperty::Autocomplete")];

pub fn stek_accessibleproperty() -> (AccessibleProperty, String) {
    let to_return = accessibleproperty_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}

// TODO
const accessiblerole_TYPE: [(AccessibleRole, &str); 1] = [(AccessibleRole::Alert, "AccessibleRole::Alert")];

pub fn stek_accessiblerole() -> (AccessibleRole, String) {
    let to_return = accessiblerole_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}
const textdirection_TYPE: [(TextDirection, &str); 1] = [(TextDirection::None, "TextDirection::None")];

pub fn stek_textdirection() -> (TextDirection, String) {
    let to_return = textdirection.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}

const align_TYPE: [(Align, &str); 1] = [(Align::Fill, "Align::Fill")];

pub fn stek_align() -> (Align, String) {
    let to_return = align_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}

const overflow_TYPE: [(Overflow, &str); 1] = [(Overflow::Visible, "Overflow::Visible")];

pub fn stek_overflow() -> (Overflow, String) {
    let to_return = overflow_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}

const stateflags_TYPE: [(StateFlags, &str); 1] = [(StateFlags::NORMAL, "StateFlags::NORMAL")];

pub fn stek_stateflags() -> (StateFlags, String) {
    let to_return = stateflags_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}
const accessiblerelation_TYPE: [(AccessibleRelation, &str); 1] = [(AccessibleRelation::NORMAL, "AccessibleRelation::NORMAL")];

pub fn stek_accessiblerelation() -> (AccessibleRelation, String) {
    let to_return = accessiblerelation_TYPE.choose(&mut rand::thread_rng()).unwrap();
    let to_return = (to_return.0, to_return.1.to_string());

    debug_printing(&to_return.1);
    return to_return;
}

// pub fn fsf(){
//     StateFlags::
// }

// // TODO
// const SMOLL_TYPE: [(NORMALL, &str); 1] = [(NORMALL::NORMAL, "NORMALL::NORMAL")];
//
// pub fn stek_SMOLL() -> (NORMALL, String) {
//     let to_return = SMOLL_TYPE.choose(&mut rand::thread_rng()).unwrap();
//     let to_return = (to_return.0, to_return.1.to_string());
//
//     debug_printing(&to_return.1);
//     return to_return;
// }
//
//
//
//
// SMOLL
// NORMALL
