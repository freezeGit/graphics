
// lib.rs
//! This crate provides GUI components and application framework.
//!
//! It is intended to help me learn by writing a Rust version of
//! Stroustrup's graphics/gui code from
//! Programming Principles and Practice using C++

// ------------------------------
// Reusable module
// ------------------------------
/// Module containing reusable GUI components and utilities.
///
/// This module provides basic building blocks for creating GUI applications,
/// including buttons, screens, and visual styling utilities. It implements
/// a custom drawing system through the `Draw` trait.

mod gui_lib {
    pub use eframe::egui::{Ui, Button as EguiButton, vec2, Visuals, Color32};

    /// Creates a light theme similar to Windows 10 appearance.
    pub fn custom_light_visuals() -> Visuals {
        let mut visuals = Visuals::light(); // Start from egui's built-in light theme
        //let bkgd = Color32::from_rgb(240, 240, 240); // Main Windows 10 background color
        let bkgd = Color32::from_rgb(200, 200, 210); // My background color

        // Set overall background and panel colors
        visuals.extreme_bg_color = bkgd; // rarely used but set for completeness
        visuals.window_fill = bkgd;      // background of windows, popups, etc.
        visuals.panel_fill = bkgd;       // CentralPanel and other panels
        visuals.override_text_color = Some(Color32::BLACK); //set default text color

        visuals
    }

    /// Trait for components that can be drawn in the UI.
    ///
    /// Implement this trait for any component that needs to be rendered
    /// in the application's user interface.
    pub trait Draw {
        /// Draws the component in the provided UI context.
        ///
        /// # Arguments
        /// * `ui` - Mutable reference to the UI context

        fn draw(&self, ui: &mut Ui);
    }

    /// A customizable button component.
    ///
    /// # Fields
    /// * `width` - The width of the button in pixels
    /// * `height` - The height of the button in pixels
    /// * `label` - The text displayed on the button
    #[derive(Debug, Default)]
    pub struct Button {
        pub width: f32,
        pub height: f32,
        pub label: String,
    }

    // Implement Draw trait for Button
    impl Draw for Button {
        fn draw(&self, ui: &mut Ui) {
            let size = vec2(self.width, self.height);
            ui.add_sized(size, EguiButton::new(&self.label));
        }
    }

    /// A customizable Circle component.
    ///
    /// # Fields
    /// * `width` - The width of the button in pixels
    /// * `height` - The height of the button in pixels
    /// * `label` - The text displayed on the button
    #[derive(Debug, Default)]
    pub struct Circle {
        pub width: f32,
        pub height: f32,
        pub label: String,
    }

    // Implement Draw trait for Button
    impl Draw for Circle {
        fn draw(&self, ui: &mut Ui) {
            let size = vec2(self.width, self.height);
            ui.add_sized(size, EguiButton::new(&self.label));
        }
    }

    /// A container for drawable components.
    ///
    /// Screen acts as a container that can hold and manage multiple
    /// UI components that implement the `Draw` trait.
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        /// Renders all components contained in the screen.
        ///
        /// # Arguments
        /// * `ui` - Mutable reference to the UI context
        pub fn run(&self, ui: &mut Ui) {
            for component in &self.components {
                component.draw(ui);
            }
        }
    }
}

// ------------------------------
// App-specific module
// ------------------------------
/// Module containing the main application implementation.
///
/// This module defines the main application structure and its behavior,
/// utilizing the components defined in the `gui_lib` module.
mod app {
    use super::gui_lib::{Screen, Button};
    use eframe::egui::{Context, CentralPanel};

    /// Main application structure.
    ///
    /// Represents the root of the application and contains
    /// the main screen with all UI components.

    pub struct MyApp {
        screen: Screen,
    }

    // impl MyApp {
    //     /// Creates a new instance of the application.
    //     ///
    //     /// # Returns
    //     /// A new `MyApp` instance initialized with a default screen
    //     /// containing a sample button.
    //     pub fn new() -> Self {
    //         Self {
    //             screen: Screen {
    //                 components: vec![Box::new(Button {
    //                     width: 120.0,
    //                     height: 40.0,
    //                     label: "Click Me!".to_string(),
    //                 })],
    //             },
    //         }
    //     }
    // }

    impl MyApp {
        /// Creates a new instance of the application.
        ///
        /// # Returns
        /// A new `MyApp` instance initialized with a default screen
        /// containing a sample button.
        pub fn new() -> Self {
            Self {
                screen: Screen {
                    components: vec![
                        Box::new(Button {
                        width: 120.0,
                        height: 40.0,
                        label: "Click Me!".to_string(),
                    }),
                         Box::new(Button {
                             width: 120.0,
                             height: 40.0,
                             label: "Click Me!".to_string(),
                         }),]
                },
            }
        }
    }

    impl eframe::App for MyApp {
        fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
            CentralPanel::default().show(ctx, |ui| {
                self.screen.run(ui);
            });
        }
    }
}

// Optionally expose parts of the modules publicly
pub use gui_lib::{Draw, Button, Screen, custom_light_visuals};
pub use app::MyApp;
pub use eframe::egui::vec2;


