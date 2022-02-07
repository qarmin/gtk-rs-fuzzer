pub const BOTTOM_TEXT: &str = r#####"
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
"#####;
