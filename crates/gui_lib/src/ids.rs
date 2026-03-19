//! ## Module ids contains IDs for all the widgets and dialogs
//! ## and enum WidgetMsg
// src/gui_lib/ids.rs

// WidgetId - unique ID for each widget.
pub type WidgetId = u32;

// Widget IDs -------------------
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonId(pub WidgetId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SliderId(pub WidgetId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragFloatId(pub WidgetId);

// DialogId - unique ID for each dialog.
pub type DialogId = u32;

// Dialog IDs ------------------

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageBoxDlgId(pub DialogId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextEntryDlgId(pub DialogId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragFloatDlgId(pub DialogId);

// ----------------------------------
// WidgetMsg - enum for all messages emitted from widgets and dialogs.
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetMsg {
    // Widget outcomes:
    ButtonClicked(ButtonId),
    SliderChanged(SliderId, f32),
    DragFloatChanged(DragFloatId, f32),

    // Dialog outcomes:
    DialogAcceptedText(TextEntryDlgId, String),
    DialogAcceptedDragFloat(DragFloatDlgId, f32),
}
