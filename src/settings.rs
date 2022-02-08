pub const IGNORED_FUNCTIONS: &[&str] = &[
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
    // Reported, but not fixed in used GTK/GTK-rs version
    "emit_escape",    // https://github.com/gtk-rs/gtk4-rs/issues/870
    "im_context",     // https://github.com/gtk-rs/gtk4-rs/issues/874
    "insert_prefix",  // https://github.com/gtk-rs/gtk4-rs/issues/873
    "drag_dest_item", // https://github.com/gtk-rs/gtk-rs-core/issues/537
    "print_settings", // https://github.com/gtk-rs/gtk4-rs/issues/880
    "selected_printer", // https://github.com/gtk-rs/gtk4-rs/issues/882
                      // TODO
];

// List of classes which will be used(IGNORED_CLASSES NOT ignore classes from this array).
// If empty, all classes which are not present in IGNORED_CLASSES are used
pub const CLASSES_TO_USE: &[&str] = &[];

pub const IGNORED_CLASSES: &[&str] = &[
    // Classes, which objects I can't create
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
    // Reported, but crashes etc. are expected
    "NamedAction",  // https://github.com/gtk-rs/gtk4-rs/issues/875
    "SignalAction", // Error same as in NamedAction
    "ListStore",    // https://github.com/gtk-rs/gtk4-rs/issues/878
    "TreeStore",    // https://github.com/gtk-rs/gtk4-rs/issues/878
    // Reported, but not fixed in used version GTK/GTK-rs
    "PrintJob",         // https://github.com/gtk-rs/gtk4-rs/issues/881
    "Printer",          // https://github.com/gtk-rs/gtk4-rs/issues/883
    "SingleSelection",  // https://github.com/gtk-rs/gtk-rs-core/issues/539
    "TreeListModel",    // https://github.com/gtk-rs/gtk-rs-core/issues/539
    "ColumnViewColumn", // https://github.com/gtk-rs/gtk4-rs/issues/885
];
