use crate::helpers::*;
use gdk4::DragAction;
use gtk4::builders::{PrinterBuilder, SettingsBuilder};
use gtk4::prelude::*;
use gtk4::*;
use rand::{thread_rng, Rng};

// pub fn gget_atcontext() -> ATContext {
//     ATContext::new()
// }
pub fn gget_aboutdialog() -> AboutDialog {
    AboutDialog::new()
}
pub fn gget_actionbar() -> ActionBar {
    ActionBar::new()
}
pub fn gget_activateaction() -> ActivateAction {
    ActivateAction::get()
}
pub fn gget_appchooserbutton() -> AppChooserButton {
    AppChooserButton::new(&take_string())
}
pub fn gget_appchooserdialog() -> AppChooserDialog {
    AppChooserDialog::default()
}
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
pub fn gget_boolfilter() -> BoolFilter {
    let expression: Option<Expression> = None;
    BoolFilter::new(expression)
}
pub fn gget_boxlayout() -> BoxLayout {
    BoxLayout::new(stek_orientation())
}
pub fn gget_builder() -> Builder {
    Builder::new()
}
// pub fn gget_builderlistitemfactory() -> BuilderListItemFactory {
//     let builder_scope : Option<BuilderScope> = None;
//     BuilderListItemFactory::new(builder_scope,)
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
pub fn gget_colorchooserdialog() -> ColorChooserDialog {
    ColorChooserDialog::default()
}
pub fn gget_colorchooserwidget() -> ColorChooserWidget {
    ColorChooserWidget::new()
}
pub fn gget_columnview() -> ColumnView {
    let sm: Option<&SelectionModel> = None;
    ColumnView::new(sm)
}
pub fn gget_columnviewcolumn() -> ColumnViewColumn {
    let thing: Option<&ListItemFactory> = None;
    ColumnViewColumn::new(Some(&take_string()), thing)
}
pub fn gget_combobox() -> ComboBox {
    ComboBox::new()
}
pub fn gget_comboboxtext() -> ComboBoxText {
    ComboBoxText::new()
}
pub fn gget_constraint() -> Constraint {
    let thing: Option<&ConstraintTarget> = None;
    let thing2: ConstraintAttribute = ConstraintAttribute::None;
    let thing3: ConstraintRelation = ConstraintRelation::Eq;
    Constraint::new_constant(thing, thing2, thing3, take_f64(), take_i32())
}
pub fn gget_constraintlayout() -> ConstraintLayout {
    ConstraintLayout::new()
}
pub fn gget_cssprovider() -> CssProvider {
    CssProvider::new()
}
pub fn gget_directorylist() -> DirectoryList {
    let thing: Option<&gio::File> = None;
    DirectoryList::new(Some(&take_string()), thing)
}
// pub fn gget_dragicon() -> DragIcon {
//     DragIcon::new()
// }
pub fn gget_dragsource() -> DragSource {
    DragSource::new()
}
pub fn gget_dropcontrollermotion() -> DropControllerMotion {
    DropControllerMotion::new()
}
pub fn gget_dropdown() -> DropDown {
    let thing: Option<&Expression> = None;
    let thing2: Option<&gio::ListModel> = None;
    DropDown::new(thing2, thing)
}
pub fn gget_droptarget() -> DropTarget {
    DropTarget::new(stek_glib_type(), DragAction::COPY)
}
pub fn gget_droptargetasync() -> DropTargetAsync {
    DropTargetAsync::default()
}
pub fn gget_editablelabel() -> EditableLabel {
    EditableLabel::new(&take_string())
}
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
pub fn gget_expander() -> Expander {
    Expander::default()
}
pub fn gget_filechooserwidget() -> FileChooserWidget {
    FileChooserWidget::default()
}
pub fn gget_filefilter() -> FileFilter {
    FileFilter::new()
}
pub fn gget_filterlistmodel() -> FilterListModel {
    FilterListModel::default()
}
// pub fn gget_fixedlayoutchild() -> FixedLayoutChild {
//     FixedLayoutChild::default()
// }
pub fn gget_flattenlistmodel() -> FlattenListModel {
    let thing: Option<&gio::ListModel> = None;
    FlattenListModel::new(thing)
}
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
pub fn gget_gridview() -> GridView {
    GridView::default()
}
pub fn gget_headerbar() -> HeaderBar {
    HeaderBar::new()
}
pub fn gget_iconpaintable() -> IconPaintable {
    let thin: Option<String> = None;
    let thing = gio::File::new_tmp(thin).unwrap();
    IconPaintable::for_file(&thing.0, take_i32(), take_i32())
}
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
pub fn gget_keyvaltrigger() -> KeyvalTrigger {
    let thing = gdk::Key::from_name("f").unwrap();
    let thing2 = gdk::ModifierType::SHIFT_MASK;
    KeyvalTrigger::new(thing, thing2)
}
pub fn gget_label() -> Label {
    Label::default()
}
pub fn gget_levelbar() -> LevelBar {
    LevelBar::new()
}
pub fn gget_linkbutton() -> LinkButton {
    LinkButton::default()
}
pub fn gget_listbox() -> ListBox {
    ListBox::new()
}
// pub fn gget_listitem() -> ListItem {
//     ListItem::new()
// }
pub fn gget_liststore() -> ListStore {
    ListStore::new(&[stek_glib_type()])
}
pub fn gget_listview() -> ListView {
    ListView::default()
}
pub fn gget_lockbutton() -> LockButton {
    LockButton::default()
}
// pub fn gget_maplistmodel() -> MapListModel {
//     MapListModel::new()
// }
pub fn gget_menubutton() -> MenuButton {
    MenuButton::new()
}
pub fn gget_messagedialog() -> MessageDialog {
    MessageDialog::default()
}
pub fn gget_mnemonicaction() -> MnemonicAction {
    MnemonicAction::get()
}
pub fn gget_mnemonictrigger() -> MnemonicTrigger {
    MnemonicTrigger::new(gdk::Key::from_name("f").unwrap())
}
pub fn gget_multiselection() -> MultiSelection {
    let thing: Option<&gio::ListModel> = None;
    MultiSelection::new(thing)
}
pub fn gget_namedaction() -> NamedAction {
    NamedAction::new(&take_string())
}
pub fn gget_nevertrigger() -> NeverTrigger {
    NeverTrigger::get()
}
pub fn gget_noselection() -> NoSelection {
    let thing: Option<&gio::ListModel> = None;
    NoSelection::new(thing)
}
pub fn gget_notebook() -> Notebook {
    Notebook::default()
}
// pub fn gget_notebookpage() -> NotebookPage {
//     NotebookPage::new()
// }
pub fn gget_nothingaction() -> NothingAction {
    NothingAction::get()
}
// pub fn gget_overlaylayoutchild() -> OverlayLayoutChild {
//     OverlayLayoutChild::new()
// }
pub fn gget_padcontroller() -> PadController {
    PadController::default()
}
pub fn gget_pagesetup() -> PageSetup {
    PageSetup::new()
}
pub fn gget_pagesetupunixdialog() -> PageSetupUnixDialog {
    PageSetupUnixDialog::default()
}
pub fn gget_paned() -> Paned {
    Paned::default()
}
pub fn gget_passwordentry() -> PasswordEntry {
    PasswordEntry::new()
}
pub fn gget_picture() -> Picture {
    Picture::new()
}
pub fn gget_popovermenu() -> PopoverMenu {
    let thing: Option<&gio::MenuModel> = None;
    PopoverMenu::from_model(thing)
}
pub fn gget_popovermenubar() -> PopoverMenuBar {
    let thing: Option<&gio::MenuModel> = None;
    PopoverMenuBar::from_model(thing)
}
// pub fn gget_printcontext() -> PrintContext {
//     PrintContext::new()
// }
pub fn gget_printjob() -> PrintJob {
    PrintJob::default()
}
pub fn gget_printsettings() -> PrintSettings {
    PrintSettings::new()
}
pub fn gget_printunixdialog() -> PrintUnixDialog {
    PrintUnixDialog::default()
}
pub fn gget_printer() -> Printer {
    PrinterBuilder::build(Default::default())
}
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
pub fn gget_selectionfiltermodel() -> SelectionFilterModel {
    let thing: Option<&SelectionModel> = None;
    SelectionFilterModel::new(thing)
}
pub fn gget_settings() -> Settings {
    SettingsBuilder::build(Default::default())
}
pub fn gget_shortcut() -> Shortcut {
    Shortcut::default()
}
pub fn gget_shortcutcontroller() -> ShortcutController {
    ShortcutController::new()
}
pub fn gget_shortcutsgroup() -> ShortcutsGroup {
    ShortcutsGroup::builder().build()
}
pub fn gget_shortcutsshortcut() -> ShortcutsShortcut {
    ShortcutsShortcut::builder().build()
}
pub fn gget_shortcutswindow() -> ShortcutsWindow {
    ShortcutsWindow::builder().build()
}
pub fn gget_signalaction() -> SignalAction {
    SignalAction::new(&take_string())
}
pub fn gget_singleselection() -> SingleSelection {
    SingleSelection::default()
}
pub fn gget_sizegroup() -> SizeGroup {
    SizeGroup::new(SizeGroupMode::None) // TODO
}
pub fn gget_slicelistmodel() -> SliceListModel {
    SliceListModel::default()
}
pub fn gget_snapshot() -> Snapshot {
    Snapshot::default()
}
pub fn gget_sortlistmodel() -> SortListModel {
    SortListModel::default()
}
pub fn gget_spinbutton() -> SpinButton {
    SpinButton::default()
}
pub fn gget_spinner() -> Spinner {
    Spinner::new()
}
pub fn gget_stack() -> Stack {
    Stack::new()
}
// pub fn gget_stackpage() -> StackPage {
//     StackPage::new()
// }
pub fn gget_stringfilter() -> StringFilter {
    StringFilter::default()
}
pub fn gget_stringobject() -> StringObject {
    StringObject::new(&take_string())
}
pub fn gget_stringsorter() -> StringSorter {
    StringSorter::default()
}
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
pub fn gget_treelistmodel() -> TreeListModel {
    TreeListModel::default()
}
// pub fn gget_treelistrow() -> TreeListRow {
//     TreeListRow::new()
// }
pub fn gget_treelistrowsorter() -> TreeListRowSorter {
    let thing: Option<&Sorter> = None;
    TreeListRowSorter::new(thing)
}
// pub fn gget_treeselection() -> TreeSelection {
//     TreeSelection::def()
// }
pub fn gget_treestore() -> TreeStore {
    TreeStore::new(&[stek_glib_type()])
}
pub fn gget_treeviewcolumn() -> TreeViewColumn {
    TreeViewColumn::new()
}
pub fn gget_video() -> Video {
    Video::new()
}
pub fn gget_viewport() -> Viewport {
    Viewport::default()
}
pub fn gget_volumebutton() -> VolumeButton {
    VolumeButton::new()
}
// pub fn gget_widget() -> Widget {
//     Widget::new()
// }
pub fn gget_widgetpaintable() -> WidgetPaintable {
    let thing: Option<&Widget> = None;
    WidgetPaintable::new(thing)
}
pub fn gget_window() -> Window {
    Window::new()
}
pub fn gget_windowcontrols() -> WindowControls {
    WindowControls::default()
}
