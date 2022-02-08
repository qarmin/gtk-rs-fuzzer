pub const IGNORED_FUNCTIONS: [&str; 34] = [
    // Non existent functions(probably bug in parsing files)
    "with_entry",               // ComboBox, ComboBoxText
    "toplevels",                // Window
    "list_toplevels",           // Window
    "default_icon_name",        // Window
    "emit_activate",            // MenuButton, ColorButton, AppChooserButton
    "is_primary",               // MenuButton
    "is_indent_for_icon",       // TreeExpander
    "default",                  // RecentManager
    "must_always_show_arrow",   // MenuButton
    "get",                      // AcrivateAction - probably this is static function
    "shows_arrow",              // DropDown
    "current_drop",             // DropTarget
    "is_gtk_hint_font_metrics", // Settings
    // Reported, but not fixes in used GTK/GTK-rs version
    "emit_escape",   // https://github.com/gtk-rs/gtk4-rs/issues/870
    "im_context",    // https://github.com/gtk-rs/gtk4-rs/issues/874
    "insert_prefix", // https://github.com/gtk-rs/gtk4-rs/issues/873
    // TODO
    "emit_stop_search",
    "emit_previous_match",
    "emit_next_match",
    "emit_unselect_all",
    "emit_toggle_cursor_row",
    "emit_select_all",
    "emit_activate_cursor_row",
    "emit_close",
    "emit_select_cursor_item",
    "emit_toggle_cursor_item",
    "emit_unselect_all",
    "emit_select_all",
    "emit_activate_cursor_item",
    "emit_unselect_all",
    "emit_toggle_cursor_child",
    "emit_select_all",
    "emit_activate_cursor_child",
    "drag_dest_item",
];

pub const IGNORED_CLASSES: [&str; 29] = [
    "ATContext",
    "AssistantPage",
    "BuilderListItemFactory",
    "DragIcon",
    "FixedLayoutChild",
    "GridLayoutChild",
    "ListItem",
    "MapListModel",
    "NotebookPage",
    "OverlayLayoutChild",
    "PrintContext",
    "StackPage",
    "TreeListRow",
    "TreeSelection",
    "Widget",
    // Reported, but not fixed in used version
    "NamedAction", // https://github.com/gtk-rs/gtk4-rs/issues/875
    // TODO
    "ColumnViewColumn",
    "DropTarget",          // Reported
    "DropTargetAsync",     // Reported
    "FileChooserWidget",   // TODO .emit_desktop_folder()"
    "ListStore",           // Append
    "PageSetupUnixDialog", //PageSetupUnixDialog.print_settings()
    "PrintJob",            // TODO
    "PrintUnixDialog",     //PrintUnixDialog.selected_printer()
    "Printer",             //Printer.capabilities()
    "SignalAction",        // TODO creating
    "SingleSelection",     //SingleSelection.model
    "TreeListModel",       //TreeListModel.model()
    "TreeStore",           // TODO creating
];
