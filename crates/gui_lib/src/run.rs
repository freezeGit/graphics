//! This module contains the [`run_the_app`] function that is called by `main()` to run the application.

// run.rs

/// Called by `fn main()` to run the native application.
///
/// Runs the application using the [`eframe::run_native`] function
/// which initializes an `eframe` native window
/// with custom viewport size (`width` and `height`)
/// and sets up the layout and visuals.
///
/// # Type Parameters
///
/// * `App` - Your application type. Must implement the `UserApp` trait.
///
/// # Parameters
///
/// * `app_name` - The title displayed in the window title bar
/// * `width` - Initial window width in pixels
/// * `height` - Initial window height in pixels
///
/// # Returns
///
/// Returns `Ok(())` if the application exits normally, or an `eframe::Error`
/// if the window fails to initialize or crashes.
pub fn run_the_app<App>(app_name: &str, width: f32, height: f32) -> Result<(), eframe::Error>
where
    App: UserApp + 'static,
{
    let native_options = custom_native_options(width, height);

    eframe::run_native(
        app_name,
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            let app = Box::new(App::new());
            Ok(app)
        }),
    )
}

/// Custom native options, with a custom viewport size.
pub fn custom_native_options(xv: f32, yv: f32) -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_inner_size(egui::vec2(xv, yv));
    native_options
}

/// A trait representing a user-defined application that extends the functionality of the `eframe::App` framework.
///
/// Implementors of this trait must define the `new()` function with an empty parameter list,
/// which serves as a constructor for creating an instance of the application.
///
/// # Requirements
///
/// - The type implementing this trait must also implement the `eframe::App` trait, as this trait
///   builds upon the `eframe::App` interface.
/// - The `new()` function must have an empty parameter list.
///
/// # Notes
///
/// This trait is designed to provide a flexible and standardized way for users to define
/// and initialize their custom applications when using the `eframe` framework.
/// The `new()` function must have an empty parameter list. This guarantees that
/// the application `new()` constructor will have the correct signature to be called by the
/// `run_the_app()` function.

pub trait UserApp: eframe::App {
    fn new() -> Self;
}
