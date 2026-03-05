//! This crate provides Shape objects, GUI components and application framework.
//!
//! It is intended to help me learn by writing a Rust version of
//! Stroustrup's graphics/gui API from
//! Programming Principles and Practice using C++

// ------------------------------
// This module will become its own library crate
// ------------------------------
/// Module containing GUI components and utilities.
///
/// This module provides basic building blocks for creating GUI applications,
/// including buttons, canvas and visual styling utilities. It implements
/// a custom drawing system through the `Draw` trait. It implements a custom widget system through the
/// 'Widget' trait, and a custom modal dialog system through th 'do_modal' trait.
// ---- Public re-exports ----
pub use eframe::egui::{
    self as egui,
    Color32, Pos2, Vec2, Rect, Stroke, StrokeKind,
    CornerRadius, Ui, Visuals,
    pos2, vec2,
};

// ---- Submodules ----
pub mod ids;
pub mod shapes;
pub mod widgets;
pub mod dialogs;
pub mod canvas;
pub mod timer;

// ---- Public API ----
pub use ids::*;
pub use shapes::*;
pub use widgets::*;
pub use dialogs::*;
pub use canvas::*;
pub use timer::Timer;

/// Constructs and returns a customized instance of `eframe::NativeOptions`.
///
/// This function initializes a default `eframe::NativeOptions` object and modifies its viewport to have
/// an inner size of 1200x800 pixels. The customized `NativeOptions` object is returned for further use.
///
/// # Returns
/// * `eframe::NativeOptions` - An instance of `eframe::NativeOptions` with the specified viewport size.
///
/// # Example

/// Use instead of `eframe::NativeOptions::default()` to set a custom viewport size.
pub fn native_options() -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_inner_size(vec2(1200.0, 800.0));
    native_options
}
//----------------------------------------------------------


// pub trait World: std::fmt::Debug {
//     fn advance(&mut self);
// }

