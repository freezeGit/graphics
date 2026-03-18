//! gui_lib
//!
//! A small GUI library built on top of [`egui`] and [`eframe`].
//!
//! Provides:
//! - simple canvas drawing (Circle, Line, Rectangle, Text, etc.)
//! - widgets (Button, DragFloat, etc.)
//! - basic dialogs
//! - timer support for simulation loops
//!
//! Designed for small simulation and visualization applications.
//!
//! The eframe::App trait is the bridge between the custom gui_lib application
//! and the eframe framework that handles all the platform-specific details
//! of creating a window and running an event loop. In particular the App struct
//! will implement the eframe::App trait update() method.
// lib.rs

pub mod canvas;
pub mod dialogs;
pub mod ids;
pub mod shapes;
pub mod timer;
pub mod widgets;


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

/// Creates and returns an instance of `eframe::NativeOptions` with a custom viewport size.
///
/// # Arguments
/// * `xv` - A `f32` representing the horizontal size (width) of the viewport.
/// * `yv` - A `f32` representing the vertical size (height) of the viewport.
///

pub fn native_options(xv: f32, yv: f32) -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options
        .viewport
        .with_inner_size(egui::vec2(xv, yv));
    native_options
}

