//! ## Module ids contains IDs for all the widgets and dialogs
//!
//! Each Id struct identifies a type of widget or dialog, and a specific instantiation.
//! For example ButtonId(pub WidgetId) identifies a widget of type button, and the specific
//! button is labeled by the WidgetId.

// src/gui_lib/ids.rs

// ------------ Widget IDs -------------------
// WidgetId - unique ID for each widget.
pub type WidgetId = u32;

// ---------------------------
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonId(pub WidgetId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SliderId(pub WidgetId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragFloatId(pub WidgetId);

//---------------- Dialog IDs ------------------
// DialogId - unique ID for each dialog.
pub type DialogId = u32;
// --------------------------------------

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageBoxDlgId(pub DialogId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextEntryDlgId(pub DialogId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragFloatDlgId(pub DialogId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MultiTextEntryDlgId(pub DialogId);


// ----------------------------------
// /// WidgetMsg - enum for all messages emitted from widgets and dialogs.
// #[derive(Debug, Clone, PartialEq)]
// pub enum WidgetMsg {
//     // Widget outcomes:
//     ButtonClicked(ButtonId),
//     SliderChanged(SliderId, f32),
//     DragFloatChanged(DragFloatId, f32),
//
//     // Dialog outcomes:
//     DialogAcceptedText(TextEntryDlgId, String),
//     DialogAcceptedDragFloat(DragFloatDlgId, f32),
//     DialogAcceptedMultiTextEntry(MultiTextEntryDlgId, Vec<TextEntryResult>),
// }
