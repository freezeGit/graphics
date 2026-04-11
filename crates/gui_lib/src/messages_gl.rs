//! ## Module messages contains enum WidgetMsg for emitted mesages

// messages_gl

use crate::*;
/// WidgetMsg - enum for all messages emitted from widgets and dialogs.
///
/// Each WidgetMsg variant carries associated data:
///
/// The Id identifies the type of widget or dialog, and also the specific instantiation.
///
/// In addition, messages other than `ButtonClicked` carry additional data that is specific
/// to the particular widget or dialog.
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
