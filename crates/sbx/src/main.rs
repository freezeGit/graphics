//! Demo crate for an application with a custom UI.
//!
//! This crate showcases the implementation of a demo application using the `eframe`
//! framework and a custom `gui_lib` library to render various graphical components.
//!
//! It an be used as a template to get started with gui_lib.
//!
//! # Components
//!
//! ## TheApp
//! The main structure and entry point of the application.
//! - Contains a `Canvas` for holding a collection of shapes and widgets.
//! - Provides methods for creating and updating the UI.
//!
//! ## TheCanvas
//! A container for rendering and managing graphical shapes
//! and interactive widgets.
//! - Manages a collection of shapes using the `Shape` trait.
//! - Supports dynamic updates of shape properties.
//! - Manages a collection of widgets using the `Widget` trait.
//! - Integrates with the `gui_lib` library for rendering.
//!
//! ## TheWorld
//! Program state and logic.
//! It deliberately has no dependency on gui_lib or egui.
//!
//! ## Widgets
//! User interface elements implemented via the [`gui_lib::Widget`].
//!
//! ## Shapes
//! Custom shapes implemented via the [`gui_lib::Shape`] trait:
//!
//! ## Dialogs
//! Modal dialogs implemented via the [`gui_lib::Dialog`] trait.
//!
//! # Animation
//! - The [`Timer`] struct is used to manage animations and state transitions.
//! - This demo crate demonstrates basic animations using time-based checks.
//! - Many programs will not need animation, and will not use the `Timer` struct.
//!
//! # Usage
//!
//! ## Running the Application
//! Call the [`run_the_app()`] function to start the application.
//! It initializes an `eframe` native window and sets up the demo layout and visuals.
//!
//! ## Modifying Shapes
//! The application supports dynamic modification of shape properties, such as:
//! - Color, size, and position
//! - Properties of concrete shapes like circles, rectangles, and lines.
//! - These can be altered within the [`Canvas::update`] method using the shape trait's API.
//!
//! ## Extending Functionality
//! - Additional shapes and widgets can be defined.
//! - Use the [`gui_lib::Shape`] trait to define custom graphical components.
//! - Use the [`gui_lib::Widget`] trait to define custom widgets.
//! - Use the [`gui_lib::Dialog`] trait to define custom dialogs.
//!
//! # Notes
//! - `ctx.request_repaint_after()` ensures a smooth interface
//! by updating the frame at a fixed interval.
//! - Animations may be run with a second (slower) Timer loop.
//! - If `ctx.request_repaint()` or `ctx.request_repaint_after()` is not called
//! egui is reactive, meaning it only repaints when there's an input event
//! (like mouse movement or a key press).
//!
//! # Modules Used:
//! - Uses core functionality from:
//!   - [`eframe::egui`]
//!   - [`gui_lib`]
//! - Demonstrates integration with [`gui_lib`] for rendering and shapes.
//!
//! # Errors
//! This application returns an [`eframe::Error`] if initialization or event handling fails.
// main.rs
mod app;
mod canvas;
mod ids;
mod world;

// -----------------------------------------------------------

/// Constants for application configuration.
const APP_NAME: &str = "Sand box app for gui_lib";
const XWVP: f32 = 1200.0; // Width of viewport in pixels.
const YHVP: f32 = 800.0; // Height of viewport in pixels.

/// ## Running the Application
///
/// Function `main()` starts the application.
/// It calls the `run_the_app()` function,
/// which initializes an `eframe` native window
/// with a custom viewport size (`width` and `height`)
/// and sets up the layout and visuals.
// ============================================================
// Function main() starts the application.
// ============================================================
fn main() -> Result<(), eframe::Error> {
    gui_lib::run_the_app::<app::TheApp>(APP_NAME, XWVP, YHVP)
}
