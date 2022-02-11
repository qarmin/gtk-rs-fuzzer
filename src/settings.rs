use std::ops::Range;

pub const USE_PARENT_ITEMS: bool = true;
pub const USE_TRAIT_ITEMS: bool = false;
// pub const NUMBER_OF_REPEATS: u32 = 2; // How many time repeat function executing to be sure that this function cause problems

pub const RANGE_OF_USED_CLASSES: Range<usize> = 0..200;

pub const IGNORED_FUNCTIONS: &[&str] = &[
    // Non existent functions(probably bug in parsing files or this will be available in 4.6 + versions)
    "set_show_arrow",            // DropDown
    "add_suffix",                // FileFilter
    "set_always_show_arrow",     // MenuButton
    "set_primary",               // MenuButton
    "set_gtk_hint_font_metrics", // Settings
    "set_indent_for_icon",       // TreeExpander
    "emit_activate",             // AppChooserButton
    "shows_arrow",               // DropDown
    "current_drop",              // DropTarget
    "natural_wrap_mode",         // Label
    "is_primary",                // MenuButton
    "is_gtk_hint_font_metrics",  // Settings
    "must_always_show_arrow",    // MenuButton
    "child",                     // MenuButton
    "is_indent_for_icon",        // TreeExpander
    "compute_cursor_extents",    // Text
    // Other
    "set_direction", // Some parsing problem
    // Rust error, multiple applicable items in scope
    "alignment",
    "font_map",
    "cursor",
    "set_alignment",
    // Not reported, but panic are self describing
    "set_height_request",
    "set_width_request",
    "set_day",
    "set_month",
    "set_year",
    "set_height",
    "set_max_width_chars",
    "set_scale",
    "set_weight",
    "set_size",
    "set_size_points",
    "set_width",
    "set_width_chars",
    "set_wrap_width",
    "set_xalign",
    "set_yalign",
    "set_text_column",
    "set_text_xalign",
    "set_text_yalign",
    "set_pulse",
    "set_value",
    "set_digits",
    "set_climb_rate",
    "set_max_height",
    "set_max_width",
    "set_min_width",
    "set_min_height",
    "set_nat_height",
    "set_nat_width",
    "set_page",
    "set_gtk_cursor_aspect_ratio",
    "set_gtk_cursor_blink_time",
    "set_gtk_cursor_theme_size",
    "set_gtk_cursor_blink_timeout",
    "set_gtk_dnd_drag_threshold",
    "set_gtk_double_click_time",
    "set_gtk_recent_files_max_age",
    "set_gtk_double_click_distance",
    "set_gtk_xft_antialias",
    "set_gtk_xft_dpi",
    "set_gtk_xft_hinting",
    "",
    "",
    "",
    "",
    "",
    // Reported, but not fixed in used GTK/GTK-rs version
    "emit_escape",         // https://github.com/gtk-rs/gtk4-rs/issues/870
    "im_context",          // https://github.com/gtk-rs/gtk4-rs/issues/874
    "insert_prefix",       // https://github.com/gtk-rs/gtk4-rs/issues/873
    "drag_dest_item",      // https://github.com/gtk-rs/gtk-rs-core/issues/537
    "print_settings",      // https://github.com/gtk-rs/gtk4-rs/issues/880
    "selected_printer",    // https://github.com/gtk-rs/gtk4-rs/issues/882
    "renderer",            // https://github.com/gtk-rs/gtk4-rs/issues/886
    "surface",             // https://github.com/gtk-rs/gtk4-rs/issues/886
    "content_type",        // https://github.com/gtk-rs/gtk4-rs/issues/887
    "set_current_page",    // https://github.com/gtk-rs/gtk4-rs/issues/888
    "request_mode",        // https://github.com/gtk-rs/gtk4-rs/issues/889
    "current_path_string", // https://github.com/gtk-rs/gtk4-rs/issues/890
    "edit_widget",
    "edited_cell",
    "focus_cell",
    "emit_popup",
    "name",
    "widget",
    "chars",
    "current_folder",
    "current_name",
    "font_features",
    // Pack Report
    "preview_text",
    "map",
    "theme_name",
    "uri",
    "set_detailed_action_name",
    "header_bar",
    "emit_cycle_handle_focus",
    "popup",
    "realize",
    "set_visible",
    "show",
    "set_language",
    "emit_move_cursor",
    "to_node",
    "gl_shader_pop_texture",
    "pop",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
];

pub const FUNCTIONS_TO_USE: &[&str] = &[];

// List of classes which will be used(IGNORED_CLASSES NOT ignore classes from this array).
// If empty, all classes which are not present in IGNORED_CLASSES are used
pub const CLASSES_TO_USE: &[&str] = &[];

pub const IGNORED_ENUMS: &[&str] = &[
    "SymbolicColor",
    "NaturalWrapMode",
    "State",
    "EventControllerScrollFlags", // Bitflags
    "InputHints",                 // Bitflags
    "StateFlags",                 // BitFlags
    "PickFlags",                  // Bitflags
    "FontChooserLevel",           // Bitflags
];

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
    "Tooltip",
    "PrintCapabilities",
    "Printer",
    "PrintJob",
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
