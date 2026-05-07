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

pub mod app_gl;
pub mod canvas_gl;
pub mod dialogs_gl;
pub mod ids_gl;
pub mod messages_gl;
pub mod shapes_gl;
pub mod timer_gl;
pub mod widgets_gl;
pub mod world_gl;

// Public API re-exports (nice for both demo apps AND your internal modules)
// TDJ: tidy this section up
pub use app_gl::*;
pub use world_gl::*;

pub use ids_gl::*;
pub use messages_gl::WidgetMsg;

pub use canvas_gl::*;
pub use dialogs_gl::*;
pub use shapes_gl::base::{Shape, ShapeBase}; // because Shape + ShapeBase live in shapes/base.rs
pub use shapes_gl::*; // re-export Circle/Rectangle/etc
pub use timer_gl::*;
pub use widgets_gl::{Widget, *}; // Widget trait lives in widgets_gl.rs (and any other widget types)

// Handy egui re-exports
pub use eframe::egui::{self, Color32, Context, Pos2, Rect, Stroke, Ui, Vec2};
