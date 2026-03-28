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
//! - The `Timer` class is used to manage animations and state transitions.
//! - Shapes on the canvas have their properties dynamically updated, e.g., blinking colors.
//!
//! # Usage
//!
//! ## Running the Application
//! Call the `run_the_app()` function to start the application.
//! It initializes an `eframe` native window and sets up the layout and visuals.
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
//! # Notes
//! - `ctx.request_repaint_after()` ensures a smooth interface by updating the frame at a fixed interval.
//! Animations may be run with a second (slower) Timer loop.
//! - If ctx.request_repaint() or ctx.request_repaint_after() is not called
//! egui is reactive, meaning it only repaints when there's an input event
//! (like mouse movement or a key press).
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

// ============================================================
// Function main() starts the application.
// ============================================================
fn main() -> Result<(), eframe::Error> {
    run_the_app()
}

// -----------------------------------------------------------
const APP_NAME: &str = "gui_lib sandbox app";
const XWVP: f32 = 1200.0; // Width of viewport in pixels.
const YHVP: f32 = 800.0; // Height of viewport in pixels.

/// Function run_the_app() starts a native (desktop) app.
///
/// Calls eframe::run_native() to create TheApp.
/// Change constant APP_NAME to change the name of the app.
/// Change constants XWVP and YHVP to adjust the width and height of the viewport.
/// This function can be modified to change the theme.
fn run_the_app() -> Result<(), eframe::Error> {
    let native_options = custom_native_options(XWVP, YHVP);
    eframe::run_native(
        APP_NAME,
        native_options, // or eframe::NativeOptions::default()
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light()); //light theme
            let app = Box::new(app::TheApp::new());
            Ok(app)
        }),
    )
}

/// Creates and returns an instance of `eframe::NativeOptions` with a custom viewport size.
///
/// # Arguments
/// * `xv` - A `f32` representing the horizontal size (width) of the viewport.
/// * `yv` - A `f32` representing the vertical size (height) of the viewport.
///
/// This function can be customized to change
/// options controlling the behavior of a native window.
fn custom_native_options(xv: f32, yv: f32) -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_inner_size(egui::vec2(xv, yv));
    native_options
}
