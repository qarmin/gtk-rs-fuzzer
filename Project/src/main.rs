mod helpers;
mod ziemniak;

use crate::ziemniak::execute_things;
use gtk4::prelude::*;

const CRASHES: u64 = 1;

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

fn crashes() {
    // gtk4::DropTarget::new(glib::Type::BOOL, DragAction::COPY).formats();
    // gtk4::SearchEntry::new().emit_stop_search();
    // gtk4::SearchEntry::new().emit_previous_match();
    // gtk4::SearchEntry::new().emit_next_match();
    // gtk4::ListBox::new().emit_unselect_all();
    // gtk4::SearchEntry::new().emit_stop_search();
    // gtk4::SearchEntry::new().emit_previous_match();
    // gtk4::SearchEntry::new().emit_next_match();
    // gtk4::ListBox::new().emit_unselect_all();
    // gtk4::ListBox::new().emit_toggle_cursor_row();
    // gtk4::ListBox::new().emit_select_all();
    // gtk4::ListBox::new().emit_activate_cursor_row();
    // gtk4::InfoBar::new().emit_close();
    // gtk4::IconView::new().emit_select_cursor_item();
    // gtk4::IconView::new().emit_toggle_cursor_item();
    // gtk4::IconView::new().emit_unselect_all();
    // gtk4::IconView::new().emit_select_all();
    // gtk4::IconView::new().emit_activate_cursor_item();
    // gtk4::FlowBox::new().emit_unselect_all();
    // gtk4::FlowBox::new().emit_toggle_cursor_child();
    // gtk4::FlowBox::new().emit_select_all();
    // gtk4::FlowBox::new().emit_activate_cursor_child();
    // gtk4::IconView::new().drag_dest_item();
    // gtk4::EventControllerKey::new().im_context();
    // gtk4::EntryCompletion::new().insert_prefix();
    // gtk4::Assistant::new().emit_escape();
    // gtk4::ListBox::new().emit_toggle_cursor_row();
    // gtk4::ListBox::new().emit_select_all();
    // gtk4::ListBox::new().emit_activate_cursor_row();
    // gtk4::InfoBar::new().emit_close();
    // gtk4::IconView::new().emit_select_cursor_item();
    // gtk4::IconView::new().emit_toggle_cursor_item();
    // gtk4::IconView::new().emit_unselect_all();
    // gtk4::IconView::new().emit_select_all();
    // gtk4::IconView::new().emit_activate_cursor_item();
    // gtk4::FlowBox::new().emit_unselect_all();
    // gtk4::FlowBox::new().emit_toggle_cursor_child();
    // gtk4::FlowBox::new().emit_select_all();
    // gtk4::FlowBox::new().emit_activate_cursor_child();
    // gtk4::IconView::new().drag_dest_item();
    // gtk4::EventControllerKey::new().im_context();
    // gtk4::EntryCompletion::new().insert_prefix();
    // gtk4::Assistant::new().emit_escape();
}
