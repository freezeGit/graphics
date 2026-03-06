// main.rs

/// Demonstration module for an application with a custom UI.
///
/// This module showcases the implementation of a demo application using the `eframe`
/// framework and a custom `gui_lib` library to render various graphical components.
///
/// # Modules
/// - The `demo` module defines an application structure (`TheApp`) and its behavior.
/// - Uses utilities and components from the `gui_lib` module.
///
/// # Components
///
/// ## TheApp
/// The main structure and entry point of the application.
/// - Contains a `Canvas` for holding a collection of shapes.
/// - Provides methods for creating and updating the UI.
///
/// ## Canvas
/// A container for rendering and managing graphical shapes.
///
/// ## Shapes
/// Custom shapes implemented via the `gui_lib::Shape` trait:
/// - `Circle`: A circular shape with customizable size, fill color, and outline.
/// - `Rectangle`: A rectangular shape with customizable size, position, and fill color.
/// - `Polyline`: A series of connected line segments with customizable line width and color.
/// - `Text`: A text label with customizable position, color, and font size.
///  - Other Shapes can be added as needed.
///
/// # Animation
/// - Demonstrates basic animations and state toggles using time-based checks.
/// - Shapes on the canvas have their properties dynamically updated, e.g., blinking colors.
///
/// # Usage
///
/// ## Running the Application
/// Call the `run_the_app()` function to start the application.
/// It initializes an `eframe` native window and sets up the demo layout and visuals.
///
/// demo::run_the_app()
///
/// ## Modifying Shapes
/// The application supports dynamic modification of shape properties, such as:
/// - Color, size, and position.
/// These can be altered within the `update` method using the shape trait's API.
///
/// ## Extending Functionality
/// - Additional shapes and widgets can be added to the `Canvas`.
/// - Use the `Shape` trait to define custom graphical components.
///
/// # Example
/// use super::demo::run_the_app;
///
/// fn main() -> Result<(), eframe::Error> {
///    run_the_app()
/// }
///
/// # Notes
/// - `ctx.request_repaint_after()` ensures smooth interface by updating the frame at a fixed interval.
/// - Animations may be run with a second (slower) Timer loop.
///
/// # Modules Used:
/// - Uses core functionality from:
///   - `eframe::egui`
///   - `crate::gui_lib`
/// - Demonstrates integration with external modules like `gui_lib` for rendering and shapes.
///
/// # Errors
/// This application returns an `eframe::Error` if initialization or event handling fails.
///

// Demonstration module. App-specific code
// ------------------------------
/// Module containing the demo application implementation.
///
/// This module defines the demo application structure and its behavior,
/// using the components defined in the `gui_lib` module.

mod ids;
mod world;
mod canvas;
mod app;
//mod demo; // optional. Shrink and delete finally

fn main() -> Result<(), eframe::Error> {
    //demo::run_the_app()
    app::run_the_app()
}