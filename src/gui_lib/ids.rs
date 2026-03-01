// src/gui_lib/ids.rs

pub type WidgetId = u32;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonId(pub WidgetId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SliderId(pub WidgetId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragFloatId(pub WidgetId);

// -------------------------------

pub type DialogId = u32;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextEntryDlgId(pub DialogId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragFloatDlgId(pub DialogId);

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