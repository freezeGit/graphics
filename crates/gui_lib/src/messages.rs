// messages.rs

use crate::ids::{
    ButtonId, DragFloatDlgId, DragFloatId, MultiTextEntryDlgId, SliderId,
    TextEntryDlgId,
};
//use crate::TextEntryResult;
//use crate::ids::{ButtonId, DragFloatId, SliderId};

/// WidgetMsg - enum for all messages emitted from widgets and dialogs.
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetMsg {
    // Widget outcomes:
    ButtonClicked(ButtonId),
    SliderChanged(SliderId, f32),
    DragFloatChanged(DragFloatId, f32),

    // Dialog outcomes:
    DialogAcceptedText(TextEntryDlgId, String),
    DialogAcceptedDragFloat(DragFloatDlgId, f32),
    //DialogAcceptedMultiTextEntry(MultiTextEntryDlgId, Vec<TextEntryResult>),
    DialogAcceptedMultiTextEntry(MultiTextEntryDlgId, Vec<(String, String)>),
    //DialogAcceptedFields(DialogId, Vec<(String, String)>)
}
