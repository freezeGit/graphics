// lib.rs

mod canvas;
mod dialogs;
mod ids;
mod shapes;
mod timer;
mod widgets;

// Public API re-exports (nice for both demo apps AND your internal modules)
// IDs and message types
pub use ids::{
    ButtonId, DialogId, DragFloatDlgId, DragFloatId, MessageBoxDlgId, SliderId, TextEntryDlgId,
    WidgetMsg,
};

pub use canvas::*;
pub use dialogs::*;
//pub use ids::WidgetMsg;          // WidgetMsg lives in ids.rs
pub use shapes::base::{Shape, ShapeBase}; // because Shape + ShapeBase live in shapes/base.rs
pub use shapes::*; // re-export Circle/Rectangle/etc
pub use timer::*;
pub use widgets::{Widget, *}; // Widget trait lives in widgets.rs (and any other widget types)

// Handy egui re-exports
pub use eframe::egui::{self, Color32, Context, Pos2, Rect, Stroke, Ui, Vec2};

pub fn native_options() -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options
        .viewport
        .with_inner_size(egui::vec2(1200.0, 800.0));
    native_options
}
