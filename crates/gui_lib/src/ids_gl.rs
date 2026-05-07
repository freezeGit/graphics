//! ## Module ids contains IDs for all the widgets and dialogs
//!
//! Each Id struct identifies a type of widget or dialog, and a specific instantiation.
//! For example ButtonId(pub WidgetId) identifies a widget of type button, and the specific
//! button is labeled by the WidgetId.

// src/gui_lib/ids_gl

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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RadioBoxesDlgId(pub DialogId);
