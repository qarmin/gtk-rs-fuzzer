use glib::{random_double_range, random_int, random_int_range};
use gtk4::prelude::*;
use gtk4::*;

pub fn execute_things() {
    // ATContext
    {
        let _object0 = gget_atcontext(); //ZZZZ ATContext YYYY gget_atcontext
        _object0.accessible();
        let _object1 = gget_atcontext(); //ZZZZ ATContext YYYY gget_atcontext
        _object1.accessible_role();
        let _object2 = gget_atcontext(); //ZZZZ ATContext YYYY gget_atcontext
        _object2.display();
    }
    // AboutDialog
    {
        let _object4 = gget_aboutdialog(); //ZZZZ AboutDialog YYYY gget_aboutdialog
        _object4.logo();
        let _object6 = gget_aboutdialog(); //ZZZZ AboutDialog YYYY gget_aboutdialog
        _object6.wraps_license();
    }
    // ActionBar
    {
        let _object8 = gget_actionbar(); //ZZZZ ActionBar YYYY gget_actionbar
        _object8.center_widget();
        let _object9 = gget_actionbar(); //ZZZZ ActionBar YYYY gget_actionbar
        _object9.is_revealed();
    }
    // ActivateAction
    {
        let _object11 = gget_activateaction(); //ZZZZ ActivateAction YYYY gget_activateaction
        _object11.get();
    }
    // Adjustment
    {}
    // AlternativeTrigger
    {}
    // AnyFilter
    {}
    // AppChooserButton
    {
        let _object17 = gget_appchooserbutton(); //ZZZZ AppChooserButton YYYY gget_appchooserbutton
        _object17.append_separator();
        let _object19 = gget_appchooserbutton(); //ZZZZ AppChooserButton YYYY gget_appchooserbutton
        _object19.emit_activate();
        let _object20 = gget_appchooserbutton(); //ZZZZ AppChooserButton YYYY gget_appchooserbutton
        _object20.is_modal();
        let _object22 = gget_appchooserbutton(); //ZZZZ AppChooserButton YYYY gget_appchooserbutton
        _object22.shows_default_item();
        let _object23 = gget_appchooserbutton(); //ZZZZ AppChooserButton YYYY gget_appchooserbutton
        _object23.shows_dialog_item();
    }
    // AppChooserDialog
    {
        let _object25 = gget_appchooserdialog(); //ZZZZ AppChooserDialog YYYY gget_appchooserdialog
        _object25.gfile();
        let _object27 = gget_appchooserdialog(); //ZZZZ AppChooserDialog YYYY gget_appchooserdialog
        _object27.widget();
    }
    // AppChooserWidget
    {
        let _object30 = gget_appchooserwidget(); //ZZZZ AppChooserWidget YYYY gget_appchooserwidget
        _object30.shows_all();
        let _object31 = gget_appchooserwidget(); //ZZZZ AppChooserWidget YYYY gget_appchooserwidget
        _object31.shows_default();
        let _object32 = gget_appchooserwidget(); //ZZZZ AppChooserWidget YYYY gget_appchooserwidget
        _object32.shows_fallback();
        let _object33 = gget_appchooserwidget(); //ZZZZ AppChooserWidget YYYY gget_appchooserwidget
        _object33.shows_other();
        let _object34 = gget_appchooserwidget(); //ZZZZ AppChooserWidget YYYY gget_appchooserwidget
        _object34.shows_recommended();
    }
    // Application
    {}
    // ApplicationWindow
    {}
    // AspectFrame
    {
        let _object40 = gget_aspectframe(); //ZZZZ AspectFrame YYYY gget_aspectframe
        _object40.is_obey_child();
    }
    // Assistant
    {
        let _object43 = gget_assistant(); //ZZZZ Assistant YYYY gget_assistant
        _object43.commit();
        let _object44 = gget_assistant(); //ZZZZ Assistant YYYY gget_assistant
        _object44.current_page();
        let _object45 = gget_assistant(); //ZZZZ Assistant YYYY gget_assistant
        _object45.emit_escape();
        let _object46 = gget_assistant(); //ZZZZ Assistant YYYY gget_assistant
        _object46.n_pages();
        let _object48 = gget_assistant(); //ZZZZ Assistant YYYY gget_assistant
        _object48.next_page();
        let _object49 = gget_assistant(); //ZZZZ Assistant YYYY gget_assistant
        _object49.pages();
        let _object50 = gget_assistant(); //ZZZZ Assistant YYYY gget_assistant
        _object50.previous_page();
        let _object51 = gget_assistant(); //ZZZZ Assistant YYYY gget_assistant
        _object51.update_buttons_state();
    }
    // AssistantPage
    {
        let _object52 = gget_assistantpage(); //ZZZZ AssistantPage YYYY gget_assistantpage
        _object52.child();
        let _object53 = gget_assistantpage(); //ZZZZ AssistantPage YYYY gget_assistantpage
        _object53.is_complete();
        let _object54 = gget_assistantpage(); //ZZZZ AssistantPage YYYY gget_assistantpage
        _object54.page_type();
        let _object55 = gget_assistantpage(); //ZZZZ AssistantPage YYYY gget_assistantpage
        _object55.title();
    }
    // BinLayout
    {}
    // BookmarkList
    {
        let _object57 = gget_bookmarklist(); //ZZZZ BookmarkList YYYY gget_bookmarklist
        _object57.attributes();
        let _object58 = gget_bookmarklist(); //ZZZZ BookmarkList YYYY gget_bookmarklist
        _object58.filename();
        let _object59 = gget_bookmarklist(); //ZZZZ BookmarkList YYYY gget_bookmarklist
        _object59.is_loading();
    }
    // BoolFilter
    {
        let _object61 = gget_boolfilter(); //ZZZZ BoolFilter YYYY gget_boolfilter
        _object61.inverts();
    }
    // Box
    {}
    // BoxLayout
    {
        let _object66 = gget_boxlayout(); //ZZZZ BoxLayout YYYY gget_boxlayout
        _object66.is_homogeneous();
    }
    // Builder
    {
        let _object69 = gget_builder(); //ZZZZ Builder YYYY gget_builder
        _object69.objects();
        let _object70 = gget_builder(); //ZZZZ Builder YYYY gget_builder
        _object70.scope();
        let _object71 = gget_builder(); //ZZZZ Builder YYYY gget_builder
        _object71.translation_domain();
    }
    // BuilderListItemFactory
    {
        let _object72 = gget_builderlistitemfactory(); //ZZZZ BuilderListItemFactory YYYY gget_builderlistitemfactory
        _object72.bytes();
        let _object73 = gget_builderlistitemfactory(); //ZZZZ BuilderListItemFactory YYYY gget_builderlistitemfactory
        _object73.resource();
        let _object74 = gget_builderlistitemfactory(); //ZZZZ BuilderListItemFactory YYYY gget_builderlistitemfactory
        _object74.scope();
    }
    // Button
    {}
    // Calendar
    {
        let _object78 = gget_calendar(); //ZZZZ Calendar YYYY gget_calendar
        _object78.clear_marks();
        let _object79 = gget_calendar(); //ZZZZ Calendar YYYY gget_calendar
        _object79.date();
        let _object81 = gget_calendar(); //ZZZZ Calendar YYYY gget_calendar
        _object81.shows_day_names();
        let _object82 = gget_calendar(); //ZZZZ Calendar YYYY gget_calendar
        _object82.shows_heading();
        let _object83 = gget_calendar(); //ZZZZ Calendar YYYY gget_calendar
        _object83.shows_week_numbers();
    }
    // CellAreaBox
    {}
    // CellRendererAccel
    {
        let _object86 = gget_cellrendereraccel(); //ZZZZ CellRendererAccel YYYY gget_cellrendereraccel
        _object86.accel_mods();
    }
    // CellRendererCombo
    {}
    // CellRendererPixbuf
    {
        let _object92 = gget_cellrendererpixbuf(); //ZZZZ CellRendererPixbuf YYYY gget_cellrendererpixbuf
        _object92.gicon();
        let _object94 = gget_cellrendererpixbuf(); //ZZZZ CellRendererPixbuf YYYY gget_cellrendererpixbuf
        _object94.pixbuf_expander_closed();
        let _object95 = gget_cellrendererpixbuf(); //ZZZZ CellRendererPixbuf YYYY gget_cellrendererpixbuf
        _object95.pixbuf_expander_open();
        let _object96 = gget_cellrendererpixbuf(); //ZZZZ CellRendererPixbuf YYYY gget_cellrendererpixbuf
        _object96.texture();
    }
    // CellRendererProgress
    {
        let _object98 = gget_cellrendererprogress(); //ZZZZ CellRendererProgress YYYY gget_cellrendererprogress
        _object98.is_inverted();
    }
    // CellRendererSpin
    {}
    // CellRendererSpinner
    {
        let _object103 = gget_cellrendererspinner(); //ZZZZ CellRendererSpinner YYYY gget_cellrendererspinner
        _object103.is_active();
    }
    // CellRendererText
    {}
    // CellRendererToggle
    {
        let _object108 = gget_cellrenderertoggle(); //ZZZZ CellRendererToggle YYYY gget_cellrenderertoggle
        _object108.is_activatable();
        let _object109 = gget_cellrenderertoggle(); //ZZZZ CellRendererToggle YYYY gget_cellrenderertoggle
        _object109.is_active();
        let _object110 = gget_cellrenderertoggle(); //ZZZZ CellRendererToggle YYYY gget_cellrenderertoggle
        _object110.is_inconsistent();
        let _object111 = gget_cellrenderertoggle(); //ZZZZ CellRendererToggle YYYY gget_cellrenderertoggle
        _object111.is_radio();
    }
    // CellView
    {
        let _object114 = gget_cellview(); //ZZZZ CellView YYYY gget_cellview
        _object114.displayed_row();
        let _object115 = gget_cellview(); //ZZZZ CellView YYYY gget_cellview
        _object115.draws_sensitive();
        let _object116 = gget_cellview(); //ZZZZ CellView YYYY gget_cellview
        _object116.fits_model();
    }
    // CenterBox
    {
        let _object119 = gget_centerbox(); //ZZZZ CenterBox YYYY gget_centerbox
        _object119.center_widget();
        let _object120 = gget_centerbox(); //ZZZZ CenterBox YYYY gget_centerbox
        _object120.end_widget();
        let _object122 = gget_centerbox(); //ZZZZ CenterBox YYYY gget_centerbox
        _object122.start_widget();
    }
    // CenterLayout
    {
        let _object123 = gget_centerlayout(); //ZZZZ CenterLayout YYYY gget_centerlayout
        _object123.baseline_position();
        let _object124 = gget_centerlayout(); //ZZZZ CenterLayout YYYY gget_centerlayout
        _object124.center_widget();
        let _object125 = gget_centerlayout(); //ZZZZ CenterLayout YYYY gget_centerlayout
        _object125.end_widget();
        let _object127 = gget_centerlayout(); //ZZZZ CenterLayout YYYY gget_centerlayout
        _object127.orientation();
        let _object128 = gget_centerlayout(); //ZZZZ CenterLayout YYYY gget_centerlayout
        _object128.start_widget();
    }
    // CheckButton
    {}
    // ColorButton
    {
        let _object132 = gget_colorbutton(); //ZZZZ ColorButton YYYY gget_colorbutton
        _object132.emit_activate();
        let _object133 = gget_colorbutton(); //ZZZZ ColorButton YYYY gget_colorbutton
        _object133.is_modal();
        let _object135 = gget_colorbutton(); //ZZZZ ColorButton YYYY gget_colorbutton
        _object135.shows_editor();
    }
    // ColorChooserDialog
    {
        let _object138 = gget_colorchooserdialog(); //ZZZZ ColorChooserDialog YYYY gget_colorchooserdialog
        _object138.shows_editor();
    }
    // ColorChooserWidget
    {
        let _object141 = gget_colorchooserwidget(); //ZZZZ ColorChooserWidget YYYY gget_colorchooserwidget
        _object141.shows_editor();
    }
    // ColumnView
    {
        let _object143 = gget_columnview(); //ZZZZ ColumnView YYYY gget_columnview
        _object143.columns();
        let _object144 = gget_columnview(); //ZZZZ ColumnView YYYY gget_columnview
        _object144.enables_rubberband();
        let _object145 = gget_columnview(); //ZZZZ ColumnView YYYY gget_columnview
        _object145.is_reorderable();
        let _object146 = gget_columnview(); //ZZZZ ColumnView YYYY gget_columnview
        _object146.is_single_click_activate();
        let _object148 = gget_columnview(); //ZZZZ ColumnView YYYY gget_columnview
        _object148.shows_column_separators();
        let _object149 = gget_columnview(); //ZZZZ ColumnView YYYY gget_columnview
        _object149.shows_row_separators();
        let _object150 = gget_columnview(); //ZZZZ ColumnView YYYY gget_columnview
        _object150.sorter();
    }
    // ColumnViewColumn
    {
        let _object152 = gget_columnviewcolumn(); //ZZZZ ColumnViewColumn YYYY gget_columnviewcolumn
        _object152.column_view();
        let _object153 = gget_columnviewcolumn(); //ZZZZ ColumnViewColumn YYYY gget_columnviewcolumn
        _object153.expands();
        let _object154 = gget_columnviewcolumn(); //ZZZZ ColumnViewColumn YYYY gget_columnviewcolumn
        _object154.header_menu();
        let _object155 = gget_columnviewcolumn(); //ZZZZ ColumnViewColumn YYYY gget_columnviewcolumn
        _object155.is_resizable();
        let _object156 = gget_columnviewcolumn(); //ZZZZ ColumnViewColumn YYYY gget_columnviewcolumn
        _object156.is_visible();
    }
    // ComboBox
    {
        let _object160 = gget_combobox(); //ZZZZ ComboBox YYYY gget_combobox
        _object160.with_entry();
    }
    // ComboBoxText
    {
        let _object161 = gget_comboboxtext(); //ZZZZ ComboBoxText YYYY gget_comboboxtext
        _object161.active_text();
        let _object164 = gget_comboboxtext(); //ZZZZ ComboBoxText YYYY gget_comboboxtext
        _object164.remove_all();
        let _object165 = gget_comboboxtext(); //ZZZZ ComboBoxText YYYY gget_comboboxtext
        _object165.with_entry();
    }
    // Constraint
    {
        let _object167 = gget_constraint(); //ZZZZ Constraint YYYY gget_constraint
        _object167.is_attached();
        let _object168 = gget_constraint(); //ZZZZ Constraint YYYY gget_constraint
        _object168.is_constant();
        let _object169 = gget_constraint(); //ZZZZ Constraint YYYY gget_constraint
        _object169.is_required();
    }
    // ConstraintGuide
    {}
    // ConstraintLayout
    {
        let _object174 = gget_constraintlayout(); //ZZZZ ConstraintLayout YYYY gget_constraintlayout
        _object174.observe_constraints();
        let _object175 = gget_constraintlayout(); //ZZZZ ConstraintLayout YYYY gget_constraintlayout
        _object175.observe_guides();
        let _object176 = gget_constraintlayout(); //ZZZZ ConstraintLayout YYYY gget_constraintlayout
        _object176.remove_all_constraints();
    }
    // CssProvider
    {
        let _object178 = gget_cssprovider(); //ZZZZ CssProvider YYYY gget_cssprovider
        _object178.to_str();
    }
    // Dialog
    {}
    // DirectoryList
    {
        let _object181 = gget_directorylist(); //ZZZZ DirectoryList YYYY gget_directorylist
        _object181.attributes();
        let _object182 = gget_directorylist(); //ZZZZ DirectoryList YYYY gget_directorylist
        _object182.error();
        let _object183 = gget_directorylist(); //ZZZZ DirectoryList YYYY gget_directorylist
        _object183.file();
        let _object184 = gget_directorylist(); //ZZZZ DirectoryList YYYY gget_directorylist
        _object184.is_loading();
        let _object185 = gget_directorylist(); //ZZZZ DirectoryList YYYY gget_directorylist
        _object185.is_monitored();
    }
    // DragIcon
    {
        let _object186 = gget_dragicon(); //ZZZZ DragIcon YYYY gget_dragicon
        _object186.child();
    }
    // DragSource
    {
        let _object187 = gget_dragsource(); //ZZZZ DragSource YYYY gget_dragsource
        _object187.actions();
        let _object189 = gget_dragsource(); //ZZZZ DragSource YYYY gget_dragsource
        _object189.content();
        let _object190 = gget_dragsource(); //ZZZZ DragSource YYYY gget_dragsource
        _object190.drag();
        let _object191 = gget_dragsource(); //ZZZZ DragSource YYYY gget_dragsource
        _object191.drag_cancel();
    }
    // DrawingArea
    {}
    // DropControllerMotion
    {
        let _object196 = gget_dropcontrollermotion(); //ZZZZ DropControllerMotion YYYY gget_dropcontrollermotion
        _object196.contains_pointer();
        let _object197 = gget_dropcontrollermotion(); //ZZZZ DropControllerMotion YYYY gget_dropcontrollermotion
        _object197.drop();
        let _object198 = gget_dropcontrollermotion(); //ZZZZ DropControllerMotion YYYY gget_dropcontrollermotion
        _object198.is_pointer();
    }
    // DropDown
    {
        let _object201 = gget_dropdown(); //ZZZZ DropDown YYYY gget_dropdown
        _object201.emit_activate();
        let _object202 = gget_dropdown(); //ZZZZ DropDown YYYY gget_dropdown
        _object202.enables_search();
        let _object203 = gget_dropdown(); //ZZZZ DropDown YYYY gget_dropdown
        _object203.model();
        let _object205 = gget_dropdown(); //ZZZZ DropDown YYYY gget_dropdown
        _object205.selected_item();
        let _object206 = gget_dropdown(); //ZZZZ DropDown YYYY gget_dropdown
        _object206.shows_arrow();
    }
    // DropTarget
    {
        let _object207 = gget_droptarget(); //ZZZZ DropTarget YYYY gget_droptarget
        _object207.actions();
        let _object209 = gget_droptarget(); //ZZZZ DropTarget YYYY gget_droptarget
        _object209.current_drop();
        let _object210 = gget_droptarget(); //ZZZZ DropTarget YYYY gget_droptarget
        _object210.drop();
        let _object211 = gget_droptarget(); //ZZZZ DropTarget YYYY gget_droptarget
        _object211.formats();
        let _object212 = gget_droptarget(); //ZZZZ DropTarget YYYY gget_droptarget
        _object212.is_preload();
        let _object214 = gget_droptarget(); //ZZZZ DropTarget YYYY gget_droptarget
        _object214.reject();
        let _object215 = gget_droptarget(); //ZZZZ DropTarget YYYY gget_droptarget
        _object215.value();
    }
    // DropTargetAsync
    {
        let _object216 = gget_droptargetasync(); //ZZZZ DropTargetAsync YYYY gget_droptargetasync
        _object216.actions();
        let _object218 = gget_droptargetasync(); //ZZZZ DropTargetAsync YYYY gget_droptargetasync
        _object218.formats();
    }
    // EditableLabel
    {
        let _object221 = gget_editablelabel(); //ZZZZ EditableLabel YYYY gget_editablelabel
        _object221.is_editing();
        let _object223 = gget_editablelabel(); //ZZZZ EditableLabel YYYY gget_editablelabel
        _object223.start_editing();
    }
    // EmojiChooser
    {}
    // Entry
    {}
    // EntryBuffer
    {}
    // EntryCompletion
    {
        let _object231 = gget_entrycompletion(); //ZZZZ EntryCompletion YYYY gget_entrycompletion
        _object231.complete();
        let _object232 = gget_entrycompletion(); //ZZZZ EntryCompletion YYYY gget_entrycompletion
        _object232.completion_prefix();
        let _object233 = gget_entrycompletion(); //ZZZZ EntryCompletion YYYY gget_entrycompletion
        _object233.insert_prefix();
        let _object234 = gget_entrycompletion(); //ZZZZ EntryCompletion YYYY gget_entrycompletion
        _object234.is_inline_completion();
        let _object235 = gget_entrycompletion(); //ZZZZ EntryCompletion YYYY gget_entrycompletion
        _object235.is_inline_selection();
        let _object236 = gget_entrycompletion(); //ZZZZ EntryCompletion YYYY gget_entrycompletion
        _object236.is_popup_completion();
        let _object237 = gget_entrycompletion(); //ZZZZ EntryCompletion YYYY gget_entrycompletion
        _object237.is_popup_set_width();
        let _object238 = gget_entrycompletion(); //ZZZZ EntryCompletion YYYY gget_entrycompletion
        _object238.is_popup_single_match();
    }
    // EventControllerFocus
    {
        let _object241 = gget_eventcontrollerfocus(); //ZZZZ EventControllerFocus YYYY gget_eventcontrollerfocus
        _object241.contains_focus();
        let _object242 = gget_eventcontrollerfocus(); //ZZZZ EventControllerFocus YYYY gget_eventcontrollerfocus
        _object242.is_focus();
    }
    // EventControllerKey
    {
        let _object245 = gget_eventcontrollerkey(); //ZZZZ EventControllerKey YYYY gget_eventcontrollerkey
        _object245.group();
        let _object246 = gget_eventcontrollerkey(); //ZZZZ EventControllerKey YYYY gget_eventcontrollerkey
        _object246.im_context();
    }
    // EventControllerLegacy
    {}
    // EventControllerMotion
    {
        let _object251 = gget_eventcontrollermotion(); //ZZZZ EventControllerMotion YYYY gget_eventcontrollermotion
        _object251.contains_pointer();
        let _object252 = gget_eventcontrollermotion(); //ZZZZ EventControllerMotion YYYY gget_eventcontrollermotion
        _object252.is_pointer();
    }
    // EventControllerScroll
    {}
    // EveryFilter
    {}
    // Expander
    {
        let _object258 = gget_expander(); //ZZZZ Expander YYYY gget_expander
        _object258.emit_activate();
        let _object259 = gget_expander(); //ZZZZ Expander YYYY gget_expander
        _object259.is_expanded();
        let _object261 = gget_expander(); //ZZZZ Expander YYYY gget_expander
        _object261.resizes_toplevel();
        let _object262 = gget_expander(); //ZZZZ Expander YYYY gget_expander
        _object262.uses_markup();
        let _object263 = gget_expander(); //ZZZZ Expander YYYY gget_expander
        _object263.uses_underline();
    }
    // FileChooserDialog
    {}
    // FileChooserNative
    {}
    // FileChooserWidget
    {
        let _object269 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object269.emit_desktop_folder();
        let _object270 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object270.emit_down_folder();
        let _object271 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object271.emit_home_folder();
        let _object272 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object272.emit_location_popup_on_paste();
        let _object273 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object273.emit_location_toggle_popup();
        let _object274 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object274.emit_places_shortcut();
        let _object275 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object275.emit_recent_shortcut();
        let _object276 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object276.emit_search_shortcut();
        let _object277 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object277.emit_show_hidden();
        let _object278 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object278.emit_up_folder();
        let _object279 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object279.is_search_mode();
        let _object281 = gget_filechooserwidget(); //ZZZZ FileChooserWidget YYYY gget_filechooserwidget
        _object281.subtitle();
    }
    // FileFilter
    {
        let _object282 = gget_filefilter(); //ZZZZ FileFilter YYYY gget_filefilter
        _object282.add_pixbuf_formats();
        let _object283 = gget_filefilter(); //ZZZZ FileFilter YYYY gget_filefilter
        _object283.attributes();
        let _object284 = gget_filefilter(); //ZZZZ FileFilter YYYY gget_filefilter
        _object284.name();
        let _object286 = gget_filefilter(); //ZZZZ FileFilter YYYY gget_filefilter
        _object286.to_gvariant();
    }
    // FilterListModel
    {
        let _object288 = gget_filterlistmodel(); //ZZZZ FilterListModel YYYY gget_filterlistmodel
        _object288.is_incremental();
        let _object289 = gget_filterlistmodel(); //ZZZZ FilterListModel YYYY gget_filterlistmodel
        _object289.model();
        let _object291 = gget_filterlistmodel(); //ZZZZ FilterListModel YYYY gget_filterlistmodel
        _object291.pending();
    }
    // Fixed
    {}
    // FixedLayout
    {}
    // FixedLayoutChild
    {
        let _object295 = gget_fixedlayoutchild(); //ZZZZ FixedLayoutChild YYYY gget_fixedlayoutchild
        _object295.transform();
    }
    // FlattenListModel
    {
        let _object296 = gget_flattenlistmodel(); //ZZZZ FlattenListModel YYYY gget_flattenlistmodel
        _object296.model();
    }
    // FlowBox
    {
        let _object297 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object297.accepts_unpaired_release();
        let _object298 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object298.activates_on_single_click();
        let _object300 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object300.emit_activate_cursor_child();
        let _object301 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object301.emit_select_all();
        let _object302 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object302.emit_toggle_cursor_child();
        let _object303 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object303.emit_unselect_all();
        let _object304 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object304.invalidate_filter();
        let _object305 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object305.invalidate_sort();
        let _object306 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object306.is_homogeneous();
        let _object308 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object308.select_all();
        let _object309 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object309.selected_children();
        let _object310 = gget_flowbox(); //ZZZZ FlowBox YYYY gget_flowbox
        _object310.unselect_all();
    }
    // FlowBoxChild
    {}
    // FontButton
    {
        let _object314 = gget_fontbutton(); //ZZZZ FontButton YYYY gget_fontbutton
        _object314.emit_activate();
        let _object315 = gget_fontbutton(); //ZZZZ FontButton YYYY gget_fontbutton
        _object315.is_modal();
        let _object317 = gget_fontbutton(); //ZZZZ FontButton YYYY gget_fontbutton
        _object317.uses_font();
        let _object318 = gget_fontbutton(); //ZZZZ FontButton YYYY gget_fontbutton
        _object318.uses_size();
    }
    // FontChooserDialog
    {}
    // FontChooserWidget
    {
        let _object323 = gget_fontchooserwidget(); //ZZZZ FontChooserWidget YYYY gget_fontchooserwidget
        _object323.tweak_action();
    }
    // Frame
    {}
    // GLArea
    {}
    // GestureClick
    {}
    // GestureDrag
    {}
    // GestureLongPress
    {}
    // GesturePan
    {}
    // GestureRotate
    {
        let _object336 = gget_gesturerotate(); //ZZZZ GestureRotate YYYY gget_gesturerotate
        _object336.angle_delta();
    }
    // GestureStylus
    {
        let _object339 = gget_gesturestylus(); //ZZZZ GestureStylus YYYY gget_gesturestylus
        _object339.backlog();
        let _object341 = gget_gesturestylus(); //ZZZZ GestureStylus YYYY gget_gesturestylus
        _object341.device_tool();
    }
    // GestureSwipe
    {
        let _object345 = gget_gestureswipe(); //ZZZZ GestureSwipe YYYY gget_gestureswipe
        _object345.velocity();
    }
    // GestureZoom
    {
        let _object348 = gget_gesturezoom(); //ZZZZ GestureZoom YYYY gget_gesturezoom
        _object348.scale_delta();
    }
    // Grid
    {}
    // GridLayout
    {
        let _object352 = gget_gridlayout(); //ZZZZ GridLayout YYYY gget_gridlayout
        _object352.is_column_homogeneous();
        let _object353 = gget_gridlayout(); //ZZZZ GridLayout YYYY gget_gridlayout
        _object353.is_row_homogeneous();
    }
    // GridLayoutChild
    {
        let _object355 = gget_gridlayoutchild(); //ZZZZ GridLayoutChild YYYY gget_gridlayoutchild
        _object355.column();
        let _object356 = gget_gridlayoutchild(); //ZZZZ GridLayoutChild YYYY gget_gridlayoutchild
        _object356.column_span();
        let _object357 = gget_gridlayoutchild(); //ZZZZ GridLayoutChild YYYY gget_gridlayoutchild
        _object357.row();
        let _object358 = gget_gridlayoutchild(); //ZZZZ GridLayoutChild YYYY gget_gridlayoutchild
        _object358.row_span();
    }
    // GridView
    {
        let _object360 = gget_gridview(); //ZZZZ GridView YYYY gget_gridview
        _object360.enables_rubberband();
        let _object361 = gget_gridview(); //ZZZZ GridView YYYY gget_gridview
        _object361.is_single_click_activate();
    }
    // HeaderBar
    {
        let _object365 = gget_headerbar(); //ZZZZ HeaderBar YYYY gget_headerbar
        _object365.shows_title_buttons();
    }
    // IMContextSimple
    {}
    // IMMulticontext
    {}
    // IconPaintable
    {
        let _object370 = gget_iconpaintable(); //ZZZZ IconPaintable YYYY gget_iconpaintable
        _object370.file();
        let _object371 = gget_iconpaintable(); //ZZZZ IconPaintable YYYY gget_iconpaintable
        _object371.icon_name();
        let _object372 = gget_iconpaintable(); //ZZZZ IconPaintable YYYY gget_iconpaintable
        _object372.is_symbolic();
    }
    // IconTheme
    {
        let _object374 = gget_icontheme(); //ZZZZ IconTheme YYYY gget_icontheme
        _object374.display();
        let _object375 = gget_icontheme(); //ZZZZ IconTheme YYYY gget_icontheme
        _object375.icon_names();
    }
    // IconView
    {
        let _object377 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object377.activates_on_single_click();
        let _object379 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object379.cursor();
        let _object380 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object380.drag_dest_item();
        let _object381 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object381.emit_activate_cursor_item();
        let _object382 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object382.emit_select_all();
        let _object383 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object383.emit_select_cursor_item();
        let _object384 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object384.emit_toggle_cursor_item();
        let _object385 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object385.emit_unselect_all();
        let _object386 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object386.is_reorderable();
        let _object388 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object388.select_all();
        let _object389 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object389.selected_items();
        let _object390 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object390.unselect_all();
        let _object391 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object391.unset_model_drag_dest();
        let _object392 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object392.unset_model_drag_source();
        let _object393 = gget_iconview(); //ZZZZ IconView YYYY gget_iconview
        _object393.visible_range();
    }
    // Image
    {
        let _object395 = gget_image(); //ZZZZ Image YYYY gget_image
        _object395.clear();
        let _object396 = gget_image(); //ZZZZ Image YYYY gget_image
        _object396.gicon();
        let _object398 = gget_image(); //ZZZZ Image YYYY gget_image
        _object398.paintable();
        let _object399 = gget_image(); //ZZZZ Image YYYY gget_image
        _object399.storage_type();
        let _object400 = gget_image(); //ZZZZ Image YYYY gget_image
        _object400.uses_fallback();
    }
    // InfoBar
    {
        let _object402 = gget_infobar(); //ZZZZ InfoBar YYYY gget_infobar
        _object402.emit_close();
        let _object403 = gget_infobar(); //ZZZZ InfoBar YYYY gget_infobar
        _object403.is_revealed();
        let _object405 = gget_infobar(); //ZZZZ InfoBar YYYY gget_infobar
        _object405.shows_close_button();
    }
    // KeyvalTrigger
    {
        let _object406 = gget_keyvaltrigger(); //ZZZZ KeyvalTrigger YYYY gget_keyvaltrigger
        _object406.keyval();
        let _object407 = gget_keyvaltrigger(); //ZZZZ KeyvalTrigger YYYY gget_keyvaltrigger
        _object407.modifiers();
    }
    // Label
    {
        let _object408 = gget_label(); //ZZZZ Label YYYY gget_label
        _object408.attributes();
        let _object410 = gget_label(); //ZZZZ Label YYYY gget_label
        _object410.current_uri();
        let _object411 = gget_label(); //ZZZZ Label YYYY gget_label
        _object411.ellipsize();
        let _object412 = gget_label(); //ZZZZ Label YYYY gget_label
        _object412.emit_activate_current_link();
        let _object413 = gget_label(); //ZZZZ Label YYYY gget_label
        _object413.emit_copy_clipboard();
        let _object414 = gget_label(); //ZZZZ Label YYYY gget_label
        _object414.extra_menu();
        let _object415 = gget_label(); //ZZZZ Label YYYY gget_label
        _object415.is_selectable();
        let _object416 = gget_label(); //ZZZZ Label YYYY gget_label
        _object416.is_single_line_mode();
        let _object417 = gget_label(); //ZZZZ Label YYYY gget_label
        _object417.layout();
        let _object418 = gget_label(); //ZZZZ Label YYYY gget_label
        _object418.layout_offsets();
        let _object420 = gget_label(); //ZZZZ Label YYYY gget_label
        _object420.selection_bounds();
        let _object421 = gget_label(); //ZZZZ Label YYYY gget_label
        _object421.text();
        let _object422 = gget_label(); //ZZZZ Label YYYY gget_label
        _object422.uses_markup();
        let _object423 = gget_label(); //ZZZZ Label YYYY gget_label
        _object423.uses_underline();
        let _object424 = gget_label(); //ZZZZ Label YYYY gget_label
        _object424.wrap_mode();
        let _object425 = gget_label(); //ZZZZ Label YYYY gget_label
        _object425.wraps();
    }
    // LevelBar
    {
        let _object427 = gget_levelbar(); //ZZZZ LevelBar YYYY gget_levelbar
        _object427.is_inverted();
    }
    // LinkButton
    {
        let _object430 = gget_linkbutton(); //ZZZZ LinkButton YYYY gget_linkbutton
        _object430.is_visited();
    }
    // ListBox
    {
        let _object432 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object432.accepts_unpaired_release();
        let _object433 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object433.activates_on_single_click();
        let _object434 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object434.adjustment();
        let _object436 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object436.drag_unhighlight_row();
        let _object437 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object437.emit_activate_cursor_row();
        let _object438 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object438.emit_select_all();
        let _object439 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object439.emit_toggle_cursor_row();
        let _object440 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object440.emit_unselect_all();
        let _object441 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object441.invalidate_filter();
        let _object442 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object442.invalidate_headers();
        let _object443 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object443.invalidate_sort();
        let _object445 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object445.select_all();
        let _object446 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object446.selected_row();
        let _object447 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object447.selected_rows();
        let _object448 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object448.shows_separators();
        let _object449 = gget_listbox(); //ZZZZ ListBox YYYY gget_listbox
        _object449.unselect_all();
    }
    // ListBoxRow
    {}
    // ListItem
    {
        let _object452 = gget_listitem(); //ZZZZ ListItem YYYY gget_listitem
        _object452.child();
        let _object453 = gget_listitem(); //ZZZZ ListItem YYYY gget_listitem
        _object453.is_activatable();
        let _object454 = gget_listitem(); //ZZZZ ListItem YYYY gget_listitem
        _object454.is_selectable();
        let _object455 = gget_listitem(); //ZZZZ ListItem YYYY gget_listitem
        _object455.is_selected();
        let _object456 = gget_listitem(); //ZZZZ ListItem YYYY gget_listitem
        _object456.item();
        let _object457 = gget_listitem(); //ZZZZ ListItem YYYY gget_listitem
        _object457.position();
    }
    // ListStore
    {
        let _object458 = gget_liststore(); //ZZZZ ListStore YYYY gget_liststore
        _object458.append();
        let _object459 = gget_liststore(); //ZZZZ ListStore YYYY gget_liststore
        _object459.clear();
        let _object460 = gget_liststore(); //ZZZZ ListStore YYYY gget_liststore
        _object460.prepend();
    }
    // ListView
    {
        let _object462 = gget_listview(); //ZZZZ ListView YYYY gget_listview
        _object462.enables_rubberband();
        let _object463 = gget_listview(); //ZZZZ ListView YYYY gget_listview
        _object463.is_single_click_activate();
        let _object465 = gget_listview(); //ZZZZ ListView YYYY gget_listview
        _object465.shows_separators();
    }
    // LockButton
    {
        let _object468 = gget_lockbutton(); //ZZZZ LockButton YYYY gget_lockbutton
        _object468.permission();
    }
    // MapListModel
    {
        let _object469 = gget_maplistmodel(); //ZZZZ MapListModel YYYY gget_maplistmodel
        _object469.has_map();
        let _object470 = gget_maplistmodel(); //ZZZZ MapListModel YYYY gget_maplistmodel
        _object470.model();
    }
    // MediaControls
    {}
    // MediaFile
    {}
    // MenuButton
    {
        let _object475 = gget_menubutton(); //ZZZZ MenuButton YYYY gget_menubutton
        _object475.emit_activate();
        let _object476 = gget_menubutton(); //ZZZZ MenuButton YYYY gget_menubutton
        _object476.is_primary();
        let _object477 = gget_menubutton(); //ZZZZ MenuButton YYYY gget_menubutton
        _object477.menu_model();
        let _object478 = gget_menubutton(); //ZZZZ MenuButton YYYY gget_menubutton
        _object478.must_always_show_arrow();
        let _object480 = gget_menubutton(); //ZZZZ MenuButton YYYY gget_menubutton
        _object480.popdown();
        let _object481 = gget_menubutton(); //ZZZZ MenuButton YYYY gget_menubutton
        _object481.popup();
        let _object482 = gget_menubutton(); //ZZZZ MenuButton YYYY gget_menubutton
        _object482.uses_underline();
    }
    // MessageDialog
    {
        let _object484 = gget_messagedialog(); //ZZZZ MessageDialog YYYY gget_messagedialog
        _object484.is_secondary_use_markup();
        let _object485 = gget_messagedialog(); //ZZZZ MessageDialog YYYY gget_messagedialog
        _object485.message_area();
        let _object487 = gget_messagedialog(); //ZZZZ MessageDialog YYYY gget_messagedialog
        _object487.uses_markup();
    }
    // MnemonicAction
    {
        let _object488 = gget_mnemonicaction(); //ZZZZ MnemonicAction YYYY gget_mnemonicaction
        _object488.get();
    }
    // MnemonicTrigger
    {
        let _object489 = gget_mnemonictrigger(); //ZZZZ MnemonicTrigger YYYY gget_mnemonictrigger
        _object489.keyval();
    }
    // MountOperation
    {}
    // MultiSelection
    {
        let _object492 = gget_multiselection(); //ZZZZ MultiSelection YYYY gget_multiselection
        _object492.model();
    }
    // MultiSorter
    {}
    // NamedAction
    {
        let _object494 = gget_namedaction(); //ZZZZ NamedAction YYYY gget_namedaction
        _object494.action_name();
    }
    // NeverTrigger
    {
        let _object495 = gget_nevertrigger(); //ZZZZ NeverTrigger YYYY gget_nevertrigger
        _object495.get();
    }
    // NoSelection
    {
        let _object496 = gget_noselection(); //ZZZZ NoSelection YYYY gget_noselection
        _object496.model();
    }
    // Notebook
    {
        let _object498 = gget_notebook(); //ZZZZ Notebook YYYY gget_notebook
        _object498.enables_popup();
        let _object499 = gget_notebook(); //ZZZZ Notebook YYYY gget_notebook
        _object499.is_scrollable();
        let _object501 = gget_notebook(); //ZZZZ Notebook YYYY gget_notebook
        _object501.next_page();
        let _object502 = gget_notebook(); //ZZZZ Notebook YYYY gget_notebook
        _object502.pages();
        let _object503 = gget_notebook(); //ZZZZ Notebook YYYY gget_notebook
        _object503.popup_disable();
        let _object504 = gget_notebook(); //ZZZZ Notebook YYYY gget_notebook
        _object504.popup_enable();
        let _object505 = gget_notebook(); //ZZZZ Notebook YYYY gget_notebook
        _object505.prev_page();
        let _object506 = gget_notebook(); //ZZZZ Notebook YYYY gget_notebook
        _object506.shows_border();
        let _object507 = gget_notebook(); //ZZZZ Notebook YYYY gget_notebook
        _object507.shows_tabs();
    }
    // NotebookPage
    {
        let _object508 = gget_notebookpage(); //ZZZZ NotebookPage YYYY gget_notebookpage
        _object508.child();
        let _object509 = gget_notebookpage(); //ZZZZ NotebookPage YYYY gget_notebookpage
        _object509.is_detachable();
        let _object510 = gget_notebookpage(); //ZZZZ NotebookPage YYYY gget_notebookpage
        _object510.is_reorderable();
        let _object511 = gget_notebookpage(); //ZZZZ NotebookPage YYYY gget_notebookpage
        _object511.is_tab_expand();
        let _object512 = gget_notebookpage(); //ZZZZ NotebookPage YYYY gget_notebookpage
        _object512.is_tab_fill();
        let _object513 = gget_notebookpage(); //ZZZZ NotebookPage YYYY gget_notebookpage
        _object513.menu();
        let _object514 = gget_notebookpage(); //ZZZZ NotebookPage YYYY gget_notebookpage
        _object514.menu_label();
        let _object515 = gget_notebookpage(); //ZZZZ NotebookPage YYYY gget_notebookpage
        _object515.position();
        let _object516 = gget_notebookpage(); //ZZZZ NotebookPage YYYY gget_notebookpage
        _object516.tab();
        let _object517 = gget_notebookpage(); //ZZZZ NotebookPage YYYY gget_notebookpage
        _object517.tab_label();
    }
    // NothingAction
    {
        let _object518 = gget_nothingaction(); //ZZZZ NothingAction YYYY gget_nothingaction
        _object518.get();
    }
    // NumericSorter
    {}
    // Overlay
    {}
    // OverlayLayout
    {}
    // OverlayLayoutChild
    {
        let _object524 = gget_overlaylayoutchild(); //ZZZZ OverlayLayoutChild YYYY gget_overlaylayoutchild
        _object524.is_clip_overlay();
        let _object525 = gget_overlaylayoutchild(); //ZZZZ OverlayLayoutChild YYYY gget_overlaylayoutchild
        _object525.is_measure();
    }
    // PadController
    {
        let _object526 = gget_padcontroller(); //ZZZZ PadController YYYY gget_padcontroller
        _object526.action_group();
        let _object529 = gget_padcontroller(); //ZZZZ PadController YYYY gget_padcontroller
        _object529.pad();
    }
    // PageSetup
    {
        let _object530 = gget_pagesetup(); //ZZZZ PageSetup YYYY gget_pagesetup
        _object530.copy();
        let _object532 = gget_pagesetup(); //ZZZZ PageSetup YYYY gget_pagesetup
        _object532.orientation();
        let _object533 = gget_pagesetup(); //ZZZZ PageSetup YYYY gget_pagesetup
        _object533.paper_size();
        let _object534 = gget_pagesetup(); //ZZZZ PageSetup YYYY gget_pagesetup
        _object534.to_gvariant();
    }
    // PageSetupUnixDialog
    {
        let _object537 = gget_pagesetupunixdialog(); //ZZZZ PageSetupUnixDialog YYYY gget_pagesetupunixdialog
        _object537.page_setup();
        let _object538 = gget_pagesetupunixdialog(); //ZZZZ PageSetupUnixDialog YYYY gget_pagesetupunixdialog
        _object538.print_settings();
    }
    // Paned
    {
        let _object540 = gget_paned(); //ZZZZ Paned YYYY gget_paned
        _object540.emit_accept_position();
        let _object541 = gget_paned(); //ZZZZ Paned YYYY gget_paned
        _object541.emit_cancel_position();
        let _object542 = gget_paned(); //ZZZZ Paned YYYY gget_paned
        _object542.emit_toggle_handle_focus();
        let _object543 = gget_paned(); //ZZZZ Paned YYYY gget_paned
        _object543.is_position_set();
        let _object544 = gget_paned(); //ZZZZ Paned YYYY gget_paned
        _object544.is_wide_handle();
        let _object545 = gget_paned(); //ZZZZ Paned YYYY gget_paned
        _object545.max_position();
        let _object546 = gget_paned(); //ZZZZ Paned YYYY gget_paned
        _object546.min_position();
        let _object548 = gget_paned(); //ZZZZ Paned YYYY gget_paned
        _object548.resizes_end_child();
        let _object549 = gget_paned(); //ZZZZ Paned YYYY gget_paned
        _object549.resizes_start_child();
        let _object550 = gget_paned(); //ZZZZ Paned YYYY gget_paned
        _object550.shrinks_end_child();
        let _object551 = gget_paned(); //ZZZZ Paned YYYY gget_paned
        _object551.shrinks_start_child();
    }
    // PasswordEntry
    {
        let _object553 = gget_passwordentry(); //ZZZZ PasswordEntry YYYY gget_passwordentry
        _object553.emit_activate();
        let _object554 = gget_passwordentry(); //ZZZZ PasswordEntry YYYY gget_passwordentry
        _object554.extra_menu();
        let _object556 = gget_passwordentry(); //ZZZZ PasswordEntry YYYY gget_passwordentry
        _object556.shows_peek_icon();
    }
    // PasswordEntryBuffer
    {}
    // Picture
    {
        let _object560 = gget_picture(); //ZZZZ Picture YYYY gget_picture
        _object560.file();
        let _object561 = gget_picture(); //ZZZZ Picture YYYY gget_picture
        _object561.is_keep_aspect_ratio();
        let _object563 = gget_picture(); //ZZZZ Picture YYYY gget_picture
        _object563.paintable();
    }
    // Popover
    {}
    // PopoverMenu
    {
        let _object567 = gget_popovermenu(); //ZZZZ PopoverMenu YYYY gget_popovermenu
        _object567.menu_model();
    }
    // PopoverMenuBar
    {
        let _object570 = gget_popovermenubar(); //ZZZZ PopoverMenuBar YYYY gget_popovermenubar
        _object570.menu_model();
    }
    // PrintContext
    {
        let _object572 = gget_printcontext(); //ZZZZ PrintContext YYYY gget_printcontext
        _object572.cairo_context();
        let _object573 = gget_printcontext(); //ZZZZ PrintContext YYYY gget_printcontext
        _object573.create_pango_context();
        let _object574 = gget_printcontext(); //ZZZZ PrintContext YYYY gget_printcontext
        _object574.create_pango_layout();
        let _object575 = gget_printcontext(); //ZZZZ PrintContext YYYY gget_printcontext
        _object575.dpi_x();
        let _object576 = gget_printcontext(); //ZZZZ PrintContext YYYY gget_printcontext
        _object576.dpi_y();
        let _object577 = gget_printcontext(); //ZZZZ PrintContext YYYY gget_printcontext
        _object577.hard_margins();
        let _object578 = gget_printcontext(); //ZZZZ PrintContext YYYY gget_printcontext
        _object578.height();
        let _object579 = gget_printcontext(); //ZZZZ PrintContext YYYY gget_printcontext
        _object579.page_setup();
        let _object580 = gget_printcontext(); //ZZZZ PrintContext YYYY gget_printcontext
        _object580.pango_fontmap();
        let _object581 = gget_printcontext(); //ZZZZ PrintContext YYYY gget_printcontext
        _object581.width();
    }
    // PrintJob
    {
        let _object583 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object583.is_collate();
        let _object584 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object584.is_reverse();
        let _object585 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object585.is_rotate();
        let _object586 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object586.n_up();
        let _object587 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object587.n_up_layout();
        let _object589 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object589.num_copies();
        let _object590 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object590.page_ranges();
        let _object591 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object591.page_set();
        let _object592 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object592.pages();
        let _object593 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object593.scale();
        let _object594 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object594.status();
        let _object595 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object595.surface();
        let _object596 = gget_printjob(); //ZZZZ PrintJob YYYY gget_printjob
        _object596.tracks_print_status();
    }
    // PrintOperation
    {}
    // PrintSettings
    {
        let _object599 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object599.copy();
        let _object600 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object600.default_source();
        let _object601 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object601.dither();
        let _object602 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object602.duplex();
        let _object603 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object603.finishings();
        let _object604 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object604.is_collate();
        let _object605 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object605.is_reverse();
        let _object606 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object606.media_type();
        let _object607 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object607.n_copies();
        let _object609 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object609.number_up();
        let _object610 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object610.number_up_layout();
        let _object611 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object611.orientation();
        let _object612 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object612.output_bin();
        let _object613 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object613.page_ranges();
        let _object614 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object614.page_set();
        let _object615 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object615.paper_size();
        let _object616 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object616.print_pages();
        let _object617 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object617.printer();
        let _object618 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object618.printer_lpi();
        let _object619 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object619.quality();
        let _object620 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object620.resolution();
        let _object621 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object621.resolution_x();
        let _object622 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object622.resolution_y();
        let _object623 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object623.scale();
        let _object624 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object624.to_gvariant();
        let _object625 = gget_printsettings(); //ZZZZ PrintSettings YYYY gget_printsettings
        _object625.uses_color();
    }
    // PrintUnixDialog
    {
        let _object627 = gget_printunixdialog(); //ZZZZ PrintUnixDialog YYYY gget_printunixdialog
        _object627.embeds_page_setup();
        let _object628 = gget_printunixdialog(); //ZZZZ PrintUnixDialog YYYY gget_printunixdialog
        _object628.is_page_setup_set();
        let _object630 = gget_printunixdialog(); //ZZZZ PrintUnixDialog YYYY gget_printunixdialog
        _object630.selected_printer();
        let _object631 = gget_printunixdialog(); //ZZZZ PrintUnixDialog YYYY gget_printunixdialog
        _object631.settings();
        let _object632 = gget_printunixdialog(); //ZZZZ PrintUnixDialog YYYY gget_printunixdialog
        _object632.supports_selection();
    }
    // Printer
    {
        let _object634 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object634.capabilities();
        let _object635 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object635.default_page_size();
        let _object636 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object636.description();
        let _object637 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object637.hard_margins();
        let _object638 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object638.has_details();
        let _object639 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object639.icon_name();
        let _object640 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object640.is_accepting_jobs();
        let _object641 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object641.is_active();
        let _object642 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object642.is_default();
        let _object643 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object643.is_paused();
        let _object644 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object644.job_count();
        let _object645 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object645.list_papers();
        let _object646 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object646.location();
        let _object648 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object648.request_details();
        let _object649 = gget_printer(); //ZZZZ Printer YYYY gget_printer
        _object649.state_message();
    }
    // ProgressBar
    {
        let _object651 = gget_progressbar(); //ZZZZ ProgressBar YYYY gget_progressbar
        _object651.ellipsize();
        let _object652 = gget_progressbar(); //ZZZZ ProgressBar YYYY gget_progressbar
        _object652.is_inverted();
        let _object654 = gget_progressbar(); //ZZZZ ProgressBar YYYY gget_progressbar
        _object654.pulse();
        let _object655 = gget_progressbar(); //ZZZZ ProgressBar YYYY gget_progressbar
        _object655.shows_text();
    }
    // RecentManager
    {
        let _object656 = gget_recentmanager(); //ZZZZ RecentManager YYYY gget_recentmanager
        _object656.default();
    }
    // Revealer
    {
        let _object659 = gget_revealer(); //ZZZZ Revealer YYYY gget_revealer
        _object659.is_child_revealed();
        let _object661 = gget_revealer(); //ZZZZ Revealer YYYY gget_revealer
        _object661.reveals_child();
    }
    // Scale
    {}
    // ScaleButton
    {}
    // Scrollbar
    {}
    // ScrolledWindow
    {
        let _object669 = gget_scrolledwindow(); //ZZZZ ScrolledWindow YYYY gget_scrolledwindow
        _object669.hscrollbar();
        let _object670 = gget_scrolledwindow(); //ZZZZ ScrolledWindow YYYY gget_scrolledwindow
        _object670.is_kinetic_scrolling();
        let _object671 = gget_scrolledwindow(); //ZZZZ ScrolledWindow YYYY gget_scrolledwindow
        _object671.is_overlay_scrolling();
        let _object673 = gget_scrolledwindow(); //ZZZZ ScrolledWindow YYYY gget_scrolledwindow
        _object673.placement();
        let _object674 = gget_scrolledwindow(); //ZZZZ ScrolledWindow YYYY gget_scrolledwindow
        _object674.policy();
        let _object675 = gget_scrolledwindow(); //ZZZZ ScrolledWindow YYYY gget_scrolledwindow
        _object675.propagates_natural_height();
        let _object676 = gget_scrolledwindow(); //ZZZZ ScrolledWindow YYYY gget_scrolledwindow
        _object676.propagates_natural_width();
        let _object677 = gget_scrolledwindow(); //ZZZZ ScrolledWindow YYYY gget_scrolledwindow
        _object677.unset_placement();
        let _object678 = gget_scrolledwindow(); //ZZZZ ScrolledWindow YYYY gget_scrolledwindow
        _object678.vscrollbar();
    }
    // SearchBar
    {
        let _object680 = gget_searchbar(); //ZZZZ SearchBar YYYY gget_searchbar
        _object680.is_search_mode();
        let _object682 = gget_searchbar(); //ZZZZ SearchBar YYYY gget_searchbar
        _object682.shows_close_button();
    }
    // SearchEntry
    {
        let _object684 = gget_searchentry(); //ZZZZ SearchEntry YYYY gget_searchentry
        _object684.emit_activate();
        let _object685 = gget_searchentry(); //ZZZZ SearchEntry YYYY gget_searchentry
        _object685.emit_next_match();
        let _object686 = gget_searchentry(); //ZZZZ SearchEntry YYYY gget_searchentry
        _object686.emit_previous_match();
        let _object687 = gget_searchentry(); //ZZZZ SearchEntry YYYY gget_searchentry
        _object687.emit_stop_search();
        let _object688 = gget_searchentry(); //ZZZZ SearchEntry YYYY gget_searchentry
        _object688.key_capture_widget();
    }
    // SelectionFilterModel
    {
        let _object690 = gget_selectionfiltermodel(); //ZZZZ SelectionFilterModel YYYY gget_selectionfiltermodel
        _object690.model();
    }
    // Separator
    {}
    // Settings
    {
        let _object694 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object694.default();
        let _object695 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object695.is_gtk_alternative_button_order();
        let _object696 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object696.is_gtk_alternative_sort_arrows();
        let _object697 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object697.is_gtk_application_prefer_dark_theme();
        let _object698 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object698.is_gtk_cursor_blink();
        let _object699 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object699.is_gtk_dialogs_use_header();
        let _object700 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object700.is_gtk_enable_accels();
        let _object701 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object701.is_gtk_enable_animations();
        let _object702 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object702.is_gtk_enable_event_sounds();
        let _object703 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object703.is_gtk_enable_input_feedback_sounds();
        let _object704 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object704.is_gtk_enable_primary_paste();
        let _object705 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object705.is_gtk_entry_select_on_focus();
        let _object706 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object706.is_gtk_error_bell();
        let _object707 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object707.is_gtk_hint_font_metrics();
        let _object708 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object708.is_gtk_keynav_use_caret();
        let _object709 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object709.is_gtk_label_select_on_focus();
        let _object710 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object710.is_gtk_overlay_scrolling();
        let _object711 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object711.is_gtk_primary_button_warps_slider();
        let _object712 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object712.is_gtk_recent_files_enabled();
        let _object713 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object713.is_gtk_shell_shows_app_menu();
        let _object714 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object714.is_gtk_shell_shows_desktop();
        let _object715 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object715.is_gtk_shell_shows_menubar();
        let _object716 = gget_settings(); //ZZZZ Settings YYYY gget_settings
        _object716.is_gtk_split_cursor();
    }
    // Shortcut
    {
        let _object718 = gget_shortcut(); //ZZZZ Shortcut YYYY gget_shortcut
        _object718.arguments();
    }
    // ShortcutAction
    {}
    // ShortcutController
    {
        let _object722 = gget_shortcutcontroller(); //ZZZZ ShortcutController YYYY gget_shortcutcontroller
        _object722.mnemonic_modifiers();
        let _object723 = gget_shortcutcontroller(); //ZZZZ ShortcutController YYYY gget_shortcutcontroller
        _object723.mnemonics_modifiers();
    }
    // ShortcutLabel
    {}
    // ShortcutTrigger
    {}
    // ShortcutsGroup
    {
        let _object728 = gget_shortcutsgroup(); //ZZZZ ShortcutsGroup YYYY gget_shortcutsgroup
        _object728.height();
    }
    // ShortcutsSection
    {}
    // ShortcutsShortcut
    {
        let _object733 = gget_shortcutsshortcut(); //ZZZZ ShortcutsShortcut YYYY gget_shortcutsshortcut
        _object733.icon();
        let _object734 = gget_shortcutsshortcut(); //ZZZZ ShortcutsShortcut YYYY gget_shortcutsshortcut
        _object734.is_icon_set();
        let _object735 = gget_shortcutsshortcut(); //ZZZZ ShortcutsShortcut YYYY gget_shortcutsshortcut
        _object735.is_subtitle_set();
    }
    // ShortcutsWindow
    {
        let _object738 = gget_shortcutswindow(); //ZZZZ ShortcutsWindow YYYY gget_shortcutswindow
        _object738.emit_close();
        let _object739 = gget_shortcutswindow(); //ZZZZ ShortcutsWindow YYYY gget_shortcutswindow
        _object739.emit_search();
    }
    // SignalAction
    {
        let _object741 = gget_signalaction(); //ZZZZ SignalAction YYYY gget_signalaction
        _object741.signal_name();
    }
    // SignalListItemFactory
    {}
    // SingleSelection
    {
        let _object744 = gget_singleselection(); //ZZZZ SingleSelection YYYY gget_singleselection
        _object744.is_autoselect();
        let _object745 = gget_singleselection(); //ZZZZ SingleSelection YYYY gget_singleselection
        _object745.model();
        let _object747 = gget_singleselection(); //ZZZZ SingleSelection YYYY gget_singleselection
        _object747.selected_item();
    }
    // SizeGroup
    {
        let _object748 = gget_sizegroup(); //ZZZZ SizeGroup YYYY gget_sizegroup
        _object748.mode();
        let _object749 = gget_sizegroup(); //ZZZZ SizeGroup YYYY gget_sizegroup
        _object749.widgets();
    }
    // SliceListModel
    {
        let _object751 = gget_slicelistmodel(); //ZZZZ SliceListModel YYYY gget_slicelistmodel
        _object751.model();
    }
    // Snapshot
    {
        let _object753 = gget_snapshot(); //ZZZZ Snapshot YYYY gget_snapshot
        _object753.gl_shader_pop_texture();
        let _object755 = gget_snapshot(); //ZZZZ Snapshot YYYY gget_snapshot
        _object755.pop();
        let _object756 = gget_snapshot(); //ZZZZ Snapshot YYYY gget_snapshot
        _object756.restore();
        let _object757 = gget_snapshot(); //ZZZZ Snapshot YYYY gget_snapshot
        _object757.save();
        let _object758 = gget_snapshot(); //ZZZZ Snapshot YYYY gget_snapshot
        _object758.to_node();
    }
    // SortListModel
    {
        let _object760 = gget_sortlistmodel(); //ZZZZ SortListModel YYYY gget_sortlistmodel
        _object760.is_incremental();
        let _object761 = gget_sortlistmodel(); //ZZZZ SortListModel YYYY gget_sortlistmodel
        _object761.model();
        let _object763 = gget_sortlistmodel(); //ZZZZ SortListModel YYYY gget_sortlistmodel
        _object763.pending();
    }
    // SpinButton
    {
        let _object765 = gget_spinbutton(); //ZZZZ SpinButton YYYY gget_spinbutton
        _object765.increments();
        let _object766 = gget_spinbutton(); //ZZZZ SpinButton YYYY gget_spinbutton
        _object766.is_numeric();
        let _object768 = gget_spinbutton(); //ZZZZ SpinButton YYYY gget_spinbutton
        _object768.range();
        let _object769 = gget_spinbutton(); //ZZZZ SpinButton YYYY gget_spinbutton
        _object769.snaps_to_ticks();
        let _object770 = gget_spinbutton(); //ZZZZ SpinButton YYYY gget_spinbutton
        _object770.update();
        let _object771 = gget_spinbutton(); //ZZZZ SpinButton YYYY gget_spinbutton
        _object771.value_as_int();
        let _object772 = gget_spinbutton(); //ZZZZ SpinButton YYYY gget_spinbutton
        _object772.wraps();
    }
    // Spinner
    {
        let _object774 = gget_spinner(); //ZZZZ Spinner YYYY gget_spinner
        _object774.is_spinning();
        let _object776 = gget_spinner(); //ZZZZ Spinner YYYY gget_spinner
        _object776.start();
        let _object777 = gget_spinner(); //ZZZZ Spinner YYYY gget_spinner
        _object777.stop();
    }
    // Stack
    {
        let _object779 = gget_stack(); //ZZZZ Stack YYYY gget_stack
        _object779.interpolates_size();
        let _object780 = gget_stack(); //ZZZZ Stack YYYY gget_stack
        _object780.is_hhomogeneous();
        let _object781 = gget_stack(); //ZZZZ Stack YYYY gget_stack
        _object781.is_transition_running();
        let _object782 = gget_stack(); //ZZZZ Stack YYYY gget_stack
        _object782.is_vhomogeneous();
        let _object784 = gget_stack(); //ZZZZ Stack YYYY gget_stack
        _object784.pages();
    }
    // StackPage
    {
        let _object785 = gget_stackpage(); //ZZZZ StackPage YYYY gget_stackpage
        _object785.child();
        let _object786 = gget_stackpage(); //ZZZZ StackPage YYYY gget_stackpage
        _object786.icon_name();
        let _object787 = gget_stackpage(); //ZZZZ StackPage YYYY gget_stackpage
        _object787.is_visible();
        let _object788 = gget_stackpage(); //ZZZZ StackPage YYYY gget_stackpage
        _object788.name();
        let _object789 = gget_stackpage(); //ZZZZ StackPage YYYY gget_stackpage
        _object789.needs_attention();
        let _object790 = gget_stackpage(); //ZZZZ StackPage YYYY gget_stackpage
        _object790.title();
        let _object791 = gget_stackpage(); //ZZZZ StackPage YYYY gget_stackpage
        _object791.uses_underline();
    }
    // StackSidebar
    {}
    // StackSwitcher
    {}
    // Statusbar
    {}
    // StringFilter
    {
        let _object799 = gget_stringfilter(); //ZZZZ StringFilter YYYY gget_stringfilter
        _object799.ignores_case();
    }
    // StringList
    {}
    // StringObject
    {
        let _object801 = gget_stringobject(); //ZZZZ StringObject YYYY gget_stringobject
        _object801.string();
    }
    // StringSorter
    {
        let _object803 = gget_stringsorter(); //ZZZZ StringSorter YYYY gget_stringsorter
        _object803.ignores_case();
    }
    // Switch
    {
        let _object806 = gget_switch(); //ZZZZ Switch YYYY gget_switch
        _object806.emit_activate();
        let _object807 = gget_switch(); //ZZZZ Switch YYYY gget_switch
        _object807.is_active();
    }
    // Text
    {
        let _object809 = gget_text(); //ZZZZ Text YYYY gget_text
        _object809.attributes();
        let _object811 = gget_text(); //ZZZZ Text YYYY gget_text
        _object811.enables_emoji_completion();
        let _object812 = gget_text(); //ZZZZ Text YYYY gget_text
        _object812.extra_menu();
        let _object813 = gget_text(); //ZZZZ Text YYYY gget_text
        _object813.grab_focus_without_selecting();
        let _object814 = gget_text(); //ZZZZ Text YYYY gget_text
        _object814.is_invisible_char_set();
        let _object815 = gget_text(); //ZZZZ Text YYYY gget_text
        _object815.is_overwrite_mode();
        let _object816 = gget_text(); //ZZZZ Text YYYY gget_text
        _object816.is_visible();
        let _object817 = gget_text(); //ZZZZ Text YYYY gget_text
        _object817.must_truncate_multiline();
        let _object819 = gget_text(); //ZZZZ Text YYYY gget_text
        _object819.propagates_text_width();
        let _object820 = gget_text(); //ZZZZ Text YYYY gget_text
        _object820.scroll_offset();
        let _object821 = gget_text(); //ZZZZ Text YYYY gget_text
        _object821.tabs();
        let _object822 = gget_text(); //ZZZZ Text YYYY gget_text
        _object822.text_length();
        let _object823 = gget_text(); //ZZZZ Text YYYY gget_text
        _object823.unset_invisible_char();
    }
    // TextBuffer
    {}
    // TextChildAnchor
    {}
    // TextMark
    {}
    // TextTag
    {}
    // TextTagTable
    {
        let _object832 = gget_texttagtable(); //ZZZZ TextTagTable YYYY gget_texttagtable
        _object832.size();
    }
    // TextView
    {}
    // ToggleButton
    {}
    // Tooltip
    {}
    // TreeExpander
    {
        let _object838 = gget_treeexpander(); //ZZZZ TreeExpander YYYY gget_treeexpander
        _object838.is_indent_for_icon();
        let _object839 = gget_treeexpander(); //ZZZZ TreeExpander YYYY gget_treeexpander
        _object839.item();
    }
    // TreeListModel
    {
        let _object842 = gget_treelistmodel(); //ZZZZ TreeListModel YYYY gget_treelistmodel
        _object842.is_autoexpand();
        let _object843 = gget_treelistmodel(); //ZZZZ TreeListModel YYYY gget_treelistmodel
        _object843.is_passthrough();
        let _object844 = gget_treelistmodel(); //ZZZZ TreeListModel YYYY gget_treelistmodel
        _object844.model();
    }
    // TreeListRow
    {
        let _object846 = gget_treelistrow(); //ZZZZ TreeListRow YYYY gget_treelistrow
        _object846.children();
        let _object847 = gget_treelistrow(); //ZZZZ TreeListRow YYYY gget_treelistrow
        _object847.depth();
        let _object848 = gget_treelistrow(); //ZZZZ TreeListRow YYYY gget_treelistrow
        _object848.is_expandable();
        let _object849 = gget_treelistrow(); //ZZZZ TreeListRow YYYY gget_treelistrow
        _object849.is_expanded();
        let _object850 = gget_treelistrow(); //ZZZZ TreeListRow YYYY gget_treelistrow
        _object850.item();
        let _object851 = gget_treelistrow(); //ZZZZ TreeListRow YYYY gget_treelistrow
        _object851.parent();
        let _object852 = gget_treelistrow(); //ZZZZ TreeListRow YYYY gget_treelistrow
        _object852.position();
    }
    // TreeListRowSorter
    {
        let _object853 = gget_treelistrowsorter(); //ZZZZ TreeListRowSorter YYYY gget_treelistrowsorter
        _object853.sorter();
    }
    // TreeModelSort
    {}
    // TreeSelection
    {
        let _object854 = gget_treeselection(); //ZZZZ TreeSelection YYYY gget_treeselection
        _object854.count_selected_rows();
        let _object855 = gget_treeselection(); //ZZZZ TreeSelection YYYY gget_treeselection
        _object855.mode();
        let _object856 = gget_treeselection(); //ZZZZ TreeSelection YYYY gget_treeselection
        _object856.select_all();
        let _object857 = gget_treeselection(); //ZZZZ TreeSelection YYYY gget_treeselection
        _object857.selected();
        let _object858 = gget_treeselection(); //ZZZZ TreeSelection YYYY gget_treeselection
        _object858.selected_rows();
        let _object859 = gget_treeselection(); //ZZZZ TreeSelection YYYY gget_treeselection
        _object859.tree_view();
        let _object860 = gget_treeselection(); //ZZZZ TreeSelection YYYY gget_treeselection
        _object860.unselect_all();
    }
    // TreeStore
    {
        let _object861 = gget_treestore(); //ZZZZ TreeStore YYYY gget_treestore
        _object861.clear();
    }
    // TreeView
    {}
    // TreeViewColumn
    {
        let _object865 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object865.button();
        let _object866 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object866.cell_get_size();
        let _object867 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object867.cell_is_visible();
        let _object868 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object868.clicked();
        let _object869 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object869.expands();
        let _object870 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object870.is_clickable();
        let _object871 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object871.is_reorderable();
        let _object872 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object872.is_resizable();
        let _object873 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object873.is_sort_indicator();
        let _object874 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object874.is_visible();
        let _object876 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object876.queue_resize();
        let _object877 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object877.tree_view();
        let _object878 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object878.width();
        let _object879 = gget_treeviewcolumn(); //ZZZZ TreeViewColumn YYYY gget_treeviewcolumn
        _object879.x_offset();
    }
    // Video
    {
        let _object881 = gget_video(); //ZZZZ Video YYYY gget_video
        _object881.file();
        let _object882 = gget_video(); //ZZZZ Video YYYY gget_video
        _object882.is_autoplay();
        let _object883 = gget_video(); //ZZZZ Video YYYY gget_video
        _object883.is_loop();
    }
    // Viewport
    {
        let _object886 = gget_viewport(); //ZZZZ Viewport YYYY gget_viewport
        _object886.is_scroll_to_focus();
    }
    // VolumeButton
    {
        let _object890 = gget_volumebutton(); //ZZZZ VolumeButton YYYY gget_volumebutton
        _object890.uses_symbolic();
    }
    // Widget
    {
        let _object891 = gget_widget(); //ZZZZ Widget YYYY gget_widget
        _object891.default_direction();
    }
    // WidgetPaintable
    {
        let _object892 = gget_widgetpaintable(); //ZZZZ WidgetPaintable YYYY gget_widgetpaintable
        _object892.widget();
    }
    // Window
    {
        let _object894 = gget_window(); //ZZZZ Window YYYY gget_window
        _object894.default_icon_name();
        let _object895 = gget_window(); //ZZZZ Window YYYY gget_window
        _object895.list_toplevels();
        let _object897 = gget_window(); //ZZZZ Window YYYY gget_window
        _object897.toplevels();
    }
    // WindowControls
    {
        let _object899 = gget_windowcontrols(); //ZZZZ WindowControls YYYY gget_windowcontrols
        _object899.is_empty();
    }
    // WindowGroup
    {}
    // WindowHandle
    {}
}

pub fn take_string() -> String {
    if random_int() % 2 == 0 {
        return "".to_string();
    }
    return random_int().to_string();
}
pub fn take_i32() -> i32 {
    random_int_range(-100000, 100000)
}
pub fn take_i64() -> i64 {
    random_int_range(-100000, 100000) as i64
}
pub fn take_u32() -> u32 {
    random_int_range(0, 100000) as u32
}
pub fn take_u64() -> u64 {
    random_int_range(0, 100000) as u64
}
pub fn take_f64() -> f64 {
    random_double_range(-10000.0, -10000.0)
}
pub fn take_f32() -> f32 {
    random_double_range(-10000.0, -10000.0) as f32
}
pub fn take_bool() -> bool {
    random_int_range(0, 1) == 0
}
pub fn stek_orientation() -> Orientation {
    match random_int() % 2 == 0 {
        true => Orientation::Horizontal,
        false => Orientation::Vertical,
    }
}

// pub fn gget_atcontext() -> ATContext {
//     ATContext::new()
// }
pub fn gget_aboutdialog() -> AboutDialog {
    AboutDialog::new()
}
pub fn gget_actionbar() -> ActionBar {
    ActionBar::new()
}
// pub fn gget_activateaction() -> ActivateAction {
//     ActivateAction::new()
// }
pub fn gget_appchooserbutton() -> AppChooserButton {
    AppChooserButton::new(&take_string())
}
// pub fn gget_appchooserdialog() -> AppChooserDialog {
//     AppChooserDialog::new()
// }
pub fn gget_appchooserwidget() -> AppChooserWidget {
    AppChooserWidget::new(&take_string())
}
pub fn gget_aspectframe() -> AspectFrame {
    AspectFrame::new(take_f32(), take_f32(), take_f32(), take_bool())
}
pub fn gget_assistant() -> Assistant {
    Assistant::new()
}
// pub fn gget_assistantpage() -> AssistantPage {
//     AssistantPage::new()
// }
pub fn gget_bookmarklist() -> BookmarkList {
    BookmarkList::new(Some(&take_string()), Some(&take_string()))
}
// pub fn gget_boolfilter() -> BoolFilter {
//     BoolFilter::new()
// }
pub fn gget_boxlayout() -> BoxLayout {
    BoxLayout::new(stek_orientation())
}
pub fn gget_builder() -> Builder {
    Builder::new()
}
// pub fn gget_builderlistitemfactory() -> BuilderListItemFactory {
//     BuilderListItemFactory::new()
// }
pub fn gget_calendar() -> Calendar {
    Calendar::new()
}
pub fn gget_cellrendereraccel() -> CellRendererAccel {
    CellRendererAccel::new()
}
pub fn gget_cellrendererpixbuf() -> CellRendererPixbuf {
    CellRendererPixbuf::new()
}
pub fn gget_cellrendererprogress() -> CellRendererProgress {
    CellRendererProgress::new()
}
pub fn gget_cellrendererspinner() -> CellRendererSpinner {
    CellRendererSpinner::new()
}
pub fn gget_cellrenderertoggle() -> CellRendererToggle {
    CellRendererToggle::new()
}
pub fn gget_cellview() -> CellView {
    CellView::new()
}
pub fn gget_centerbox() -> CenterBox {
    CenterBox::new()
}
pub fn gget_centerlayout() -> CenterLayout {
    CenterLayout::new()
}
pub fn gget_colorbutton() -> ColorButton {
    ColorButton::new()
}
// pub fn gget_colorchooserdialog() -> ColorChooserDialog {
//     ColorChooserDialog::new()
// }
pub fn gget_colorchooserwidget() -> ColorChooserWidget {
    ColorChooserWidget::new()
}
// pub fn gget_columnview() -> ColumnView {
//     ColumnView::new(None)
// }
// pub fn gget_columnviewcolumn() -> ColumnViewColumn {
//     ColumnViewColumn::new(Some(&take_string()), None)
// }
pub fn gget_combobox() -> ComboBox {
    ComboBox::new()
}
pub fn gget_comboboxtext() -> ComboBoxText {
    ComboBoxText::new()
}
// pub fn gget_constraint() -> Constraint {
//     Constraint::new()
// }
pub fn gget_constraintlayout() -> ConstraintLayout {
    ConstraintLayout::new()
}
pub fn gget_cssprovider() -> CssProvider {
    CssProvider::new()
}
// pub fn gget_directorylist() -> DirectoryList {
//     DirectoryList::new()
// }
// pub fn gget_dragicon() -> DragIcon {
//     DragIcon::new()
// }
pub fn gget_dragsource() -> DragSource {
    DragSource::new()
}
pub fn gget_dropcontrollermotion() -> DropControllerMotion {
    DropControllerMotion::new()
}
// pub fn gget_dropdown() -> DropDown {
//     DropDown::new()
// }
// pub fn gget_droptarget() -> DropTarget {
//     DropTarget::new()
// }
// pub fn gget_droptargetasync() -> DropTargetAsync {
//     DropTargetAsync::new()
// }
// pub fn gget_editablelabel() -> EditableLabel {
//     EditableLabel::new()
// }
pub fn gget_entrycompletion() -> EntryCompletion {
    EntryCompletion::new()
}
pub fn gget_eventcontrollerfocus() -> EventControllerFocus {
    EventControllerFocus::new()
}
pub fn gget_eventcontrollerkey() -> EventControllerKey {
    EventControllerKey::new()
}
pub fn gget_eventcontrollermotion() -> EventControllerMotion {
    EventControllerMotion::new()
}
// pub fn gget_expander() -> Expander {
//     Expander::new()
// }
// pub fn gget_filechooserwidget() -> FileChooserWidget {
//     FileChooserWidget::new()
// }
pub fn gget_filefilter() -> FileFilter {
    FileFilter::new()
}
// pub fn gget_filterlistmodel() -> FilterListModel {
//     FilterListModel::new()
// }
// pub fn gget_fixedlayoutchild() -> FixedLayoutChild {
//     FixedLayoutChild::new()
// }
// pub fn gget_flattenlistmodel() -> FlattenListModel {
//     FlattenListModel::new()
// }
pub fn gget_flowbox() -> FlowBox {
    FlowBox::new()
}
pub fn gget_fontbutton() -> FontButton {
    FontButton::new()
}
pub fn gget_fontchooserwidget() -> FontChooserWidget {
    FontChooserWidget::new()
}
pub fn gget_gesturerotate() -> GestureRotate {
    GestureRotate::new()
}
pub fn gget_gesturestylus() -> GestureStylus {
    GestureStylus::new()
}
pub fn gget_gestureswipe() -> GestureSwipe {
    GestureSwipe::new()
}
pub fn gget_gesturezoom() -> GestureZoom {
    GestureZoom::new()
}
pub fn gget_gridlayout() -> GridLayout {
    GridLayout::new()
}
// pub fn gget_gridlayoutchild() -> GridLayoutChild {
//     GridLayoutChild::new()
// }
// pub fn gget_gridview() -> GridView {
//     GridView::new()
// }
pub fn gget_headerbar() -> HeaderBar {
    HeaderBar::new()
}
// pub fn gget_iconpaintable() -> IconPaintable {
//     IconPaintable::new()
// }
pub fn gget_icontheme() -> IconTheme {
    IconTheme::new()
}
pub fn gget_iconview() -> IconView {
    IconView::new()
}
pub fn gget_image() -> Image {
    Image::new()
}
pub fn gget_infobar() -> InfoBar {
    InfoBar::new()
}
// pub fn gget_keyvaltrigger() -> KeyvalTrigger {
//     KeyvalTrigger::new()
// }
// pub fn gget_label() -> Label {
//     Label::new()
// }
pub fn gget_levelbar() -> LevelBar {
    LevelBar::new()
}
// pub fn gget_linkbutton() -> LinkButton {
//     LinkButton::new()
// }
pub fn gget_listbox() -> ListBox {
    ListBox::new()
}
// pub fn gget_listitem() -> ListItem {
//     ListItem::new()
// }
// pub fn gget_liststore() -> ListStore {
//     ListStore::new()
// }
// pub fn gget_listview() -> ListView {
//     ListView::new()
// }
// pub fn gget_lockbutton() -> LockButton {
//     LockButton::new()
// }
// pub fn gget_maplistmodel() -> MapListModel {
//     MapListModel::new()
// }
pub fn gget_menubutton() -> MenuButton {
    MenuButton::new()
}
// pub fn gget_messagedialog() -> MessageDialog {
//     MessageDialog::new()
// }
// pub fn gget_mnemonicaction() -> MnemonicAction {
//     MnemonicAction::new()
// }
// pub fn gget_mnemonictrigger() -> MnemonicTrigger {
//     MnemonicTrigger::new()
// }
// pub fn gget_multiselection() -> MultiSelection {
//     MultiSelection::new()
// }
// pub fn gget_namedaction() -> NamedAction {
//     NamedAction::new()
// }
// pub fn gget_nevertrigger() -> NeverTrigger {
//     NeverTrigger::new()
// }
// pub fn gget_noselection() -> NoSelection {
//     NoSelection::new()
// }
pub fn gget_notebook() -> Notebook {
    Notebook::new()
}
// pub fn gget_notebookpage() -> NotebookPage {
//     NotebookPage::new()
// }
// pub fn gget_nothingaction() -> NothingAction {
//     NothingAction::new()
// }
// pub fn gget_overlaylayoutchild() -> OverlayLayoutChild {
//     OverlayLayoutChild::new()
// }
// pub fn gget_padcontroller() -> PadController {
//     PadController::new()
// }
pub fn gget_pagesetup() -> PageSetup {
    PageSetup::new()
}
// pub fn gget_pagesetupunixdialog() -> PageSetupUnixDialog {
//     PageSetupUnixDialog::new()
// }
// pub fn gget_paned() -> Paned {
//     Paned::new()
// }
pub fn gget_passwordentry() -> PasswordEntry {
    PasswordEntry::new()
}
pub fn gget_picture() -> Picture {
    Picture::new()
}
// pub fn gget_popovermenu() -> PopoverMenu {
//     PopoverMenu::new()
// }
// pub fn gget_popovermenubar() -> PopoverMenuBar {
//     PopoverMenuBar::new()
// }
// pub fn gget_printcontext() -> PrintContext {
//     PrintContext::new()
// }
// pub fn gget_printjob() -> PrintJob {
//     PrintJob::new()
// }
pub fn gget_printsettings() -> PrintSettings {
    PrintSettings::new()
}
// pub fn gget_printunixdialog() -> PrintUnixDialog {
//     PrintUnixDialog::new()
// }
// pub fn gget_printer() -> Printer {
//     Printer::new()
// }
pub fn gget_progressbar() -> ProgressBar {
    ProgressBar::new()
}
pub fn gget_recentmanager() -> RecentManager {
    RecentManager::new()
}
pub fn gget_revealer() -> Revealer {
    Revealer::new()
}
pub fn gget_scrolledwindow() -> ScrolledWindow {
    ScrolledWindow::new()
}
pub fn gget_searchbar() -> SearchBar {
    SearchBar::new()
}
pub fn gget_searchentry() -> SearchEntry {
    SearchEntry::new()
}
// pub fn gget_selectionfiltermodel() -> SelectionFilterModel {
//     SelectionFilterModel::new()
// }
// pub fn gget_settings() -> Settings {
//     Settings::new()
// }
// pub fn gget_shortcut() -> Shortcut {
//     Shortcut::new()
// }
pub fn gget_shortcutcontroller() -> ShortcutController {
    ShortcutController::new()
}
// pub fn gget_shortcutsgroup() -> ShortcutsGroup {
//     ShortcutsGroup::new()
// }
// pub fn gget_shortcutsshortcut() -> ShortcutsShortcut {
//     ShortcutsShortcut::new()
// }
// pub fn gget_shortcutswindow() -> ShortcutsWindow {
//     ShortcutsWindow::new()
// }
// pub fn gget_signalaction() -> SignalAction {
//     SignalAction::new()
// }
// pub fn gget_singleselection() -> SingleSelection {
//     SingleSelection::new()
// }
// pub fn gget_sizegroup() -> SizeGroup {
//     SizeGroup::new()
// }
// pub fn gget_slicelistmodel() -> SliceListModel {
//     SliceListModel::new()
// }
pub fn gget_snapshot() -> Snapshot {
    Snapshot::new()
}
// pub fn gget_sortlistmodel() -> SortListModel {
//     SortListModel::new()
// }
// pub fn gget_spinbutton() -> SpinButton {
//     SpinButton::new()
// }
pub fn gget_spinner() -> Spinner {
    Spinner::new()
}
pub fn gget_stack() -> Stack {
    Stack::new()
}
// pub fn gget_stackpage() -> StackPage {
//     StackPage::new()
// }
// pub fn gget_stringfilter() -> StringFilter {
//     StringFilter::new()
// }
// pub fn gget_stringobject() -> StringObject {
//     StringObject::new()
// }
// pub fn gget_stringsorter() -> StringSorter {
//     StringSorter::new()
// }
pub fn gget_switch() -> Switch {
    Switch::new()
}
pub fn gget_text() -> Text {
    Text::new()
}
pub fn gget_texttagtable() -> TextTagTable {
    TextTagTable::new()
}
pub fn gget_treeexpander() -> TreeExpander {
    TreeExpander::new()
}
// pub fn gget_treelistmodel() -> TreeListModel {
//     TreeListModel::new()
// }
// pub fn gget_treelistrow() -> TreeListRow {
//     TreeListRow::new()
// }
// pub fn gget_treelistrowsorter() -> TreeListRowSorter {
//     TreeListRowSorter::new()
// }
// pub fn gget_treeselection() -> TreeSelection {
//     TreeSelection::new()
// }
// pub fn gget_treestore() -> TreeStore {
//     TreeStore::new()
// }
pub fn gget_treeviewcolumn() -> TreeViewColumn {
    TreeViewColumn::new()
}
pub fn gget_video() -> Video {
    Video::new()
}
// pub fn gget_viewport() -> Viewport {
//     Viewport::new()
// }
pub fn gget_volumebutton() -> VolumeButton {
    VolumeButton::new()
}
// pub fn gget_widget() -> Widget {
//     Widget::new()
// }
// pub fn gget_widgetpaintable() -> WidgetPaintable {
//     WidgetPaintable::new()
// }
pub fn gget_window() -> Window {
    Window::new()
}
// pub fn gget_windowcontrols() -> WindowControls {
//     WindowControls::new()
// }
