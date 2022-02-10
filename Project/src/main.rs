#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

mod create_objects;
mod helpers;
mod ziemniak;

use crate::create_objects::*;
use crate::ziemniak::execute_things;
use gtk4::prelude::*;
use gtk4::*;

fn main() {
    let application = gtk4::Application::builder().build();
    application.connect_activate(move |application| {
        let window = gtk4::ApplicationWindow::new(application);
        window.set_title(Some("Fuzzer gtk-rs"));

        window.show();

        if CRASHES == 0 {
            crashes();
        } else {
            execute_things();
        }
    });

    application.run();
}

const CRASHES: u64 = 1;
fn crashes() {
    println!("TESTSTTSTSTSTSTSTST");
    // BinLayout.request_mode()
    // CellAreaBox.current_path_string()
    // CellAreaBox.edit_widget()
    // CellAreaBox.edited_cell()
    // CellAreaBox.focus_cell()
    // ComboBoxText.emit_popup()
    // DragSource.name()
    // DragSource.widget()
    // FileChooserDialog.current_folder()
    // FontChooserWidget.map()
    // MessageDialog.header_bar()
    // Popover.realize()
    // Popover.show()
    // PopoverMenu.popup()
    // gtk4::AppChooserDialog::default().content_type();
}
