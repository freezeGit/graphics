//! This module contains the [`run_app`] function that is called by [`main()`] to run the application.

// run.rs

/// Called by `fn main()` to run the native application.
///
/// Creates the application using the provided `make_app` function,
/// then runs the application using the [`eframe::run_native`] function
/// which initializes an `eframe` native window
/// with custom viewport size (`xv` and `yv`) 
/// and sets up the layout and visuals.
pub fn run_app<A, F>(
    app_name: &str,
    xv: f32,
    yv: f32,
    make_app: F,
) -> Result<(), eframe::Error>
where
    A: eframe::App + 'static,
    F: FnOnce(&eframe::CreationContext<'_>) -> A + 'static,
{
    let native_options = custom_native_options(xv, yv);

    eframe::run_native(
        app_name,
        native_options,
        Box::new(move |cc| Ok(Box::new(make_app(cc)))),
    )
}

/// The `Theme` enum represents the visual theme of an application or user interface.
///
/// This enum provides two variants:
/// - `Light`: Represents a light theme, typically with lighter colors and a brighter appearance.
/// - `Dark`: Represents a dark theme, typically with darker colors and a more subdued appearance.
///
pub enum Theme {
    Light,
    Dark,
}

/// Sets the visual theme for the application using the given `CreationContext` and `Theme`.
///
/// # Parameters
///
/// - `cc`: A reference to the [`eframe::CreationContext`] which is used to configure the application during startup.
///         This provides access to the [`egui::Context`], where the visuals can be set.
/// - `theme`: An instance of the [`Theme`] enum that specifies the desired theme for the application.
///            - `Theme::Light`: Sets the application to use a light theme.
///            - `Theme::Dark`: Sets the application to use a dark theme.
///
/// This function adjusts the visual style of the user interface by selecting either
/// a light or dark appearance based on the provided theme.
///
/// # Notes
///
/// This function should typically be called during the initial setup phase of the application.
pub fn set_theme(cc: &eframe::CreationContext<'_>, theme: Theme) {
    let visuals = match theme {
        Theme::Light => egui::Visuals::light(),
        Theme::Dark => egui::Visuals::dark(),
    };
    cc.egui_ctx.set_visuals(visuals);
}

/// Custom native options, with a custom viewport size.
pub fn custom_native_options(xv: f32, yv: f32) -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_inner_size(egui::vec2(xv, yv));
    native_options
}