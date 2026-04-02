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
//! Designed to provide basic simulation and visualization capabilities for  applications.
//!
//! The eframe::App trait is the bridge between the custom gui_lib application
//! and the eframe framework that handles all the platform-specific details
//! of creating a window and running an event loop. In particular the App struct
//! will implement the eframe::App trait update() method.
// lib.rs

pub mod ids;
pub mod messages;

pub mod canvas;
pub mod shapes;
pub mod widgets;
pub mod dialogs;
pub mod timer;


// Public API re-exports (nice for both demo apps AND your internal modules)
pub use ids::*;
pub use messages::WidgetMsg;

pub use canvas::*;
pub use shapes::base::{Shape, ShapeBase}; // because Shape + ShapeBase live in shapes/base.rs
pub use shapes::*; // re-export Circle/Rectangle/etc
pub use widgets::{Widget, *}; // Widget trait lives in widgets.rs (and any other widget types)
pub use dialogs::*;
pub use timer::*;

// Handy egui re-exports
pub use eframe::egui::{self, Color32, Context, Pos2, Rect, Stroke, Ui, Vec2};


