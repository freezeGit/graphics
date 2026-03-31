//! ## Module messages contains enum WidgetMsg for emitted mesages

// messages.rs

use crate::ids::{
    ButtonId, DragFloatDlgId, DragFloatId, MultiTextEntryDlgId, SliderId, TextEntryDlgId,
};

/// WidgetMsg - enum for all messages emitted from widgets and dialogs.
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetMsg {
    // Widget outcomes:
    ButtonClicked(ButtonId),
    SliderChanged(SliderId, f32),
    DragFloatChanged(DragFloatId, f32),

    // Dialog outcomes:
    DialogAcceptedText(TextEntryDlgId, String),
    DialogAcceptedMultiTextEntry(MultiTextEntryDlgId, Vec<(String, String)>),
    DialogAcceptedDragFloat(DragFloatDlgId, f32),
}
