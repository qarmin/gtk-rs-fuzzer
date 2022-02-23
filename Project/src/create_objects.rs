use crate::enum_things::*;
use crate::helpers::*;
use gdk4::DragAction;
use gtk4::builders::SettingsBuilder;
use gtk4::prelude::*;
use gtk4::*;
use rand::{thread_rng, Rng};

// pub fn gget_listbase() -> (ListBase, &'static str) {
//     (ListBase::new(), r###"AboutDialog::new()"###)
// }
// pub fn gget_range() -> (Range, &'static str) {
//     (Range::new(), r###"Range::new()"###)
// }
// pub fn gget_cellarea() -> (CellArea, &'static str) {
//     (CellArea::def(), r###"CellArea::new()"###)
// }
// pub fn gget_multifilter() -> (MultiFilter, &'static str) {
//     (MultiFilter::new(), r###"MultiFilter::new()"###)
// }
// pub fn gget_gesture() -> (Gesture, &'static str) {
//     (Gesture::def(), r###"Gesture::new()"###)
// }
// pub fn gget_gesturesingle() -> (GestureSingle, &'static str) {
//     (GestureSingle::def(), r###"GestureSingle::new()"###)
// }
// pub fn gget_constraintlayoutchild() -> (ConstraintLayoutChild, &'static str) {
//     (ConstraintLayoutChild::new(), r###"ConstraintLayoutChild::new()"###)
// }
// pub fn gget_callbackaction() -> (CallbackAction, &'static str) {
//     (CallbackAction::default(), r###"CallbackAction::default()"###)
// }
// pub fn gget_treemodelfilter() -> (TreeModelFilter, &'static str) {
//     (TreeModelFilter::default(), r###"TreeModelFilter::default()"###)
// }
// pub fn gget_dialogflags() -> (DialogFlags, &'static str) {
//     (DialogFlags::default(), r###"DialogFlags::default()"###)
// }
pub fn gget_customfilter() -> (CustomFilter, &'static str) {
    (CustomFilter::default(), r###"CustomFilter::default()"###)
}
pub fn gget_signallistitemfactory() -> (SignalListItemFactory, &'static str) {
    (SignalListItemFactory::default(), r###"SignalListItemFactory::default()"###)
}
pub fn gget_customsorter() -> (CustomSorter, &'static str) {
    (CustomSorter::default(), r###"CustomSorter::default()"###)
}
pub fn gget_eventcontrollerlegacy() -> (EventControllerLegacy, &'static str) {
    (EventControllerLegacy::new(), r###"EventControllerLegacy::new()"###)
}
pub fn gget_aboutdialog() -> (AboutDialog, &'static str) {
    (AboutDialog::new(), r###"AboutDialog::new()"###)
}
pub fn gget_actionbar() -> (ActionBar, &'static str) {
    (ActionBar::new(), r###"ActionBar::new()"###)
}
pub fn gget_activateaction() -> (ActivateAction, &'static str) {
    (ActivateAction::get(), r###"ActivateAction::get()"###)
}
pub fn gget_adjustment() -> (Adjustment, &'static str) {
    (Adjustment::default(), r###"Adjustment::default()"###)
}
pub fn gget_alternativetrigger() -> (AlternativeTrigger, &'static str) {
    (AlternativeTrigger::default(), r###"AlternativeTrigger::default()"###)
}
pub fn gget_anyfilter() -> (AnyFilter, &'static str) {
    (AnyFilter::default(), r###"AnyFilter::default()"###)
}
pub fn gget_appchooserbutton() -> (AppChooserButton, &'static str) {
    (AppChooserButton::new(&take_string()), r###"AppChooserButton::new(&take_string())"###)
}
pub fn gget_appchooserwidget() -> (AppChooserWidget, &'static str) {
    (AppChooserWidget::new(&take_string()), r###"AppChooserWidget::new(&take_string())"###)
}
pub fn gget_assistant() -> (Assistant, &'static str) {
    (Assistant::new(), r###"Assistant::new()"###)
}
pub fn gget_binlayout() -> (BinLayout, &'static str) {
    (BinLayout::new(), r###"BinLayout::new()"###)
}
pub fn gget_bitset() -> (Bitset, &'static str) {
    (Bitset::new_empty(), r###"Bitset::new_empty()"###)
}
pub fn gget_box() -> (Box, &'static str) {
    (Box::new(stek_orientation().0, take_i32()), r###"Box::new(stek_orientation().0, take_i32())"###)
}
pub fn gget_boxlayout() -> (BoxLayout, &'static str) {
    (BoxLayout::new(stek_orientation().0), r###"BoxLayout::new(stek_orientation().0)"###)
}
pub fn gget_builder() -> (Builder, &'static str) {
    (Builder::new(), r###"Builder::new()"###)
}
pub fn gget_button() -> (Button, &'static str) {
    (Button::new(), r###"Button::new()"###)
}
pub fn gget_calendar() -> (Calendar, &'static str) {
    (Calendar::new(), r###"Calendar::new()"###)
}
pub fn gget_cellareabox() -> (CellAreaBox, &'static str) {
    (CellAreaBox::new(), r###"CellAreaBox::new()"###)
}
pub fn gget_cellrendereraccel() -> (CellRendererAccel, &'static str) {
    (CellRendererAccel::new(), r###"CellRendererAccel::new()"###)
}
pub fn gget_cellrenderercombo() -> (CellRendererCombo, &'static str) {
    (CellRendererCombo::new(), r###"CellRendererCombo::new()"###)
}
pub fn gget_cellrendererpixbuf() -> (CellRendererPixbuf, &'static str) {
    (CellRendererPixbuf::new(), r###"CellRendererPixbuf::new()"###)
}
pub fn gget_cellrendererprogress() -> (CellRendererProgress, &'static str) {
    (CellRendererProgress::new(), r###"CellRendererProgress::new()"###)
}
pub fn gget_cellrendererspin() -> (CellRendererSpin, &'static str) {
    (CellRendererSpin::new(), r###"CellRendererSpin::new()"###)
}
pub fn gget_cellrendererspinner() -> (CellRendererSpinner, &'static str) {
    (CellRendererSpinner::new(), r###"CellRendererSpinner::new()"###)
}
pub fn gget_cellrenderertext() -> (CellRendererText, &'static str) {
    (CellRendererText::new(), r###"CellRendererText::new()"###)
}
pub fn gget_cellrenderertoggle() -> (CellRendererToggle, &'static str) {
    (CellRendererToggle::new(), r###"CellRendererToggle::new()"###)
}
pub fn gget_cellview() -> (CellView, &'static str) {
    (CellView::new(), r###"CellView::new()"###)
}
pub fn gget_centerbox() -> (CenterBox, &'static str) {
    (CenterBox::new(), r###"CenterBox::new()"###)
}
pub fn gget_centerlayout() -> (CenterLayout, &'static str) {
    (CenterLayout::new(), r###"CenterLayout::new()"###)
}
pub fn gget_checkbutton() -> (CheckButton, &'static str) {
    (CheckButton::new(), r###"CheckButton::new()"###)
}
pub fn gget_colorbutton() -> (ColorButton, &'static str) {
    (ColorButton::new(), r###"ColorButton::new()"###)
}
pub fn gget_colorchooserdialog() -> (ColorChooserDialog, &'static str) {
    (ColorChooserDialog::default(), r###"ColorChooserDialog::default()"###)
}
pub fn gget_colorchooserwidget() -> (ColorChooserWidget, &'static str) {
    (ColorChooserWidget::new(), r###"ColorChooserWidget::new()"###)
}
pub fn gget_combobox() -> (ComboBox, &'static str) {
    (ComboBox::new(), r###"ComboBox::new()"###)
}
pub fn gget_comboboxtext() -> (ComboBoxText, &'static str) {
    (ComboBoxText::new(), r###"ComboBoxText::new()"###)
}
pub fn gget_constraintguide() -> (ConstraintGuide, &'static str) {
    (ConstraintGuide::default(), r###"ConstraintGuide::default()"###)
}
pub fn gget_constraintlayout() -> (ConstraintLayout, &'static str) {
    (ConstraintLayout::new(), r###"ConstraintLayout::new()"###)
}
pub fn gget_cssprovider() -> (CssProvider, &'static str) {
    (CssProvider::new(), r###"CssProvider::new()"###)
}
pub fn gget_dialog() -> (Dialog, &'static str) {
    (Dialog::new(), r###"Dialog::new()"###)
}
pub fn gget_dragsource() -> (DragSource, &'static str) {
    (DragSource::new(), r###"DragSource::new()"###)
}
pub fn gget_drawingarea() -> (DrawingArea, &'static str) {
    (DrawingArea::new(), r###"DrawingArea::new()"###)
}
pub fn gget_dropcontrollermotion() -> (DropControllerMotion, &'static str) {
    (DropControllerMotion::new(), r###"DropControllerMotion::new()"###)
}
pub fn gget_droptarget() -> (DropTarget, &'static str) {
    (
        DropTarget::new(stek_glib_type().0, DragAction::COPY),
        r###"DropTarget::new(stek_glib_type().0, DragAction::COPY)"###,
    )
}
pub fn gget_droptargetasync() -> (DropTargetAsync, &'static str) {
    (DropTargetAsync::default(), r###"DropTargetAsync::default()"###)
}
pub fn gget_editablelabel() -> (EditableLabel, &'static str) {
    (EditableLabel::new(&take_string()), r###"EditableLabel::new(&take_string())"###)
}
pub fn gget_emojichooser() -> (EmojiChooser, &'static str) {
    (EmojiChooser::new(), r###"EmojiChooser::new()"###)
}
pub fn gget_entry() -> (Entry, &'static str) {
    (Entry::default(), r###"Entry::default()"###)
}
pub fn gget_entrybuffer() -> (EntryBuffer, &'static str) {
    (EntryBuffer::default(), r###"EntryBuffer::default()"###)
}
pub fn gget_entrycompletion() -> (EntryCompletion, &'static str) {
    (EntryCompletion::new(), r###"EntryCompletion::new()"###)
}
pub fn gget_eventcontrollerfocus() -> (EventControllerFocus, &'static str) {
    (EventControllerFocus::new(), r###"EventControllerFocus::new()"###)
}
pub fn gget_eventcontrollerkey() -> (EventControllerKey, &'static str) {
    (EventControllerKey::new(), r###"EventControllerKey::new()"###)
}
pub fn gget_eventcontrollermotion() -> (EventControllerMotion, &'static str) {
    (EventControllerMotion::new(), r###"EventControllerMotion::new()"###)
}
pub fn gget_eventcontrollerscroll() -> (EventControllerScroll, &'static str) {
    (EventControllerScroll::default(), r###"EventControllerScroll::default()"###)
}
pub fn gget_everyfilter() -> (EveryFilter, &'static str) {
    (EveryFilter::new(), r###"EveryFilter::new()"###)
}
pub fn gget_expander() -> (Expander, &'static str) {
    (Expander::default(), r###"Expander::default()"###)
}
pub fn gget_filechooserdialog() -> (FileChooserDialog, &'static str) {
    (FileChooserDialog::default(), r###"FileChooserDialog::default()"###)
}
pub fn gget_filechoosernative() -> (FileChooserNative, &'static str) {
    (FileChooserNative::default(), r###"FileChooserNative::default()"###)
}
pub fn gget_filechooserwidget() -> (FileChooserWidget, &'static str) {
    (FileChooserWidget::default(), r###"FileChooserWidget::default()"###)
}
pub fn gget_filefilter() -> (FileFilter, &'static str) {
    (FileFilter::new(), r###"FileFilter::new()"###)
}
pub fn gget_filterlistmodel() -> (FilterListModel, &'static str) {
    (FilterListModel::default(), r###"FilterListModel::default()"###)
}
pub fn gget_fixed() -> (Fixed, &'static str) {
    (Fixed::new(), r###"Fixed::new()"###)
}
pub fn gget_fixedlayout() -> (FixedLayout, &'static str) {
    (FixedLayout::new(), r###"FixedLayout::new()"###)
}
pub fn gget_flowbox() -> (FlowBox, &'static str) {
    (FlowBox::new(), r###"FlowBox::new()"###)
}
pub fn gget_flowboxchild() -> (FlowBoxChild, &'static str) {
    (FlowBoxChild::new(), r###"FlowBoxChild::new()"###)
}
pub fn gget_fontbutton() -> (FontButton, &'static str) {
    (FontButton::new(), r###"FontButton::new()"###)
}
pub fn gget_fontchooserdialog() -> (FontChooserDialog, &'static str) {
    (FontChooserDialog::default(), r###"FontChooserDialog::default()"###)
}
pub fn gget_fontchooserwidget() -> (FontChooserWidget, &'static str) {
    (FontChooserWidget::new(), r###"FontChooserWidget::new()"###)
}
pub fn gget_frame() -> (Frame, &'static str) {
    (Frame::default(), r###"Frame::default()"###)
}
pub fn gget_gestureclick() -> (GestureClick, &'static str) {
    (GestureClick::new(), r###"GestureClick::new()"###)
}
pub fn gget_gesturedrag() -> (GestureDrag, &'static str) {
    (GestureDrag::new(), r###"GestureDrag::new()"###)
}
pub fn gget_gesturelongpress() -> (GestureLongPress, &'static str) {
    (GestureLongPress::new(), r###"GestureLongPress::new()"###)
}
pub fn gget_gesturepan() -> (GesturePan, &'static str) {
    (GesturePan::new(stek_orientation().0), r###"GesturePan::new(stek_orientation().0)"###)
}
pub fn gget_gesturerotate() -> (GestureRotate, &'static str) {
    (GestureRotate::new(), r###"GestureRotate::new()"###)
}
pub fn gget_gesturestylus() -> (GestureStylus, &'static str) {
    (GestureStylus::new(), r###"GestureStylus::new()"###)
}
pub fn gget_gestureswipe() -> (GestureSwipe, &'static str) {
    (GestureSwipe::new(), r###"GestureSwipe::new()"###)
}
pub fn gget_gesturezoom() -> (GestureZoom, &'static str) {
    (GestureZoom::new(), r###"GestureZoom::new()"###)
}
pub fn gget_glarea() -> (GLArea, &'static str) {
    (GLArea::new(), r###"GLArea::new()"###)
}
pub fn gget_grid() -> (Grid, &'static str) {
    (Grid::new(), r###"Grid::new()"###)
}
pub fn gget_gridlayout() -> (GridLayout, &'static str) {
    (GridLayout::new(), r###"GridLayout::new()"###)
}
pub fn gget_gridview() -> (GridView, &'static str) {
    (GridView::default(), r###"GridView::default()"###)
}
pub fn gget_headerbar() -> (HeaderBar, &'static str) {
    (HeaderBar::new(), r###"HeaderBar::new()"###)
}
pub fn gget_icontheme() -> (IconTheme, &'static str) {
    (IconTheme::new(), r###"IconTheme::new()"###)
}
pub fn gget_iconview() -> (IconView, &'static str) {
    (IconView::new(), r###"IconView::new()"###)
}
pub fn gget_image() -> (Image, &'static str) {
    (Image::new(), r###"Image::new()"###)
}
pub fn gget_imcontextsimple() -> (IMContextSimple, &'static str) {
    (IMContextSimple::new(), r###"IMContextSimple::new()"###)
}
pub fn gget_immulticontext() -> (IMMulticontext, &'static str) {
    (IMMulticontext::new(), r###"IMMulticontext::new()"###)
}
pub fn gget_infobar() -> (InfoBar, &'static str) {
    (InfoBar::new(), r###"InfoBar::new()"###)
}
pub fn gget_label() -> (Label, &'static str) {
    (Label::default(), r###"Label::default()"###)
}
pub fn gget_levelbar() -> (LevelBar, &'static str) {
    (LevelBar::new(), r###"LevelBar::new()"###)
}
pub fn gget_linkbutton() -> (LinkButton, &'static str) {
    (LinkButton::default(), r###"LinkButton::default()"###)
}
pub fn gget_listbox() -> (ListBox, &'static str) {
    (ListBox::new(), r###"ListBox::new()"###)
}
pub fn gget_listboxrow() -> (ListBoxRow, &'static str) {
    (ListBoxRow::new(), r###"ListBoxRow::new()"###)
}
pub fn gget_liststore() -> (ListStore, &'static str) {
    (ListStore::new(&[stek_glib_type().0]), r###"ListStore::new(&[stek_glib_type().0])"###)
}
pub fn gget_listview() -> (ListView, &'static str) {
    (ListView::default(), r###"ListView::default()"###)
}
pub fn gget_lockbutton() -> (LockButton, &'static str) {
    (LockButton::default(), r###"LockButton::default()"###)
}
pub fn gget_mediacontrols() -> (MediaControls, &'static str) {
    (MediaControls::default(), r###"MediaControls::default()"###)
}
pub fn gget_mediafile() -> (MediaFile, &'static str) {
    (MediaFile::new(), r###"MediaFile::new()"###)
}
pub fn gget_menubutton() -> (MenuButton, &'static str) {
    (MenuButton::new(), r###"MenuButton::new()"###)
}
pub fn gget_messagedialog() -> (MessageDialog, &'static str) {
    (MessageDialog::default(), r###"MessageDialog::default()"###)
}
pub fn gget_mnemonicaction() -> (MnemonicAction, &'static str) {
    (MnemonicAction::get(), r###"MnemonicAction::get()"###)
}
pub fn gget_mountoperation() -> (MountOperation, &'static str) {
    (MountOperation::default(), r###"MountOperation::default()"###)
}
pub fn gget_multisorter() -> (MultiSorter, &'static str) {
    (MultiSorter::new(), r###"MultiSorter::new()"###)
}
pub fn gget_namedaction() -> (NamedAction, &'static str) {
    (NamedAction::new(&take_string()), r###"NamedAction::new(&take_string())"###)
}
pub fn gget_nevertrigger() -> (NeverTrigger, &'static str) {
    (NeverTrigger::get(), r###"NeverTrigger::get()"###)
}
pub fn gget_notebook() -> (Notebook, &'static str) {
    (Notebook::default(), r###"Notebook::default()"###)
}
pub fn gget_nothingaction() -> (NothingAction, &'static str) {
    (NothingAction::get(), r###"NothingAction::get()"###)
}
pub fn gget_numericsorter() -> (NumericSorter, &'static str) {
    (NumericSorter::default(), r###"NumericSorter::default()"###)
}
pub fn gget_overlay() -> (Overlay, &'static str) {
    (Overlay::new(), r###"Overlay::new()"###)
}
pub fn gget_overlaylayout() -> (OverlayLayout, &'static str) {
    (OverlayLayout::new(), r###"OverlayLayout::new()"###)
}
pub fn gget_padcontroller() -> (PadController, &'static str) {
    (PadController::default(), r###"PadController::default()"###)
}
pub fn gget_pagesetup() -> (PageSetup, &'static str) {
    (PageSetup::new(), r###"PageSetup::new()"###)
}
//pub fn gget_pagesetupunixdialog() -> (PageSetupUnixDialog, &'static str) {
//    (PageSetupUnixDialog::default(), r###"PageSetupUnixDialog::default()"###)
//}
pub fn gget_paned() -> (Paned, &'static str) {
    (Paned::default(), r###"Paned::default()"###)
}
pub fn gget_papersize() -> (PaperSize, &'static str) {
    (PaperSize::new(None), r###"PaperSize::new(None)"###)
}
pub fn gget_passwordentry() -> (PasswordEntry, &'static str) {
    (PasswordEntry::new(), r###"PasswordEntry::new()"###)
}
pub fn gget_picture() -> (Picture, &'static str) {
    (Picture::new(), r###"Picture::new()"###)
}
pub fn gget_popover() -> (Popover, &'static str) {
    (Popover::new(), r###"Popover::new()"###)
}
pub fn gget_printoperation() -> (PrintOperation, &'static str) {
    (PrintOperation::new(), r###"PrintOperation::new()"###)
}
pub fn gget_printsettings() -> (PrintSettings, &'static str) {
    (PrintSettings::new(), r###"PrintSettings::new()"###)
}
//pub fn gget_printunixdialog() -> (PrintUnixDialog, &'static str) {
//    (PrintUnixDialog::default(), r###"PrintUnixDialog::default()"###)
//}
pub fn gget_progressbar() -> (ProgressBar, &'static str) {
    (ProgressBar::new(), r###"ProgressBar::new()"###)
}
pub fn gget_recentmanager() -> (RecentManager, &'static str) {
    (RecentManager::new(), r###"RecentManager::new()"###)
}
pub fn gget_revealer() -> (Revealer, &'static str) {
    (Revealer::new(), r###"Revealer::new()"###)
}
pub fn gget_scale() -> (Scale, &'static str) {
    (Scale::default(), r###"Scale::default()"###)
}
pub fn gget_scalebutton() -> (ScaleButton, &'static str) {
    (ScaleButton::default(), r###"ScaleButton::default()"###)
}
pub fn gget_scrollbar() -> (Scrollbar, &'static str) {
    (Scrollbar::default(), r###"Scrollbar::default()"###)
}
pub fn gget_scrolledwindow() -> (ScrolledWindow, &'static str) {
    (ScrolledWindow::new(), r###"ScrolledWindow::new()"###)
}
pub fn gget_searchbar() -> (SearchBar, &'static str) {
    (SearchBar::new(), r###"SearchBar::new()"###)
}
pub fn gget_searchentry() -> (SearchEntry, &'static str) {
    (SearchEntry::new(), r###"SearchEntry::new()"###)
}
pub fn gget_separator() -> (Separator, &'static str) {
    (Separator::default(), r###"Separator::default()"###)
}
pub fn gget_settings() -> (Settings, &'static str) {
    (SettingsBuilder::build(Default::default()), r###"SettingsBuilder::build(Default::default())"###)
}
pub fn gget_shortcut() -> (Shortcut, &'static str) {
    (Shortcut::default(), r###"Shortcut::default()"###)
}
pub fn gget_shortcutcontroller() -> (ShortcutController, &'static str) {
    (ShortcutController::new(), r###"ShortcutController::new()"###)
}
pub fn gget_shortcutlabel() -> (ShortcutLabel, &'static str) {
    (ShortcutLabel::default(), r###"ShortcutLabel::default()"###)
}
pub fn gget_shortcutsgroup() -> (ShortcutsGroup, &'static str) {
    (ShortcutsGroup::builder().build(), r###"ShortcutsGroup::builder().build()"###)
}
pub fn gget_shortcutsshortcut() -> (ShortcutsShortcut, &'static str) {
    (ShortcutsShortcut::builder().build(), r###"ShortcutsShortcut::builder().build()"###)
}
pub fn gget_shortcutswindow() -> (ShortcutsWindow, &'static str) {
    (ShortcutsWindow::builder().build(), r###"ShortcutsWindow::builder().build()"###)
}
pub fn gget_signalaction() -> (SignalAction, &'static str) {
    (SignalAction::new(&take_string()), r###"SignalAction::new(&take_string())"###)
}
pub fn gget_singleselection() -> (SingleSelection, &'static str) {
    (SingleSelection::default(), r###"SingleSelection::default()"###)
}
pub fn gget_sizegroup() -> (SizeGroup, &'static str) {
    (SizeGroup::new(stek_sizegroupmode().0), r###"SizeGroup::new(stek_sizegroupmode().0)"###)
}
pub fn gget_slicelistmodel() -> (SliceListModel, &'static str) {
    (SliceListModel::default(), r###"SliceListModel::default()"###)
}
pub fn gget_snapshot() -> (Snapshot, &'static str) {
    (Snapshot::default(), r###"Snapshot::default()"###)
}
pub fn gget_sortlistmodel() -> (SortListModel, &'static str) {
    (SortListModel::default(), r###"SortListModel::default()"###)
}
pub fn gget_spinbutton() -> (SpinButton, &'static str) {
    (SpinButton::default(), r###"SpinButton::default()"###)
}
pub fn gget_spinner() -> (Spinner, &'static str) {
    (Spinner::new(), r###"Spinner::new()"###)
}
pub fn gget_stack() -> (Stack, &'static str) {
    (Stack::new(), r###"Stack::new()"###)
}
pub fn gget_stacksidebar() -> (StackSidebar, &'static str) {
    (StackSidebar::new(), r###"StackSidebar::new()"###)
}
pub fn gget_stackswitcher() -> (StackSwitcher, &'static str) {
    (StackSwitcher::new(), r###"StackSwitcher::new()"###)
}
pub fn gget_statusbar() -> (Statusbar, &'static str) {
    (Statusbar::new(), r###"Statusbar::new()"###)
}
pub fn gget_stringfilter() -> (StringFilter, &'static str) {
    (StringFilter::default(), r###"StringFilter::default()"###)
}
pub fn gget_stringlist() -> (StringList, &'static str) {
    (StringList::default(), r###"StringList::default()"###)
}
pub fn gget_stringobject() -> (StringObject, &'static str) {
    (StringObject::new(&take_string()), r###"StringObject::new(&take_string())"###)
}
pub fn gget_stringsorter() -> (StringSorter, &'static str) {
    (StringSorter::default(), r###"StringSorter::default()"###)
}
pub fn gget_switch() -> (Switch, &'static str) {
    (Switch::new(), r###"Switch::new()"###)
}
pub fn gget_text() -> (Text, &'static str) {
    (Text::new(), r###"Text::new()"###)
}
pub fn gget_textbuffer() -> (TextBuffer, &'static str) {
    (TextBuffer::default(), r###"TextBuffer::default()"###)
}
pub fn gget_textchildanchor() -> (TextChildAnchor, &'static str) {
    (TextChildAnchor::default(), r###"TextChildAnchor::default()"###)
}
pub fn gget_textmark() -> (TextMark, &'static str) {
    (TextMark::default(), r###"TextMark::default()"###)
}
pub fn gget_texttag() -> (TextTag, &'static str) {
    (TextTag::default(), r###"TextTag::default()"###)
}
pub fn gget_texttagtable() -> (TextTagTable, &'static str) {
    (TextTagTable::new(), r###"TextTagTable::new()"###)
}
pub fn gget_textview() -> (TextView, &'static str) {
    (TextView::new(), r###"TextView::new()"###)
}
pub fn gget_togglebutton() -> (ToggleButton, &'static str) {
    (ToggleButton::new(), r###"ToggleButton::new()"###)
}
pub fn gget_treeexpander() -> (TreeExpander, &'static str) {
    (TreeExpander::new(), r###"TreeExpander::new()"###)
}
pub fn gget_treelistmodel() -> (TreeListModel, &'static str) {
    (TreeListModel::default(), r###"TreeListModel::default()"###)
}
pub fn gget_treepath() -> (TreePath, &'static str) {
    (TreePath::default(), r###"TreePath::default()"###)
}
pub fn gget_treestore() -> (TreeStore, &'static str) {
    (TreeStore::new(&[stek_glib_type().0]), r###"TreeStore::new(&[stek_glib_type().0])"###)
}
pub fn gget_treeview() -> (TreeView, &'static str) {
    (TreeView::new(), r###"TreeView::new()"###)
}
pub fn gget_treeviewcolumn() -> (TreeViewColumn, &'static str) {
    (TreeViewColumn::new(), r###"TreeViewColumn::new()"###)
}
pub fn gget_video() -> (Video, &'static str) {
    (Video::new(), r###"Video::new()"###)
}
pub fn gget_viewport() -> (Viewport, &'static str) {
    (Viewport::default(), r###"Viewport::default()"###)
}
pub fn gget_volumebutton() -> (VolumeButton, &'static str) {
    (VolumeButton::new(), r###"VolumeButton::new()"###)
}
pub fn gget_window() -> (Window, &'static str) {
    (Window::new(), r###"Window::new()"###)
}
pub fn gget_windowcontrols() -> (WindowControls, &'static str) {
    (WindowControls::default(), r###"WindowControls::default()"###)
}
pub fn gget_windowhandle() -> (WindowHandle, &'static str) {
    (WindowHandle::new(), r###"WindowHandle::new()"###)
}
pub fn gget_popovermenu() -> (PopoverMenu, &'static str) {
    let thing: Option<&gio::MenuModel> = None;
    (
        PopoverMenu::from_model(thing),
        r###"let thing: Option<&gio::MenuModel> = None;    PopoverMenu::from_model(thing)"###,
    )
}
pub fn gget_popovermenubar() -> (PopoverMenuBar, &'static str) {
    let thing: Option<&gio::MenuModel> = None;
    (
        PopoverMenuBar::from_model(thing),
        r###"let thing: Option<&gio::MenuModel> = None;    PopoverMenuBar::from_model(thing)"###,
    )
}
pub fn gget_widgetpaintable() -> (WidgetPaintable, &'static str) {
    let thing: Option<&Widget> = None;
    (WidgetPaintable::new(thing), r###"let thing: Option<&Widget> = None;    WidgetPaintable::new(thing)"###)
}
pub fn gget_treelistrowsorter() -> (TreeListRowSorter, &'static str) {
    let thing: Option<&Sorter> = None;
    (TreeListRowSorter::new(thing), r###"let thing: Option<&Sorter> = None;    TreeListRowSorter::new(thing)"###)
}
pub fn gget_selectionfiltermodel() -> (SelectionFilterModel, &'static str) {
    let thing: Option<&SelectionModel> = None;
    (
        SelectionFilterModel::new(thing),
        r###"let thing: Option<&SelectionModel> = None;    SelectionFilterModel::new(thing)"###,
    )
}
pub fn gget_noselection() -> (NoSelection, &'static str) {
    let thing: Option<&gio::ListModel> = None;
    (NoSelection::new(thing), r###"let thing: Option<&gio::ListModel> = None;    NoSelection::new(thing)"###)
}
pub fn gget_mnemonictrigger() -> (MnemonicTrigger, &'static str) {
    (
        MnemonicTrigger::new(gdk::Key::from_name("f").unwrap()),
        r###"MnemonicTrigger::new(gdk::Key::from_name("f").unwrap())"###,
    )
}
pub fn gget_multiselection() -> (MultiSelection, &'static str) {
    let thing: Option<&gio::ListModel> = None;
    (
        MultiSelection::new(thing),
        r###"let thing: Option<&gio::ListModel> = None;    MultiSelection::new(thing)"###,
    )
}
pub fn gget_keyvaltrigger() -> (KeyvalTrigger, &'static str) {
    let thing = gdk::Key::from_name("f").unwrap();
    let thing2 = gdk::ModifierType::SHIFT_MASK;
    (
        KeyvalTrigger::new(thing, thing2),
        r###"let thing = gdk::Key::from_name("f").unwrap();    let thing2 = gdk::ModifierType::SHIFT_MASK;    KeyvalTrigger::new(thing, thing2)"###,
    )
}
pub fn gget_iconpaintable() -> (IconPaintable, &'static str) {
    let thin: Option<String> = None;
    let thing = gio::File::new_tmp(thin).unwrap();
    (
        IconPaintable::for_file(&thing.0, take_i32(), take_i32()),
        r###"let thin: Option<String> = None;    let thing = gio::File::new_tmp(thin).unwrap();    IconPaintable::for_file(&thing.0, take_i32(), take_i32())"###,
    )
}
pub fn gget_flattenlistmodel() -> (FlattenListModel, &'static str) {
    let thing: Option<&gio::ListModel> = None;
    (
        FlattenListModel::new(thing),
        r###"let thing: Option<&gio::ListModel> = None;    FlattenListModel::new(thing)"###,
    )
}
pub fn gget_dropdown() -> (DropDown, &'static str) {
    let thing: Option<&Expression> = None;
    let thing2: Option<&gio::ListModel> = None;
    (
        DropDown::new(thing2, thing),
        r###"let thing: Option<&Expression> = None;    let thing2: Option<&gio::ListModel> = None;    DropDown::new(thing2, thing)"###,
    )
}
pub fn gget_directorylist() -> (DirectoryList, &'static str) {
    let thing: Option<&gio::File> = None;
    (
        DirectoryList::new(Some(&take_string()), thing),
        r###"let thing: Option<&gio::File> = None;    DirectoryList::new(Some(&take_string()), thing)"###,
    )
}
pub fn gget_constraint() -> (Constraint, &'static str) {
    let thing: Option<&ConstraintTarget> = None;
    let thing2: ConstraintAttribute = ConstraintAttribute::None;
    let thing3: ConstraintRelation = ConstraintRelation::Eq;
    (
        Constraint::new_constant(thing, thing2, thing3, take_f64(), take_i32()),
        r###"let thing: Option<&ConstraintTarget> = None;    let thing2: ConstraintAttribute = ConstraintAttribute::None;    let thing3: ConstraintRelation = ConstraintRelation::Eq;    Constraint::new_constant(thing, thing2, thing3, take_f64(), take_i32())"###,
    )
}
pub fn gget_columnview() -> (ColumnView, &'static str) {
    let sm: Option<&SelectionModel> = None;
    (ColumnView::new(sm), r###"let sm: Option<&SelectionModel> = None;    ColumnView::new(sm)"###)
}
pub fn gget_columnviewcolumn() -> (ColumnViewColumn, &'static str) {
    let thing: Option<&ListItemFactory> = None;
    (
        ColumnViewColumn::new(Some(&take_string()), thing),
        r###"let thing: Option<&ListItemFactory> = None;    ColumnViewColumn::new(Some(&take_string()), thing)"###,
    )
}
pub fn gget_bookmarklist() -> (BookmarkList, &'static str) {
    (
        BookmarkList::new(Some(&take_string()), Some(&take_string())),
        r###"BookmarkList::new(Some(&take_string()), Some(&take_string()))"###,
    )
}
pub fn gget_boolfilter() -> (BoolFilter, &'static str) {
    let expression: Option<Expression> = None;
    (
        BoolFilter::new(expression),
        r###"let expression: Option<Expression> = None;    BoolFilter::new(expression)"###,
    )
}
pub fn gget_aspectframe() -> (AspectFrame, &'static str) {
    (
        AspectFrame::new(take_f32(), take_f32(), take_f32(), take_bool()),
        r###"AspectFrame::new(take_f32(), take_f32(), take_f32(), take_bool())"###,
    )
}
pub fn gget_appchooserdialog() -> (AppChooserDialog, &'static str) {
    let expression: Option<&Window> = None;
    (
        AppChooserDialog::for_content_type(expression, DialogFlags::all(), &take_string()),
        r###"let expression: Option<&Window> = None;    AppChooserDialog::for_content_type(expression, DialogFlags::all(), &take_string())"###,
    )
}
// pub fn gget_atcontext() -> ATContext {
//     ATContext::new()
// }
// pub fn gget_passwordentrybuffer() -> PasswordEntryBuffer {
//     PasswordEntryBuffer::new()
// }
// pub fn gget_allocation() -> Allocation {
//     Allocation::default()
// }
// pub fn gget_treeiter() -> TreeIter {
//     TreeIter::default()
// }
// pub fn gget_application() -> Application {
//     Application::default()
// }
// pub fn gget_shortcutaction() -> ShortcutAction {
//     ShortcutAction::parse_string(&take_string())
// }
// pub fn gget_shortcuttrigger() -> ShortcutTrigger {
//     ShortcutTrigger::parse_string()
// }
// pub fn gget_applicationwindow() -> ApplicationWindow{
//     ApplicationWindow::new()
// }
// pub fn gget_treemodelsort() -> TreeModelSort {
//     TreeModelSort::with_model()
// }
// pub fn gget_shortcutssection() -> ShortcutsSection {
//     ShortcutsSection::new()
// }
// pub fn gget_printcapabilities() -> PrintCapabilities {
//     PrintCapabilities::defa()
// }
// pub fn gget_eventcontrollerscrollflags() -> EventControllerScrollFlags {
//     EventControllerScrollFlags::de()
// }
// Bitflags
// pub fn gget_inputhints() -> InputHints {
//     InputHints::de()
// }
// pub fn gget_assistantpage() -> AssistantPage {
//     AssistantPage::new()
// }
// pub fn gget_builderlistitemfactory() -> BuilderListItemFactory {
//     let builder_scope : Option<BuilderScope> = None;
//     BuilderListItemFactory::new(builder_scope,)
// }
// pub fn gget_dragicon() -> DragIcon {
//     DragIcon::new()
// }
// pub fn gget_fixedlayoutchild() -> FixedLayoutChild {
//     FixedLayoutChild::default()
// }
// pub fn gget_gridlayoutchild() -> GridLayoutChild {
//     GridLayoutChild::new()
// }
// pub fn gget_tooltip() -> Tooltip {
//     Tooltip::()
// }
// pub fn gget_listitem() -> ListItem {
//     ListItem::new()
// }
// pub fn gget_maplistmodel() -> MapListModel {
//     MapListModel::new()
// }
// pub fn gget_notebookpage() -> NotebookPage {
//     NotebookPage::new()
// }
// pub fn gget_overlaylayoutchild() -> OverlayLayoutChild {
//     OverlayLayoutChild::new()
// }
// pub fn gget_printcontext() -> PrintContext {
//     PrintContext::new()
// }
// pub fn gget_printjob() -> PrintJob {
//     PrintJob::default()
// }
// pub fn gget_printer() ->  {
//     PrinterBuilder::build(Default::default())
// }
// pub fn gget_treeselection() -> TreeSelection {
//     TreeSelection::def()
// }
// pub fn gget_widget() -> Widget {
//     Widget::new()
// }
// pub fn gget_treelistrow() -> TreeListRow {
//     TreeListRow::new()
// }
// pub fn gget_stackpage() -> StackPage {
//     StackPage::new()
// }
