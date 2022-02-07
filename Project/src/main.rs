mod ziemniak;

use crate::ziemniak::execute_things;
use gtk4::prelude::*;

fn main() {
    let application = gtk4::Application::builder().build();
    application.connect_activate(move |application| {
        let window = gtk4::ApplicationWindow::new(application);
        window.set_title(Some("Fuzzer gtk-rs"));

        window.show();

        execute_things();
    });

    application.run();
}
