pub const USE_PARENT_ITEMS: bool = true;
pub const USE_TRAIT_ITEMS: bool = false;
pub const NUMBER_OF_REPEATS: u32 = 2; // How many time repeat function executing to be sure that this function cause problems

pub const IGNORED_FUNCTIONS: &[&str] = &[
    // Non existent functions(probably bug in parsing files)
    "set_show_arrow",            // DropDown
    "add_suffix",                // FileFilter
    "set_always_show_arrow",     // MenuButton
    "set_primary",               // MenuButton
    "set_gtk_hint_font_metrics", // Settings
    "set_indent_for_icon",       // TreeExpander
    // Other
    "alignment", // Rust error, multiple applicable items in scope
    "font_map",  // Rust error, multiple applicable items in scope
    "cursor",    // Rust error, multiple applicable items in scope
    // Reported, but not fixed in used GTK/GTK-rs version
    "emit_escape",      // https://github.com/gtk-rs/gtk4-rs/issues/870
    "im_context",       // https://github.com/gtk-rs/gtk4-rs/issues/874
    "insert_prefix",    // https://github.com/gtk-rs/gtk4-rs/issues/873
    "drag_dest_item",   // https://github.com/gtk-rs/gtk-rs-core/issues/537
    "print_settings",   // https://github.com/gtk-rs/gtk4-rs/issues/880
    "selected_printer", // https://github.com/gtk-rs/gtk4-rs/issues/882
    "renderer",         // https://github.com/gtk-rs/gtk4-rs/issues/886
    "surface",          // https://github.com/gtk-rs/gtk4-rs/issues/886
    "content_type",     // https://github.com/gtk-rs/gtk4-rs/issues/887
    // TODO
    "request_mode",
    "current_path_string",
    "edit_widget",
    "edited_cell",
    "focus_cell",
    "emit_popup",
    "name",
    "widget",
    "current_folder",
    "map",
    "header_bar",
    "realize",
    // "show", // TODO, fix this as fast as possible
    "popup",
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
    "TreeModelSort",
    "ShortcutsSection",
    "Application",
    "ShortcutTrigger",
    "ShortcutAction",
    "PasswordEntryBuffer",
    // Other
    "ApplicationWindow", // Only one ApplicationWindows can be created
    "FileChooserWidget", // Create a lot of warnings(and possibly also crashes) "Too many open files"
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
    // TODO
    "Popover", // Select cause crash
];
