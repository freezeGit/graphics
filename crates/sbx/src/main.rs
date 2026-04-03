//! Sandbox crate for an application with a custom UI.
//!
//! This crate contains a sandbox application using the `eframe`
//! framework and a custom `gui_lib` library to render various graphical components.
//! This sandbox crate is intended to develop and test the `gui_lib` library.
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
//! User interface elements that can be added to the canvas.
//! - `Button`: A clickable button with customizable text and color.
//! - `Space`: A space between widgets.
//! - `Label`: A text label with customizable text and color.
//!
//! ## enum ActiveDialog
//! -- contains the active dialog.
//!
//! ## Shapes
//! Custom shapes implemented via the `gui_lib::Shape` trait:
//! - `Circle`: A circular shape with customizable size, fill color, and outline.
//! - `Rectangle`: A rectangular shape with customizable size, position, and fill color.
//! - `Polyline`: A series of connected line segments with customizable line width and color.
//! - `Text`: A text label with customizable position, color, and font size.
//!  - Other Shapes can be added as needed.
//!
//! # Animation
//! - The `Timer` class is used to manage any animation.
//! - Shapes on the canvas have their properties dynamically updated, e.g., blinking colors.
//!
//! ## Modifying Shapes
//! The application supports dynamic modification of shape properties, such as:
//! - Color, size, and position.
//! These can be altered within the `Canvas::update` method using the shape trait's API.
//!
//! ## Extending Functionality
//! - Additional shapes and widgets can be added to the `Canvas`.
//! - Use the `Shape` trait to define custom graphical components.
//!
//! # Modules Used:
//! - Uses core functionality from:
//!   - `eframe::egui`
//!   - `crate::gui_lib`
//! - Demonstrates integration with external modules like `gui_lib` for rendering and shapes.
//!
//! # Errors
//! This application returns an `eframe::Error` if initialization or event handling fails.
// main.rs

mod app;
mod canvas;
mod ids;
mod world;
// -----------------------------------------------------------

/// Constants for application configuration.
const APP_NAME: &str = "gui_lib sandbox app";
const XWVP: f32 = 1200.0; // Width of viewport in pixels.
const YHVP: f32 = 800.0; // Height of viewport in pixels.
const THEME: gui_lib::Theme = gui_lib::Theme::Light;

/// Initializes and creates an instance of the application.
///
/// ## Usage
/// This function is used to create an instance of the `app::TheApp` struct.
/// It is passed as an argument to the `gui_lib::run_app` function
fn create_app(cc: &eframe::CreationContext<'_>) -> app::TheApp {
    gui_lib::set_theme(cc, THEME);
    app::TheApp::new()
}
/// ## Running the Application
///
/// Function main() starts the application.
/// It calls the `run_app()` function,
/// which initializes an `eframe` native window
/// with a custom viewport size (`xv` and `yv`) 
/// and sets up the layout and visuals.
// ============================================================
// Function main() starts the application.
// ============================================================
fn main() -> Result<(), eframe::Error> {
    gui_lib::run_app(APP_NAME, XWVP, YHVP, create_app)
}

// ----------------------------------------------------------