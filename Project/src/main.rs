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

        if CRASHES == 1 {
            crashes();
        } else {
            execute_things();
        }
    });

    application.run();
}

const CRASHES: u64 = 0;
fn crashes() {
    println!("TESTSTTSTSTSTSTSTST");
}
