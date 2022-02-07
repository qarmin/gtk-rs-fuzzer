use gtk4::prelude::*;
use gtk4::*;
use glib::{random_int, random_int_range,random_double_range};
use gdk4::DragAction;
use rand::prelude::SliceRandom;

pub fn execute_things(){


	// AboutDialog
	{
		for _i in 0..100{
			let _object1 = gget_aboutdialog();
			println!("Trying to execute AboutDialog.logo()");
			_object1.logo();
			println!("Executed AboutDialog.logo()");
		}
		for _i in 0..100{
			let _object3 = gget_aboutdialog();
			println!("Trying to execute AboutDialog.wraps_license()");
			_object3.wraps_license();
			println!("Executed AboutDialog.wraps_license()");
		}
	}
	// ActionBar
	{
		for _i in 0..100{
			let _object5 = gget_actionbar();
			println!("Trying to execute ActionBar.center_widget()");
			_object5.center_widget();
			println!("Executed ActionBar.center_widget()");
		}
		for _i in 0..100{
			let _object6 = gget_actionbar();
			println!("Trying to execute ActionBar.is_revealed()");
			_object6.is_revealed();
			println!("Executed ActionBar.is_revealed()");
		}
	}
	// ActivateAction
	{
	}
	// Adjustment
	{
	}
	// AlternativeTrigger
	{
	}
	// AnyFilter
	{
	}
	// AppChooserButton
	{
		for _i in 0..100{
			let _object13 = gget_appchooserbutton();
			println!("Trying to execute AppChooserButton.append_separator()");
			_object13.append_separator();
			println!("Executed AppChooserButton.append_separator()");
		}
		for _i in 0..100{
			let _object15 = gget_appchooserbutton();
			println!("Trying to execute AppChooserButton.is_modal()");
			_object15.is_modal();
			println!("Executed AppChooserButton.is_modal()");
		}
		for _i in 0..100{
			let _object17 = gget_appchooserbutton();
			println!("Trying to execute AppChooserButton.shows_default_item()");
			_object17.shows_default_item();
			println!("Executed AppChooserButton.shows_default_item()");
		}
		for _i in 0..100{
			let _object18 = gget_appchooserbutton();
			println!("Trying to execute AppChooserButton.shows_dialog_item()");
			_object18.shows_dialog_item();
			println!("Executed AppChooserButton.shows_dialog_item()");
		}
	}
	// AppChooserDialog
	{
		for _i in 0..100{
			let _object20 = gget_appchooserdialog();
			println!("Trying to execute AppChooserDialog.gfile()");
			_object20.gfile();
			println!("Executed AppChooserDialog.gfile()");
		}
		for _i in 0..100{
			let _object22 = gget_appchooserdialog();
			println!("Trying to execute AppChooserDialog.widget()");
			_object22.widget();
			println!("Executed AppChooserDialog.widget()");
		}
	}
	// AppChooserWidget
	{
		for _i in 0..100{
			let _object25 = gget_appchooserwidget();
			println!("Trying to execute AppChooserWidget.shows_all()");
			_object25.shows_all();
			println!("Executed AppChooserWidget.shows_all()");
		}
		for _i in 0..100{
			let _object26 = gget_appchooserwidget();
			println!("Trying to execute AppChooserWidget.shows_default()");
			_object26.shows_default();
			println!("Executed AppChooserWidget.shows_default()");
		}
		for _i in 0..100{
			let _object27 = gget_appchooserwidget();
			println!("Trying to execute AppChooserWidget.shows_fallback()");
			_object27.shows_fallback();
			println!("Executed AppChooserWidget.shows_fallback()");
		}
		for _i in 0..100{
			let _object28 = gget_appchooserwidget();
			println!("Trying to execute AppChooserWidget.shows_other()");
			_object28.shows_other();
			println!("Executed AppChooserWidget.shows_other()");
		}
		for _i in 0..100{
			let _object29 = gget_appchooserwidget();
			println!("Trying to execute AppChooserWidget.shows_recommended()");
			_object29.shows_recommended();
			println!("Executed AppChooserWidget.shows_recommended()");
		}
	}
	// Application
	{
	}
	// ApplicationWindow
	{
	}
	// AspectFrame
	{
		for _i in 0..100{
			let _object35 = gget_aspectframe();
			println!("Trying to execute AspectFrame.is_obey_child()");
			_object35.is_obey_child();
			println!("Executed AspectFrame.is_obey_child()");
		}
	}
	// Assistant
	{
		for _i in 0..100{
			let _object38 = gget_assistant();
			println!("Trying to execute Assistant.commit()");
			_object38.commit();
			println!("Executed Assistant.commit()");
		}
		for _i in 0..100{
			let _object39 = gget_assistant();
			println!("Trying to execute Assistant.current_page()");
			_object39.current_page();
			println!("Executed Assistant.current_page()");
		}
		for _i in 0..100{
			let _object40 = gget_assistant();
			println!("Trying to execute Assistant.n_pages()");
			_object40.n_pages();
			println!("Executed Assistant.n_pages()");
		}
		for _i in 0..100{
			let _object42 = gget_assistant();
			println!("Trying to execute Assistant.next_page()");
			_object42.next_page();
			println!("Executed Assistant.next_page()");
		}
		for _i in 0..100{
			let _object43 = gget_assistant();
			println!("Trying to execute Assistant.pages()");
			_object43.pages();
			println!("Executed Assistant.pages()");
		}
		for _i in 0..100{
			let _object44 = gget_assistant();
			println!("Trying to execute Assistant.previous_page()");
			_object44.previous_page();
			println!("Executed Assistant.previous_page()");
		}
		for _i in 0..100{
			let _object45 = gget_assistant();
			println!("Trying to execute Assistant.update_buttons_state()");
			_object45.update_buttons_state();
			println!("Executed Assistant.update_buttons_state()");
		}
	}
	// BinLayout
	{
	}
	// BookmarkList
	{
		for _i in 0..100{
			let _object47 = gget_bookmarklist();
			println!("Trying to execute BookmarkList.attributes()");
			_object47.attributes();
			println!("Executed BookmarkList.attributes()");
		}
		for _i in 0..100{
			let _object48 = gget_bookmarklist();
			println!("Trying to execute BookmarkList.filename()");
			_object48.filename();
			println!("Executed BookmarkList.filename()");
		}
		for _i in 0..100{
			let _object49 = gget_bookmarklist();
			println!("Trying to execute BookmarkList.is_loading()");
			_object49.is_loading();
			println!("Executed BookmarkList.is_loading()");
		}
	}
	// Box
	{
	}
	// BoxLayout
	{
		for _i in 0..100{
			let _object53 = gget_boxlayout();
			println!("Trying to execute BoxLayout.is_homogeneous()");
			_object53.is_homogeneous();
			println!("Executed BoxLayout.is_homogeneous()");
		}
	}
	// Builder
	{
		for _i in 0..100{
			let _object56 = gget_builder();
			println!("Trying to execute Builder.objects()");
			_object56.objects();
			println!("Executed Builder.objects()");
		}
		for _i in 0..100{
			let _object57 = gget_builder();
			println!("Trying to execute Builder.scope()");
			_object57.scope();
			println!("Executed Builder.scope()");
		}
		for _i in 0..100{
			let _object58 = gget_builder();
			println!("Trying to execute Builder.translation_domain()");
			_object58.translation_domain();
			println!("Executed Builder.translation_domain()");
		}
	}
	// Button
	{
	}
	// Calendar
	{
		for _i in 0..100{
			let _object62 = gget_calendar();
			println!("Trying to execute Calendar.clear_marks()");
			_object62.clear_marks();
			println!("Executed Calendar.clear_marks()");
		}
		for _i in 0..100{
			let _object63 = gget_calendar();
			println!("Trying to execute Calendar.date()");
			_object63.date();
			println!("Executed Calendar.date()");
		}
		for _i in 0..100{
			let _object65 = gget_calendar();
			println!("Trying to execute Calendar.shows_day_names()");
			_object65.shows_day_names();
			println!("Executed Calendar.shows_day_names()");
		}
		for _i in 0..100{
			let _object66 = gget_calendar();
			println!("Trying to execute Calendar.shows_heading()");
			_object66.shows_heading();
			println!("Executed Calendar.shows_heading()");
		}
		for _i in 0..100{
			let _object67 = gget_calendar();
			println!("Trying to execute Calendar.shows_week_numbers()");
			_object67.shows_week_numbers();
			println!("Executed Calendar.shows_week_numbers()");
		}
	}
	// CellAreaBox
	{
	}
	// CellRendererAccel
	{
		for _i in 0..100{
			let _object70 = gget_cellrendereraccel();
			println!("Trying to execute CellRendererAccel.accel_mods()");
			_object70.accel_mods();
			println!("Executed CellRendererAccel.accel_mods()");
		}
	}
	// CellRendererCombo
	{
	}
	// CellRendererPixbuf
	{
		for _i in 0..100{
			let _object76 = gget_cellrendererpixbuf();
			println!("Trying to execute CellRendererPixbuf.gicon()");
			_object76.gicon();
			println!("Executed CellRendererPixbuf.gicon()");
		}
		for _i in 0..100{
			let _object78 = gget_cellrendererpixbuf();
			println!("Trying to execute CellRendererPixbuf.pixbuf_expander_closed()");
			_object78.pixbuf_expander_closed();
			println!("Executed CellRendererPixbuf.pixbuf_expander_closed()");
		}
		for _i in 0..100{
			let _object79 = gget_cellrendererpixbuf();
			println!("Trying to execute CellRendererPixbuf.pixbuf_expander_open()");
			_object79.pixbuf_expander_open();
			println!("Executed CellRendererPixbuf.pixbuf_expander_open()");
		}
		for _i in 0..100{
			let _object80 = gget_cellrendererpixbuf();
			println!("Trying to execute CellRendererPixbuf.texture()");
			_object80.texture();
			println!("Executed CellRendererPixbuf.texture()");
		}
	}
	// CellRendererProgress
	{
		for _i in 0..100{
			let _object82 = gget_cellrendererprogress();
			println!("Trying to execute CellRendererProgress.is_inverted()");
			_object82.is_inverted();
			println!("Executed CellRendererProgress.is_inverted()");
		}
	}
	// CellRendererSpin
	{
	}
	// CellRendererSpinner
	{
		for _i in 0..100{
			let _object87 = gget_cellrendererspinner();
			println!("Trying to execute CellRendererSpinner.is_active()");
			_object87.is_active();
			println!("Executed CellRendererSpinner.is_active()");
		}
	}
	// CellRendererText
	{
	}
	// CellRendererToggle
	{
		for _i in 0..100{
			let _object92 = gget_cellrenderertoggle();
			println!("Trying to execute CellRendererToggle.is_activatable()");
			_object92.is_activatable();
			println!("Executed CellRendererToggle.is_activatable()");
		}
		for _i in 0..100{
			let _object93 = gget_cellrenderertoggle();
			println!("Trying to execute CellRendererToggle.is_active()");
			_object93.is_active();
			println!("Executed CellRendererToggle.is_active()");
		}
		for _i in 0..100{
			let _object94 = gget_cellrenderertoggle();
			println!("Trying to execute CellRendererToggle.is_inconsistent()");
			_object94.is_inconsistent();
			println!("Executed CellRendererToggle.is_inconsistent()");
		}
		for _i in 0..100{
			let _object95 = gget_cellrenderertoggle();
			println!("Trying to execute CellRendererToggle.is_radio()");
			_object95.is_radio();
			println!("Executed CellRendererToggle.is_radio()");
		}
	}
	// CellView
	{
		for _i in 0..100{
			let _object98 = gget_cellview();
			println!("Trying to execute CellView.displayed_row()");
			_object98.displayed_row();
			println!("Executed CellView.displayed_row()");
		}
		for _i in 0..100{
			let _object99 = gget_cellview();
			println!("Trying to execute CellView.draws_sensitive()");
			_object99.draws_sensitive();
			println!("Executed CellView.draws_sensitive()");
		}
		for _i in 0..100{
			let _object100 = gget_cellview();
			println!("Trying to execute CellView.fits_model()");
			_object100.fits_model();
			println!("Executed CellView.fits_model()");
		}
	}
	// CenterBox
	{
		for _i in 0..100{
			let _object103 = gget_centerbox();
			println!("Trying to execute CenterBox.center_widget()");
			_object103.center_widget();
			println!("Executed CenterBox.center_widget()");
		}
		for _i in 0..100{
			let _object104 = gget_centerbox();
			println!("Trying to execute CenterBox.end_widget()");
			_object104.end_widget();
			println!("Executed CenterBox.end_widget()");
		}
		for _i in 0..100{
			let _object106 = gget_centerbox();
			println!("Trying to execute CenterBox.start_widget()");
			_object106.start_widget();
			println!("Executed CenterBox.start_widget()");
		}
	}
	// CenterLayout
	{
		for _i in 0..100{
			let _object107 = gget_centerlayout();
			println!("Trying to execute CenterLayout.baseline_position()");
			_object107.baseline_position();
			println!("Executed CenterLayout.baseline_position()");
		}
		for _i in 0..100{
			let _object108 = gget_centerlayout();
			println!("Trying to execute CenterLayout.center_widget()");
			_object108.center_widget();
			println!("Executed CenterLayout.center_widget()");
		}
		for _i in 0..100{
			let _object109 = gget_centerlayout();
			println!("Trying to execute CenterLayout.end_widget()");
			_object109.end_widget();
			println!("Executed CenterLayout.end_widget()");
		}
		for _i in 0..100{
			let _object111 = gget_centerlayout();
			println!("Trying to execute CenterLayout.orientation()");
			_object111.orientation();
			println!("Executed CenterLayout.orientation()");
		}
		for _i in 0..100{
			let _object112 = gget_centerlayout();
			println!("Trying to execute CenterLayout.start_widget()");
			_object112.start_widget();
			println!("Executed CenterLayout.start_widget()");
		}
	}
	// CheckButton
	{
	}
	// ColorButton
	{
		for _i in 0..100{
			let _object116 = gget_colorbutton();
			println!("Trying to execute ColorButton.is_modal()");
			_object116.is_modal();
			println!("Executed ColorButton.is_modal()");
		}
		for _i in 0..100{
			let _object118 = gget_colorbutton();
			println!("Trying to execute ColorButton.shows_editor()");
			_object118.shows_editor();
			println!("Executed ColorButton.shows_editor()");
		}
	}
	// ColorChooserWidget
	{
		for _i in 0..100{
			let _object121 = gget_colorchooserwidget();
			println!("Trying to execute ColorChooserWidget.shows_editor()");
			_object121.shows_editor();
			println!("Executed ColorChooserWidget.shows_editor()");
		}
	}
	// ComboBox
	{
	}
	// ComboBoxText
	{
		for _i in 0..100{
			let _object124 = gget_comboboxtext();
			println!("Trying to execute ComboBoxText.active_text()");
			_object124.active_text();
			println!("Executed ComboBoxText.active_text()");
		}
		for _i in 0..100{
			let _object127 = gget_comboboxtext();
			println!("Trying to execute ComboBoxText.remove_all()");
			_object127.remove_all();
			println!("Executed ComboBoxText.remove_all()");
		}
	}
	// Constraint
	{
		for _i in 0..100{
			let _object129 = gget_constraint();
			println!("Trying to execute Constraint.is_attached()");
			_object129.is_attached();
			println!("Executed Constraint.is_attached()");
		}
		for _i in 0..100{
			let _object130 = gget_constraint();
			println!("Trying to execute Constraint.is_constant()");
			_object130.is_constant();
			println!("Executed Constraint.is_constant()");
		}
		for _i in 0..100{
			let _object131 = gget_constraint();
			println!("Trying to execute Constraint.is_required()");
			_object131.is_required();
			println!("Executed Constraint.is_required()");
		}
	}
	// ConstraintGuide
	{
	}
	// ConstraintLayout
	{
		for _i in 0..100{
			let _object136 = gget_constraintlayout();
			println!("Trying to execute ConstraintLayout.observe_constraints()");
			_object136.observe_constraints();
			println!("Executed ConstraintLayout.observe_constraints()");
		}
		for _i in 0..100{
			let _object137 = gget_constraintlayout();
			println!("Trying to execute ConstraintLayout.observe_guides()");
			_object137.observe_guides();
			println!("Executed ConstraintLayout.observe_guides()");
		}
		for _i in 0..100{
			let _object138 = gget_constraintlayout();
			println!("Trying to execute ConstraintLayout.remove_all_constraints()");
			_object138.remove_all_constraints();
			println!("Executed ConstraintLayout.remove_all_constraints()");
		}
	}
	// CssProvider
	{
		for _i in 0..100{
			let _object140 = gget_cssprovider();
			println!("Trying to execute CssProvider.to_str()");
			_object140.to_str();
			println!("Executed CssProvider.to_str()");
		}
	}
	// Dialog
	{
	}
	// DirectoryList
	{
		for _i in 0..100{
			let _object143 = gget_directorylist();
			println!("Trying to execute DirectoryList.attributes()");
			_object143.attributes();
			println!("Executed DirectoryList.attributes()");
		}
		for _i in 0..100{
			let _object144 = gget_directorylist();
			println!("Trying to execute DirectoryList.error()");
			_object144.error();
			println!("Executed DirectoryList.error()");
		}
		for _i in 0..100{
			let _object145 = gget_directorylist();
			println!("Trying to execute DirectoryList.file()");
			_object145.file();
			println!("Executed DirectoryList.file()");
		}
		for _i in 0..100{
			let _object146 = gget_directorylist();
			println!("Trying to execute DirectoryList.is_loading()");
			_object146.is_loading();
			println!("Executed DirectoryList.is_loading()");
		}
		for _i in 0..100{
			let _object147 = gget_directorylist();
			println!("Trying to execute DirectoryList.is_monitored()");
			_object147.is_monitored();
			println!("Executed DirectoryList.is_monitored()");
		}
	}
	// DragSource
	{
		for _i in 0..100{
			let _object148 = gget_dragsource();
			println!("Trying to execute DragSource.actions()");
			_object148.actions();
			println!("Executed DragSource.actions()");
		}
		for _i in 0..100{
			let _object150 = gget_dragsource();
			println!("Trying to execute DragSource.content()");
			_object150.content();
			println!("Executed DragSource.content()");
		}
		for _i in 0..100{
			let _object151 = gget_dragsource();
			println!("Trying to execute DragSource.drag()");
			_object151.drag();
			println!("Executed DragSource.drag()");
		}
		for _i in 0..100{
			let _object152 = gget_dragsource();
			println!("Trying to execute DragSource.drag_cancel()");
			_object152.drag_cancel();
			println!("Executed DragSource.drag_cancel()");
		}
	}
	// DrawingArea
	{
	}
	// DropControllerMotion
	{
		for _i in 0..100{
			let _object157 = gget_dropcontrollermotion();
			println!("Trying to execute DropControllerMotion.contains_pointer()");
			_object157.contains_pointer();
			println!("Executed DropControllerMotion.contains_pointer()");
		}
		for _i in 0..100{
			let _object158 = gget_dropcontrollermotion();
			println!("Trying to execute DropControllerMotion.drop()");
			_object158.drop();
			println!("Executed DropControllerMotion.drop()");
		}
		for _i in 0..100{
			let _object159 = gget_dropcontrollermotion();
			println!("Trying to execute DropControllerMotion.is_pointer()");
			_object159.is_pointer();
			println!("Executed DropControllerMotion.is_pointer()");
		}
	}
	// DropDown
	{
		for _i in 0..100{
			let _object162 = gget_dropdown();
			println!("Trying to execute DropDown.enables_search()");
			_object162.enables_search();
			println!("Executed DropDown.enables_search()");
		}
		for _i in 0..100{
			let _object163 = gget_dropdown();
			println!("Trying to execute DropDown.model()");
			_object163.model();
			println!("Executed DropDown.model()");
		}
		for _i in 0..100{
			let _object165 = gget_dropdown();
			println!("Trying to execute DropDown.selected_item()");
			_object165.selected_item();
			println!("Executed DropDown.selected_item()");
		}
	}
	// DropTarget
	{
		for _i in 0..100{
			let _object166 = gget_droptarget();
			println!("Trying to execute DropTarget.actions()");
			_object166.actions();
			println!("Executed DropTarget.actions()");
		}
		for _i in 0..100{
			let _object168 = gget_droptarget();
			println!("Trying to execute DropTarget.drop()");
			_object168.drop();
			println!("Executed DropTarget.drop()");
		}
		for _i in 0..100{
			let _object169 = gget_droptarget();
			println!("Trying to execute DropTarget.formats()");
			_object169.formats();
			println!("Executed DropTarget.formats()");
		}
		for _i in 0..100{
			let _object170 = gget_droptarget();
			println!("Trying to execute DropTarget.is_preload()");
			_object170.is_preload();
			println!("Executed DropTarget.is_preload()");
		}
		for _i in 0..100{
			let _object172 = gget_droptarget();
			println!("Trying to execute DropTarget.reject()");
			_object172.reject();
			println!("Executed DropTarget.reject()");
		}
		for _i in 0..100{
			let _object173 = gget_droptarget();
			println!("Trying to execute DropTarget.value()");
			_object173.value();
			println!("Executed DropTarget.value()");
		}
	}
	// EditableLabel
	{
		for _i in 0..100{
			let _object175 = gget_editablelabel();
			println!("Trying to execute EditableLabel.is_editing()");
			_object175.is_editing();
			println!("Executed EditableLabel.is_editing()");
		}
		for _i in 0..100{
			let _object177 = gget_editablelabel();
			println!("Trying to execute EditableLabel.start_editing()");
			_object177.start_editing();
			println!("Executed EditableLabel.start_editing()");
		}
	}
	// EmojiChooser
	{
	}
	// Entry
	{
	}
	// EntryBuffer
	{
	}
	// EntryCompletion
	{
		for _i in 0..100{
			let _object185 = gget_entrycompletion();
			println!("Trying to execute EntryCompletion.complete()");
			_object185.complete();
			println!("Executed EntryCompletion.complete()");
		}
		for _i in 0..100{
			let _object186 = gget_entrycompletion();
			println!("Trying to execute EntryCompletion.completion_prefix()");
			_object186.completion_prefix();
			println!("Executed EntryCompletion.completion_prefix()");
		}
		for _i in 0..100{
			let _object187 = gget_entrycompletion();
			println!("Trying to execute EntryCompletion.is_inline_completion()");
			_object187.is_inline_completion();
			println!("Executed EntryCompletion.is_inline_completion()");
		}
		for _i in 0..100{
			let _object188 = gget_entrycompletion();
			println!("Trying to execute EntryCompletion.is_inline_selection()");
			_object188.is_inline_selection();
			println!("Executed EntryCompletion.is_inline_selection()");
		}
		for _i in 0..100{
			let _object189 = gget_entrycompletion();
			println!("Trying to execute EntryCompletion.is_popup_completion()");
			_object189.is_popup_completion();
			println!("Executed EntryCompletion.is_popup_completion()");
		}
		for _i in 0..100{
			let _object190 = gget_entrycompletion();
			println!("Trying to execute EntryCompletion.is_popup_set_width()");
			_object190.is_popup_set_width();
			println!("Executed EntryCompletion.is_popup_set_width()");
		}
		for _i in 0..100{
			let _object191 = gget_entrycompletion();
			println!("Trying to execute EntryCompletion.is_popup_single_match()");
			_object191.is_popup_single_match();
			println!("Executed EntryCompletion.is_popup_single_match()");
		}
	}
	// EventControllerFocus
	{
		for _i in 0..100{
			let _object194 = gget_eventcontrollerfocus();
			println!("Trying to execute EventControllerFocus.contains_focus()");
			_object194.contains_focus();
			println!("Executed EventControllerFocus.contains_focus()");
		}
		for _i in 0..100{
			let _object195 = gget_eventcontrollerfocus();
			println!("Trying to execute EventControllerFocus.is_focus()");
			_object195.is_focus();
			println!("Executed EventControllerFocus.is_focus()");
		}
	}
	// EventControllerKey
	{
		for _i in 0..100{
			let _object198 = gget_eventcontrollerkey();
			println!("Trying to execute EventControllerKey.group()");
			_object198.group();
			println!("Executed EventControllerKey.group()");
		}
	}
	// EventControllerLegacy
	{
	}
	// EventControllerMotion
	{
		for _i in 0..100{
			let _object203 = gget_eventcontrollermotion();
			println!("Trying to execute EventControllerMotion.contains_pointer()");
			_object203.contains_pointer();
			println!("Executed EventControllerMotion.contains_pointer()");
		}
		for _i in 0..100{
			let _object204 = gget_eventcontrollermotion();
			println!("Trying to execute EventControllerMotion.is_pointer()");
			_object204.is_pointer();
			println!("Executed EventControllerMotion.is_pointer()");
		}
	}
	// EventControllerScroll
	{
	}
	// EveryFilter
	{
	}
	// FileChooserDialog
	{
	}
	// FileChooserNative
	{
	}
	// FileFilter
	{
		for _i in 0..100{
			let _object213 = gget_filefilter();
			println!("Trying to execute FileFilter.add_pixbuf_formats()");
			_object213.add_pixbuf_formats();
			println!("Executed FileFilter.add_pixbuf_formats()");
		}
		for _i in 0..100{
			let _object214 = gget_filefilter();
			println!("Trying to execute FileFilter.attributes()");
			_object214.attributes();
			println!("Executed FileFilter.attributes()");
		}
		for _i in 0..100{
			let _object215 = gget_filefilter();
			println!("Trying to execute FileFilter.name()");
			_object215.name();
			println!("Executed FileFilter.name()");
		}
		for _i in 0..100{
			let _object217 = gget_filefilter();
			println!("Trying to execute FileFilter.to_gvariant()");
			_object217.to_gvariant();
			println!("Executed FileFilter.to_gvariant()");
		}
	}
	// Fixed
	{
	}
	// FixedLayout
	{
	}
	// FlowBox
	{
		for _i in 0..100{
			let _object221 = gget_flowbox();
			println!("Trying to execute FlowBox.accepts_unpaired_release()");
			_object221.accepts_unpaired_release();
			println!("Executed FlowBox.accepts_unpaired_release()");
		}
		for _i in 0..100{
			let _object222 = gget_flowbox();
			println!("Trying to execute FlowBox.activates_on_single_click()");
			_object222.activates_on_single_click();
			println!("Executed FlowBox.activates_on_single_click()");
		}
		for _i in 0..100{
			let _object224 = gget_flowbox();
			println!("Trying to execute FlowBox.invalidate_filter()");
			_object224.invalidate_filter();
			println!("Executed FlowBox.invalidate_filter()");
		}
		for _i in 0..100{
			let _object225 = gget_flowbox();
			println!("Trying to execute FlowBox.invalidate_sort()");
			_object225.invalidate_sort();
			println!("Executed FlowBox.invalidate_sort()");
		}
		for _i in 0..100{
			let _object226 = gget_flowbox();
			println!("Trying to execute FlowBox.is_homogeneous()");
			_object226.is_homogeneous();
			println!("Executed FlowBox.is_homogeneous()");
		}
		for _i in 0..100{
			let _object228 = gget_flowbox();
			println!("Trying to execute FlowBox.select_all()");
			_object228.select_all();
			println!("Executed FlowBox.select_all()");
		}
		for _i in 0..100{
			let _object229 = gget_flowbox();
			println!("Trying to execute FlowBox.selected_children()");
			_object229.selected_children();
			println!("Executed FlowBox.selected_children()");
		}
		for _i in 0..100{
			let _object230 = gget_flowbox();
			println!("Trying to execute FlowBox.unselect_all()");
			_object230.unselect_all();
			println!("Executed FlowBox.unselect_all()");
		}
	}
	// FlowBoxChild
	{
	}
	// FontButton
	{
		for _i in 0..100{
			let _object234 = gget_fontbutton();
			println!("Trying to execute FontButton.is_modal()");
			_object234.is_modal();
			println!("Executed FontButton.is_modal()");
		}
		for _i in 0..100{
			let _object236 = gget_fontbutton();
			println!("Trying to execute FontButton.uses_font()");
			_object236.uses_font();
			println!("Executed FontButton.uses_font()");
		}
		for _i in 0..100{
			let _object237 = gget_fontbutton();
			println!("Trying to execute FontButton.uses_size()");
			_object237.uses_size();
			println!("Executed FontButton.uses_size()");
		}
	}
	// FontChooserDialog
	{
	}
	// FontChooserWidget
	{
		for _i in 0..100{
			let _object242 = gget_fontchooserwidget();
			println!("Trying to execute FontChooserWidget.tweak_action()");
			_object242.tweak_action();
			println!("Executed FontChooserWidget.tweak_action()");
		}
	}
	// Frame
	{
	}
	// GLArea
	{
	}
	// GestureClick
	{
	}
	// GestureDrag
	{
	}
	// GestureLongPress
	{
	}
	// GesturePan
	{
	}
	// GestureRotate
	{
		for _i in 0..100{
			let _object255 = gget_gesturerotate();
			println!("Trying to execute GestureRotate.angle_delta()");
			_object255.angle_delta();
			println!("Executed GestureRotate.angle_delta()");
		}
	}
	// GestureStylus
	{
		for _i in 0..100{
			let _object258 = gget_gesturestylus();
			println!("Trying to execute GestureStylus.backlog()");
			_object258.backlog();
			println!("Executed GestureStylus.backlog()");
		}
		for _i in 0..100{
			let _object260 = gget_gesturestylus();
			println!("Trying to execute GestureStylus.device_tool()");
			_object260.device_tool();
			println!("Executed GestureStylus.device_tool()");
		}
	}
	// GestureSwipe
	{
		for _i in 0..100{
			let _object264 = gget_gestureswipe();
			println!("Trying to execute GestureSwipe.velocity()");
			_object264.velocity();
			println!("Executed GestureSwipe.velocity()");
		}
	}
	// GestureZoom
	{
		for _i in 0..100{
			let _object267 = gget_gesturezoom();
			println!("Trying to execute GestureZoom.scale_delta()");
			_object267.scale_delta();
			println!("Executed GestureZoom.scale_delta()");
		}
	}
	// Grid
	{
	}
	// GridLayout
	{
		for _i in 0..100{
			let _object271 = gget_gridlayout();
			println!("Trying to execute GridLayout.is_column_homogeneous()");
			_object271.is_column_homogeneous();
			println!("Executed GridLayout.is_column_homogeneous()");
		}
		for _i in 0..100{
			let _object272 = gget_gridlayout();
			println!("Trying to execute GridLayout.is_row_homogeneous()");
			_object272.is_row_homogeneous();
			println!("Executed GridLayout.is_row_homogeneous()");
		}
	}
	// HeaderBar
	{
		for _i in 0..100{
			let _object276 = gget_headerbar();
			println!("Trying to execute HeaderBar.shows_title_buttons()");
			_object276.shows_title_buttons();
			println!("Executed HeaderBar.shows_title_buttons()");
		}
	}
	// IMContextSimple
	{
	}
	// IMMulticontext
	{
	}
	// IconTheme
	{
		for _i in 0..100{
			let _object282 = gget_icontheme();
			println!("Trying to execute IconTheme.display()");
			_object282.display();
			println!("Executed IconTheme.display()");
		}
		for _i in 0..100{
			let _object283 = gget_icontheme();
			println!("Trying to execute IconTheme.icon_names()");
			_object283.icon_names();
			println!("Executed IconTheme.icon_names()");
		}
	}
	// IconView
	{
		for _i in 0..100{
			let _object285 = gget_iconview();
			println!("Trying to execute IconView.activates_on_single_click()");
			_object285.activates_on_single_click();
			println!("Executed IconView.activates_on_single_click()");
		}
		for _i in 0..100{
			let _object287 = gget_iconview();
			println!("Trying to execute IconView.cursor()");
			_object287.cursor();
			println!("Executed IconView.cursor()");
		}
		for _i in 0..100{
			let _object288 = gget_iconview();
			println!("Trying to execute IconView.is_reorderable()");
			_object288.is_reorderable();
			println!("Executed IconView.is_reorderable()");
		}
		for _i in 0..100{
			let _object290 = gget_iconview();
			println!("Trying to execute IconView.select_all()");
			_object290.select_all();
			println!("Executed IconView.select_all()");
		}
		for _i in 0..100{
			let _object291 = gget_iconview();
			println!("Trying to execute IconView.selected_items()");
			_object291.selected_items();
			println!("Executed IconView.selected_items()");
		}
		for _i in 0..100{
			let _object292 = gget_iconview();
			println!("Trying to execute IconView.unselect_all()");
			_object292.unselect_all();
			println!("Executed IconView.unselect_all()");
		}
		for _i in 0..100{
			let _object293 = gget_iconview();
			println!("Trying to execute IconView.unset_model_drag_dest()");
			_object293.unset_model_drag_dest();
			println!("Executed IconView.unset_model_drag_dest()");
		}
		for _i in 0..100{
			let _object294 = gget_iconview();
			println!("Trying to execute IconView.unset_model_drag_source()");
			_object294.unset_model_drag_source();
			println!("Executed IconView.unset_model_drag_source()");
		}
		for _i in 0..100{
			let _object295 = gget_iconview();
			println!("Trying to execute IconView.visible_range()");
			_object295.visible_range();
			println!("Executed IconView.visible_range()");
		}
	}
	// Image
	{
		for _i in 0..100{
			let _object297 = gget_image();
			println!("Trying to execute Image.clear()");
			_object297.clear();
			println!("Executed Image.clear()");
		}
		for _i in 0..100{
			let _object298 = gget_image();
			println!("Trying to execute Image.gicon()");
			_object298.gicon();
			println!("Executed Image.gicon()");
		}
		for _i in 0..100{
			let _object300 = gget_image();
			println!("Trying to execute Image.paintable()");
			_object300.paintable();
			println!("Executed Image.paintable()");
		}
		for _i in 0..100{
			let _object301 = gget_image();
			println!("Trying to execute Image.storage_type()");
			_object301.storage_type();
			println!("Executed Image.storage_type()");
		}
		for _i in 0..100{
			let _object302 = gget_image();
			println!("Trying to execute Image.uses_fallback()");
			_object302.uses_fallback();
			println!("Executed Image.uses_fallback()");
		}
	}
	// InfoBar
	{
		for _i in 0..100{
			let _object304 = gget_infobar();
			println!("Trying to execute InfoBar.is_revealed()");
			_object304.is_revealed();
			println!("Executed InfoBar.is_revealed()");
		}
		for _i in 0..100{
			let _object306 = gget_infobar();
			println!("Trying to execute InfoBar.shows_close_button()");
			_object306.shows_close_button();
			println!("Executed InfoBar.shows_close_button()");
		}
	}
	// LevelBar
	{
		for _i in 0..100{
			let _object308 = gget_levelbar();
			println!("Trying to execute LevelBar.is_inverted()");
			_object308.is_inverted();
			println!("Executed LevelBar.is_inverted()");
		}
	}
	// ListBox
	{
		for _i in 0..100{
			let _object310 = gget_listbox();
			println!("Trying to execute ListBox.accepts_unpaired_release()");
			_object310.accepts_unpaired_release();
			println!("Executed ListBox.accepts_unpaired_release()");
		}
		for _i in 0..100{
			let _object311 = gget_listbox();
			println!("Trying to execute ListBox.activates_on_single_click()");
			_object311.activates_on_single_click();
			println!("Executed ListBox.activates_on_single_click()");
		}
		for _i in 0..100{
			let _object312 = gget_listbox();
			println!("Trying to execute ListBox.adjustment()");
			_object312.adjustment();
			println!("Executed ListBox.adjustment()");
		}
		for _i in 0..100{
			let _object314 = gget_listbox();
			println!("Trying to execute ListBox.drag_unhighlight_row()");
			_object314.drag_unhighlight_row();
			println!("Executed ListBox.drag_unhighlight_row()");
		}
		for _i in 0..100{
			let _object315 = gget_listbox();
			println!("Trying to execute ListBox.invalidate_filter()");
			_object315.invalidate_filter();
			println!("Executed ListBox.invalidate_filter()");
		}
		for _i in 0..100{
			let _object316 = gget_listbox();
			println!("Trying to execute ListBox.invalidate_headers()");
			_object316.invalidate_headers();
			println!("Executed ListBox.invalidate_headers()");
		}
		for _i in 0..100{
			let _object317 = gget_listbox();
			println!("Trying to execute ListBox.invalidate_sort()");
			_object317.invalidate_sort();
			println!("Executed ListBox.invalidate_sort()");
		}
		for _i in 0..100{
			let _object319 = gget_listbox();
			println!("Trying to execute ListBox.select_all()");
			_object319.select_all();
			println!("Executed ListBox.select_all()");
		}
		for _i in 0..100{
			let _object320 = gget_listbox();
			println!("Trying to execute ListBox.selected_row()");
			_object320.selected_row();
			println!("Executed ListBox.selected_row()");
		}
		for _i in 0..100{
			let _object321 = gget_listbox();
			println!("Trying to execute ListBox.selected_rows()");
			_object321.selected_rows();
			println!("Executed ListBox.selected_rows()");
		}
		for _i in 0..100{
			let _object322 = gget_listbox();
			println!("Trying to execute ListBox.shows_separators()");
			_object322.shows_separators();
			println!("Executed ListBox.shows_separators()");
		}
		for _i in 0..100{
			let _object323 = gget_listbox();
			println!("Trying to execute ListBox.unselect_all()");
			_object323.unselect_all();
			println!("Executed ListBox.unselect_all()");
		}
	}
	// ListBoxRow
	{
	}
	// MediaControls
	{
	}
	// MediaFile
	{
	}
	// MenuButton
	{
		for _i in 0..100{
			let _object330 = gget_menubutton();
			println!("Trying to execute MenuButton.menu_model()");
			_object330.menu_model();
			println!("Executed MenuButton.menu_model()");
		}
		for _i in 0..100{
			let _object332 = gget_menubutton();
			println!("Trying to execute MenuButton.popdown()");
			_object332.popdown();
			println!("Executed MenuButton.popdown()");
		}
		for _i in 0..100{
			let _object333 = gget_menubutton();
			println!("Trying to execute MenuButton.popup()");
			_object333.popup();
			println!("Executed MenuButton.popup()");
		}
		for _i in 0..100{
			let _object334 = gget_menubutton();
			println!("Trying to execute MenuButton.uses_underline()");
			_object334.uses_underline();
			println!("Executed MenuButton.uses_underline()");
		}
	}
	// MountOperation
	{
	}
	// MultiSorter
	{
	}
	// Notebook
	{
		for _i in 0..100{
			let _object339 = gget_notebook();
			println!("Trying to execute Notebook.enables_popup()");
			_object339.enables_popup();
			println!("Executed Notebook.enables_popup()");
		}
		for _i in 0..100{
			let _object340 = gget_notebook();
			println!("Trying to execute Notebook.is_scrollable()");
			_object340.is_scrollable();
			println!("Executed Notebook.is_scrollable()");
		}
		for _i in 0..100{
			let _object342 = gget_notebook();
			println!("Trying to execute Notebook.next_page()");
			_object342.next_page();
			println!("Executed Notebook.next_page()");
		}
		for _i in 0..100{
			let _object343 = gget_notebook();
			println!("Trying to execute Notebook.pages()");
			_object343.pages();
			println!("Executed Notebook.pages()");
		}
		for _i in 0..100{
			let _object344 = gget_notebook();
			println!("Trying to execute Notebook.popup_disable()");
			_object344.popup_disable();
			println!("Executed Notebook.popup_disable()");
		}
		for _i in 0..100{
			let _object345 = gget_notebook();
			println!("Trying to execute Notebook.popup_enable()");
			_object345.popup_enable();
			println!("Executed Notebook.popup_enable()");
		}
		for _i in 0..100{
			let _object346 = gget_notebook();
			println!("Trying to execute Notebook.prev_page()");
			_object346.prev_page();
			println!("Executed Notebook.prev_page()");
		}
		for _i in 0..100{
			let _object347 = gget_notebook();
			println!("Trying to execute Notebook.shows_border()");
			_object347.shows_border();
			println!("Executed Notebook.shows_border()");
		}
		for _i in 0..100{
			let _object348 = gget_notebook();
			println!("Trying to execute Notebook.shows_tabs()");
			_object348.shows_tabs();
			println!("Executed Notebook.shows_tabs()");
		}
	}
	// NumericSorter
	{
	}
	// Overlay
	{
	}
	// OverlayLayout
	{
	}
	// PageSetup
	{
		for _i in 0..100{
			let _object354 = gget_pagesetup();
			println!("Trying to execute PageSetup.copy()");
			_object354.copy();
			println!("Executed PageSetup.copy()");
		}
		for _i in 0..100{
			let _object356 = gget_pagesetup();
			println!("Trying to execute PageSetup.orientation()");
			_object356.orientation();
			println!("Executed PageSetup.orientation()");
		}
		for _i in 0..100{
			let _object357 = gget_pagesetup();
			println!("Trying to execute PageSetup.paper_size()");
			_object357.paper_size();
			println!("Executed PageSetup.paper_size()");
		}
		for _i in 0..100{
			let _object358 = gget_pagesetup();
			println!("Trying to execute PageSetup.to_gvariant()");
			_object358.to_gvariant();
			println!("Executed PageSetup.to_gvariant()");
		}
	}
	// PasswordEntry
	{
		for _i in 0..100{
			let _object360 = gget_passwordentry();
			println!("Trying to execute PasswordEntry.extra_menu()");
			_object360.extra_menu();
			println!("Executed PasswordEntry.extra_menu()");
		}
		for _i in 0..100{
			let _object362 = gget_passwordentry();
			println!("Trying to execute PasswordEntry.shows_peek_icon()");
			_object362.shows_peek_icon();
			println!("Executed PasswordEntry.shows_peek_icon()");
		}
	}
	// PasswordEntryBuffer
	{
	}
	// Picture
	{
		for _i in 0..100{
			let _object366 = gget_picture();
			println!("Trying to execute Picture.file()");
			_object366.file();
			println!("Executed Picture.file()");
		}
		for _i in 0..100{
			let _object367 = gget_picture();
			println!("Trying to execute Picture.is_keep_aspect_ratio()");
			_object367.is_keep_aspect_ratio();
			println!("Executed Picture.is_keep_aspect_ratio()");
		}
		for _i in 0..100{
			let _object369 = gget_picture();
			println!("Trying to execute Picture.paintable()");
			_object369.paintable();
			println!("Executed Picture.paintable()");
		}
	}
	// Popover
	{
	}
	// PrintOperation
	{
	}
	// PrintSettings
	{
		for _i in 0..100{
			let _object374 = gget_printsettings();
			println!("Trying to execute PrintSettings.copy()");
			_object374.copy();
			println!("Executed PrintSettings.copy()");
		}
		for _i in 0..100{
			let _object375 = gget_printsettings();
			println!("Trying to execute PrintSettings.default_source()");
			_object375.default_source();
			println!("Executed PrintSettings.default_source()");
		}
		for _i in 0..100{
			let _object376 = gget_printsettings();
			println!("Trying to execute PrintSettings.dither()");
			_object376.dither();
			println!("Executed PrintSettings.dither()");
		}
		for _i in 0..100{
			let _object377 = gget_printsettings();
			println!("Trying to execute PrintSettings.duplex()");
			_object377.duplex();
			println!("Executed PrintSettings.duplex()");
		}
		for _i in 0..100{
			let _object378 = gget_printsettings();
			println!("Trying to execute PrintSettings.finishings()");
			_object378.finishings();
			println!("Executed PrintSettings.finishings()");
		}
		for _i in 0..100{
			let _object379 = gget_printsettings();
			println!("Trying to execute PrintSettings.is_collate()");
			_object379.is_collate();
			println!("Executed PrintSettings.is_collate()");
		}
		for _i in 0..100{
			let _object380 = gget_printsettings();
			println!("Trying to execute PrintSettings.is_reverse()");
			_object380.is_reverse();
			println!("Executed PrintSettings.is_reverse()");
		}
		for _i in 0..100{
			let _object381 = gget_printsettings();
			println!("Trying to execute PrintSettings.media_type()");
			_object381.media_type();
			println!("Executed PrintSettings.media_type()");
		}
		for _i in 0..100{
			let _object382 = gget_printsettings();
			println!("Trying to execute PrintSettings.n_copies()");
			_object382.n_copies();
			println!("Executed PrintSettings.n_copies()");
		}
		for _i in 0..100{
			let _object384 = gget_printsettings();
			println!("Trying to execute PrintSettings.number_up()");
			_object384.number_up();
			println!("Executed PrintSettings.number_up()");
		}
		for _i in 0..100{
			let _object385 = gget_printsettings();
			println!("Trying to execute PrintSettings.number_up_layout()");
			_object385.number_up_layout();
			println!("Executed PrintSettings.number_up_layout()");
		}
		for _i in 0..100{
			let _object386 = gget_printsettings();
			println!("Trying to execute PrintSettings.orientation()");
			_object386.orientation();
			println!("Executed PrintSettings.orientation()");
		}
		for _i in 0..100{
			let _object387 = gget_printsettings();
			println!("Trying to execute PrintSettings.output_bin()");
			_object387.output_bin();
			println!("Executed PrintSettings.output_bin()");
		}
		for _i in 0..100{
			let _object388 = gget_printsettings();
			println!("Trying to execute PrintSettings.page_ranges()");
			_object388.page_ranges();
			println!("Executed PrintSettings.page_ranges()");
		}
		for _i in 0..100{
			let _object389 = gget_printsettings();
			println!("Trying to execute PrintSettings.page_set()");
			_object389.page_set();
			println!("Executed PrintSettings.page_set()");
		}
		for _i in 0..100{
			let _object390 = gget_printsettings();
			println!("Trying to execute PrintSettings.paper_size()");
			_object390.paper_size();
			println!("Executed PrintSettings.paper_size()");
		}
		for _i in 0..100{
			let _object391 = gget_printsettings();
			println!("Trying to execute PrintSettings.print_pages()");
			_object391.print_pages();
			println!("Executed PrintSettings.print_pages()");
		}
		for _i in 0..100{
			let _object392 = gget_printsettings();
			println!("Trying to execute PrintSettings.printer()");
			_object392.printer();
			println!("Executed PrintSettings.printer()");
		}
		for _i in 0..100{
			let _object393 = gget_printsettings();
			println!("Trying to execute PrintSettings.printer_lpi()");
			_object393.printer_lpi();
			println!("Executed PrintSettings.printer_lpi()");
		}
		for _i in 0..100{
			let _object394 = gget_printsettings();
			println!("Trying to execute PrintSettings.quality()");
			_object394.quality();
			println!("Executed PrintSettings.quality()");
		}
		for _i in 0..100{
			let _object395 = gget_printsettings();
			println!("Trying to execute PrintSettings.resolution()");
			_object395.resolution();
			println!("Executed PrintSettings.resolution()");
		}
		for _i in 0..100{
			let _object396 = gget_printsettings();
			println!("Trying to execute PrintSettings.resolution_x()");
			_object396.resolution_x();
			println!("Executed PrintSettings.resolution_x()");
		}
		for _i in 0..100{
			let _object397 = gget_printsettings();
			println!("Trying to execute PrintSettings.resolution_y()");
			_object397.resolution_y();
			println!("Executed PrintSettings.resolution_y()");
		}
		for _i in 0..100{
			let _object398 = gget_printsettings();
			println!("Trying to execute PrintSettings.scale()");
			_object398.scale();
			println!("Executed PrintSettings.scale()");
		}
		for _i in 0..100{
			let _object399 = gget_printsettings();
			println!("Trying to execute PrintSettings.to_gvariant()");
			_object399.to_gvariant();
			println!("Executed PrintSettings.to_gvariant()");
		}
		for _i in 0..100{
			let _object400 = gget_printsettings();
			println!("Trying to execute PrintSettings.uses_color()");
			_object400.uses_color();
			println!("Executed PrintSettings.uses_color()");
		}
	}
	// ProgressBar
	{
		for _i in 0..100{
			let _object402 = gget_progressbar();
			println!("Trying to execute ProgressBar.ellipsize()");
			_object402.ellipsize();
			println!("Executed ProgressBar.ellipsize()");
		}
		for _i in 0..100{
			let _object403 = gget_progressbar();
			println!("Trying to execute ProgressBar.is_inverted()");
			_object403.is_inverted();
			println!("Executed ProgressBar.is_inverted()");
		}
		for _i in 0..100{
			let _object405 = gget_progressbar();
			println!("Trying to execute ProgressBar.pulse()");
			_object405.pulse();
			println!("Executed ProgressBar.pulse()");
		}
		for _i in 0..100{
			let _object406 = gget_progressbar();
			println!("Trying to execute ProgressBar.shows_text()");
			_object406.shows_text();
			println!("Executed ProgressBar.shows_text()");
		}
	}
	// RecentManager
	{
	}
	// Revealer
	{
		for _i in 0..100{
			let _object409 = gget_revealer();
			println!("Trying to execute Revealer.is_child_revealed()");
			_object409.is_child_revealed();
			println!("Executed Revealer.is_child_revealed()");
		}
		for _i in 0..100{
			let _object411 = gget_revealer();
			println!("Trying to execute Revealer.reveals_child()");
			_object411.reveals_child();
			println!("Executed Revealer.reveals_child()");
		}
	}
	// Scale
	{
	}
	// ScaleButton
	{
	}
	// Scrollbar
	{
	}
	// ScrolledWindow
	{
		for _i in 0..100{
			let _object419 = gget_scrolledwindow();
			println!("Trying to execute ScrolledWindow.hscrollbar()");
			_object419.hscrollbar();
			println!("Executed ScrolledWindow.hscrollbar()");
		}
		for _i in 0..100{
			let _object420 = gget_scrolledwindow();
			println!("Trying to execute ScrolledWindow.is_kinetic_scrolling()");
			_object420.is_kinetic_scrolling();
			println!("Executed ScrolledWindow.is_kinetic_scrolling()");
		}
		for _i in 0..100{
			let _object421 = gget_scrolledwindow();
			println!("Trying to execute ScrolledWindow.is_overlay_scrolling()");
			_object421.is_overlay_scrolling();
			println!("Executed ScrolledWindow.is_overlay_scrolling()");
		}
		for _i in 0..100{
			let _object423 = gget_scrolledwindow();
			println!("Trying to execute ScrolledWindow.placement()");
			_object423.placement();
			println!("Executed ScrolledWindow.placement()");
		}
		for _i in 0..100{
			let _object424 = gget_scrolledwindow();
			println!("Trying to execute ScrolledWindow.policy()");
			_object424.policy();
			println!("Executed ScrolledWindow.policy()");
		}
		for _i in 0..100{
			let _object425 = gget_scrolledwindow();
			println!("Trying to execute ScrolledWindow.propagates_natural_height()");
			_object425.propagates_natural_height();
			println!("Executed ScrolledWindow.propagates_natural_height()");
		}
		for _i in 0..100{
			let _object426 = gget_scrolledwindow();
			println!("Trying to execute ScrolledWindow.propagates_natural_width()");
			_object426.propagates_natural_width();
			println!("Executed ScrolledWindow.propagates_natural_width()");
		}
		for _i in 0..100{
			let _object427 = gget_scrolledwindow();
			println!("Trying to execute ScrolledWindow.unset_placement()");
			_object427.unset_placement();
			println!("Executed ScrolledWindow.unset_placement()");
		}
		for _i in 0..100{
			let _object428 = gget_scrolledwindow();
			println!("Trying to execute ScrolledWindow.vscrollbar()");
			_object428.vscrollbar();
			println!("Executed ScrolledWindow.vscrollbar()");
		}
	}
	// SearchBar
	{
		for _i in 0..100{
			let _object430 = gget_searchbar();
			println!("Trying to execute SearchBar.is_search_mode()");
			_object430.is_search_mode();
			println!("Executed SearchBar.is_search_mode()");
		}
		for _i in 0..100{
			let _object432 = gget_searchbar();
			println!("Trying to execute SearchBar.shows_close_button()");
			_object432.shows_close_button();
			println!("Executed SearchBar.shows_close_button()");
		}
	}
	// SearchEntry
	{
		for _i in 0..100{
			let _object434 = gget_searchentry();
			println!("Trying to execute SearchEntry.key_capture_widget()");
			_object434.key_capture_widget();
			println!("Executed SearchEntry.key_capture_widget()");
		}
	}
	// Separator
	{
	}
	// ShortcutAction
	{
	}
	// ShortcutController
	{
		for _i in 0..100{
			let _object439 = gget_shortcutcontroller();
			println!("Trying to execute ShortcutController.mnemonic_modifiers()");
			_object439.mnemonic_modifiers();
			println!("Executed ShortcutController.mnemonic_modifiers()");
		}
		for _i in 0..100{
			let _object440 = gget_shortcutcontroller();
			println!("Trying to execute ShortcutController.mnemonics_modifiers()");
			_object440.mnemonics_modifiers();
			println!("Executed ShortcutController.mnemonics_modifiers()");
		}
	}
	// ShortcutLabel
	{
	}
	// ShortcutTrigger
	{
	}
	// ShortcutsSection
	{
	}
	// SignalListItemFactory
	{
	}
	// Snapshot
	{
		for _i in 0..100{
			let _object447 = gget_snapshot();
			println!("Trying to execute Snapshot.gl_shader_pop_texture()");
			_object447.gl_shader_pop_texture();
			println!("Executed Snapshot.gl_shader_pop_texture()");
		}
		for _i in 0..100{
			let _object449 = gget_snapshot();
			println!("Trying to execute Snapshot.pop()");
			_object449.pop();
			println!("Executed Snapshot.pop()");
		}
		for _i in 0..100{
			let _object450 = gget_snapshot();
			println!("Trying to execute Snapshot.restore()");
			_object450.restore();
			println!("Executed Snapshot.restore()");
		}
		for _i in 0..100{
			let _object451 = gget_snapshot();
			println!("Trying to execute Snapshot.save()");
			_object451.save();
			println!("Executed Snapshot.save()");
		}
		for _i in 0..100{
			let _object452 = gget_snapshot();
			println!("Trying to execute Snapshot.to_node()");
			_object452.to_node();
			println!("Executed Snapshot.to_node()");
		}
	}
	// Spinner
	{
		for _i in 0..100{
			let _object454 = gget_spinner();
			println!("Trying to execute Spinner.is_spinning()");
			_object454.is_spinning();
			println!("Executed Spinner.is_spinning()");
		}
		for _i in 0..100{
			let _object456 = gget_spinner();
			println!("Trying to execute Spinner.start()");
			_object456.start();
			println!("Executed Spinner.start()");
		}
		for _i in 0..100{
			let _object457 = gget_spinner();
			println!("Trying to execute Spinner.stop()");
			_object457.stop();
			println!("Executed Spinner.stop()");
		}
	}
	// Stack
	{
		for _i in 0..100{
			let _object459 = gget_stack();
			println!("Trying to execute Stack.interpolates_size()");
			_object459.interpolates_size();
			println!("Executed Stack.interpolates_size()");
		}
		for _i in 0..100{
			let _object460 = gget_stack();
			println!("Trying to execute Stack.is_hhomogeneous()");
			_object460.is_hhomogeneous();
			println!("Executed Stack.is_hhomogeneous()");
		}
		for _i in 0..100{
			let _object461 = gget_stack();
			println!("Trying to execute Stack.is_transition_running()");
			_object461.is_transition_running();
			println!("Executed Stack.is_transition_running()");
		}
		for _i in 0..100{
			let _object462 = gget_stack();
			println!("Trying to execute Stack.is_vhomogeneous()");
			_object462.is_vhomogeneous();
			println!("Executed Stack.is_vhomogeneous()");
		}
		for _i in 0..100{
			let _object464 = gget_stack();
			println!("Trying to execute Stack.pages()");
			_object464.pages();
			println!("Executed Stack.pages()");
		}
	}
	// StackSidebar
	{
	}
	// StackSwitcher
	{
	}
	// Statusbar
	{
	}
	// StringList
	{
	}
	// Switch
	{
		for _i in 0..100{
			let _object472 = gget_switch();
			println!("Trying to execute Switch.is_active()");
			_object472.is_active();
			println!("Executed Switch.is_active()");
		}
	}
	// Text
	{
		for _i in 0..100{
			let _object474 = gget_text();
			println!("Trying to execute Text.attributes()");
			_object474.attributes();
			println!("Executed Text.attributes()");
		}
		for _i in 0..100{
			let _object476 = gget_text();
			println!("Trying to execute Text.enables_emoji_completion()");
			_object476.enables_emoji_completion();
			println!("Executed Text.enables_emoji_completion()");
		}
		for _i in 0..100{
			let _object477 = gget_text();
			println!("Trying to execute Text.extra_menu()");
			_object477.extra_menu();
			println!("Executed Text.extra_menu()");
		}
		for _i in 0..100{
			let _object478 = gget_text();
			println!("Trying to execute Text.grab_focus_without_selecting()");
			_object478.grab_focus_without_selecting();
			println!("Executed Text.grab_focus_without_selecting()");
		}
		for _i in 0..100{
			let _object479 = gget_text();
			println!("Trying to execute Text.is_invisible_char_set()");
			_object479.is_invisible_char_set();
			println!("Executed Text.is_invisible_char_set()");
		}
		for _i in 0..100{
			let _object480 = gget_text();
			println!("Trying to execute Text.is_overwrite_mode()");
			_object480.is_overwrite_mode();
			println!("Executed Text.is_overwrite_mode()");
		}
		for _i in 0..100{
			let _object481 = gget_text();
			println!("Trying to execute Text.is_visible()");
			_object481.is_visible();
			println!("Executed Text.is_visible()");
		}
		for _i in 0..100{
			let _object482 = gget_text();
			println!("Trying to execute Text.must_truncate_multiline()");
			_object482.must_truncate_multiline();
			println!("Executed Text.must_truncate_multiline()");
		}
		for _i in 0..100{
			let _object484 = gget_text();
			println!("Trying to execute Text.propagates_text_width()");
			_object484.propagates_text_width();
			println!("Executed Text.propagates_text_width()");
		}
		for _i in 0..100{
			let _object485 = gget_text();
			println!("Trying to execute Text.scroll_offset()");
			_object485.scroll_offset();
			println!("Executed Text.scroll_offset()");
		}
		for _i in 0..100{
			let _object486 = gget_text();
			println!("Trying to execute Text.tabs()");
			_object486.tabs();
			println!("Executed Text.tabs()");
		}
		for _i in 0..100{
			let _object487 = gget_text();
			println!("Trying to execute Text.text_length()");
			_object487.text_length();
			println!("Executed Text.text_length()");
		}
		for _i in 0..100{
			let _object488 = gget_text();
			println!("Trying to execute Text.unset_invisible_char()");
			_object488.unset_invisible_char();
			println!("Executed Text.unset_invisible_char()");
		}
	}
	// TextBuffer
	{
	}
	// TextChildAnchor
	{
	}
	// TextMark
	{
	}
	// TextTag
	{
	}
	// TextTagTable
	{
		for _i in 0..100{
			let _object497 = gget_texttagtable();
			println!("Trying to execute TextTagTable.size()");
			_object497.size();
			println!("Executed TextTagTable.size()");
		}
	}
	// TextView
	{
	}
	// ToggleButton
	{
	}
	// Tooltip
	{
	}
	// TreeExpander
	{
		for _i in 0..100{
			let _object503 = gget_treeexpander();
			println!("Trying to execute TreeExpander.item()");
			_object503.item();
			println!("Executed TreeExpander.item()");
		}
	}
	// TreeModelSort
	{
	}
	// TreeView
	{
	}
	// TreeViewColumn
	{
		for _i in 0..100{
			let _object508 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.button()");
			_object508.button();
			println!("Executed TreeViewColumn.button()");
		}
		for _i in 0..100{
			let _object509 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.cell_get_size()");
			_object509.cell_get_size();
			println!("Executed TreeViewColumn.cell_get_size()");
		}
		for _i in 0..100{
			let _object510 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.cell_is_visible()");
			_object510.cell_is_visible();
			println!("Executed TreeViewColumn.cell_is_visible()");
		}
		for _i in 0..100{
			let _object511 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.clicked()");
			_object511.clicked();
			println!("Executed TreeViewColumn.clicked()");
		}
		for _i in 0..100{
			let _object512 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.expands()");
			_object512.expands();
			println!("Executed TreeViewColumn.expands()");
		}
		for _i in 0..100{
			let _object513 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.is_clickable()");
			_object513.is_clickable();
			println!("Executed TreeViewColumn.is_clickable()");
		}
		for _i in 0..100{
			let _object514 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.is_reorderable()");
			_object514.is_reorderable();
			println!("Executed TreeViewColumn.is_reorderable()");
		}
		for _i in 0..100{
			let _object515 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.is_resizable()");
			_object515.is_resizable();
			println!("Executed TreeViewColumn.is_resizable()");
		}
		for _i in 0..100{
			let _object516 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.is_sort_indicator()");
			_object516.is_sort_indicator();
			println!("Executed TreeViewColumn.is_sort_indicator()");
		}
		for _i in 0..100{
			let _object517 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.is_visible()");
			_object517.is_visible();
			println!("Executed TreeViewColumn.is_visible()");
		}
		for _i in 0..100{
			let _object519 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.queue_resize()");
			_object519.queue_resize();
			println!("Executed TreeViewColumn.queue_resize()");
		}
		for _i in 0..100{
			let _object520 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.tree_view()");
			_object520.tree_view();
			println!("Executed TreeViewColumn.tree_view()");
		}
		for _i in 0..100{
			let _object521 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.width()");
			_object521.width();
			println!("Executed TreeViewColumn.width()");
		}
		for _i in 0..100{
			let _object522 = gget_treeviewcolumn();
			println!("Trying to execute TreeViewColumn.x_offset()");
			_object522.x_offset();
			println!("Executed TreeViewColumn.x_offset()");
		}
	}
	// Video
	{
		for _i in 0..100{
			let _object524 = gget_video();
			println!("Trying to execute Video.file()");
			_object524.file();
			println!("Executed Video.file()");
		}
		for _i in 0..100{
			let _object525 = gget_video();
			println!("Trying to execute Video.is_autoplay()");
			_object525.is_autoplay();
			println!("Executed Video.is_autoplay()");
		}
		for _i in 0..100{
			let _object526 = gget_video();
			println!("Trying to execute Video.is_loop()");
			_object526.is_loop();
			println!("Executed Video.is_loop()");
		}
	}
	// VolumeButton
	{
		for _i in 0..100{
			let _object530 = gget_volumebutton();
			println!("Trying to execute VolumeButton.uses_symbolic()");
			_object530.uses_symbolic();
			println!("Executed VolumeButton.uses_symbolic()");
		}
	}
	// Window
	{
	}
	// WindowGroup
	{
	}
	// WindowHandle
	{
	}
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
pub fn stek_glib_type() -> glib::Type {
    [glib::Type::BOOL].choose(&mut rand::thread_rng()).unwrap().clone()
}

//// Too hard
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
// // TOO HARD
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
// pub fn gget_droptargetasync() -> DropTargetAsync {
//     DropTargetAsync::new()
// }
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

