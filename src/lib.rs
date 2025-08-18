// lib.rs
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
/// including buttons, screens and visual styling utilities. It implements
/// a custom drawing system through the `Draw` trait.

mod gui_lib {
    pub use eframe::egui::{Button as EguiButton, Color32, Ui, Visuals,
                           CornerRadius, Stroke, StrokeKind,
                           Rect, pos2, vec2, Pos2, Vec2};

    /// Creates a light theme similar to Windows 10 appearance.
    pub fn custom_light_visuals() -> Visuals {
        //let mut visuals = Visuals::light(); // Start from egui's built-in light theme
        let mut visuals = Visuals::dark(); // Start from egui's built-in dark theme
        //let bkgd = Color32::from_rgb(240, 240, 240); // Main Windows 10 background color
        let bkgd = Color32::from_rgb(200, 200, 210); // My background color

        // Set overall background and panel colors
        visuals.extreme_bg_color = bkgd; // rarely used but set for completeness
        visuals.window_fill = bkgd; // background of windows, popups, etc.
        visuals.panel_fill = bkgd; // CentralPanel and other panels
        visuals.override_text_color = Some(Color32::BLACK); //set default text color

        visuals
    }

    /// Trait for components that can be drawn in the UI.
    ///
    /// Implement this trait for any component that needs to be rendered
    /// in the application's user interface.
    ///
    /// # Trait Implementer’s Note
    /// This trait requires `Debug` to be implemented for all types.
    /// Use `#[derive(Debug)]` or manually implement `std::fmt::Debug`.
    pub trait Draw: std::fmt::Debug {
        /// Draws the component in the provided UI context.
        ///
        /// # Arguments
        /// * `ui` - Mutable reference to the UI context
        fn draw(&self, ui: &mut Ui);
    }

    /// A container for drawable components.
    ///
    /// Screen acts as a container that can hold and manage multiple
    /// UI components that implement the `Draw` trait.
    #[derive(Debug)]
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
    /// * `position` - position of the circle center (: eframe::egui::Pos2)
    /// * `height` - The height of the button in pixels
    #[derive(Debug, Default)]
    pub struct Circle {
        pub position: Pos2,
        pub radius: f32,
    }

    // Implement Draw trait for Button
    impl Draw for Circle {
        fn draw(&self, ui: &mut Ui) {
            ui.painter().circle(
                self.position,
                self.radius,
                eframe::egui::Color32::from_rgb(100, 150, 250), // Blue circle
                eframe::egui::Stroke::new(2.0, eframe::egui::Color32::BLACK), // Black border
            );
        }
    }

    #[derive(Debug, Default)]
    pub struct Rectangle {
        pub position: eframe::egui::Pos2,
        pub size: Vec2,
     }

    // Implement Draw trait for Button
    impl Draw for Rectangle {
        fn draw(&self, ui: &mut Ui) {
            let rect = Rect::from_center_size(self.position, self.size);
            ui.painter().rect(
                rect,
                CornerRadius::ZERO,                         // or CornerRadius::same(r)
                //Color32::from_rgb(100, 150, 250),   // fill
                Color32::WHITE,   // fill
                Stroke::new(1.0, Color32::BLACK),     // border
                StrokeKind::Middle,           // Outside / Inside / Middle
            );
        }
    }

}   //gui_lib

// ------------------------------
// Demonstration module. App-specific code
// ------------------------------
/// Module containing the demo application implementation.
///
/// This module defines the demo application structure and its behavior,
/// utilizing the components defined in the `gui_lib` module.
mod demo {
    use super::gui_lib::{Button, Circle, Rectangle, Screen, Vec2};
    use eframe::egui::{CentralPanel, Context};

    /// Main application structure.
    ///
    /// Represents the root of the application and contains
    /// the main screen with all UI components.
    //Your app's gateway to native windows
    #[derive(Debug)]
    pub struct DemoApp {
        screen: Screen,
    }

    impl DemoApp {
        /// Creates a new instance of the application.
        ///
        /// # Returns
        /// A new `DemoApp` instance initialized with a default screen
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
                        Box::new(Circle {
                            //position: 120.0,
                            position: eframe::egui::Pos2::new(200.0, 200.0),
                            radius: 50.0,
                            //label: "Click Me!".to_string(),
                        }),
                        Box::new(Rectangle {
                            //position: 120.0,
                            position: eframe::egui::Pos2::new(400.0, 200.0),
                            size: Vec2::new(100.0, 75.0),
                            //label: "Click Me!".to_string(),
                        }),
                    ],
                },
            }
        }
    }

    // The eframe::App trait is the bridge between your custom application logic
    // and the eframe framework that handles all the platform-specific details
    // of creating a window and running an event loop.
    impl eframe::App for DemoApp {
        fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
            CentralPanel::default().show(ctx, |ui| {
                self.screen.run(ui);
            });
        }
    }
}

/// Run the demo app
pub fn run_demo() -> Result<(), eframe::Error> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_inner_size(vec2(1200.0, 800.0));
    eframe::run_native(
        "GUI Draw Example",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(custom_light_visuals()); //custom_light_visuals() lib.rs
            //cc.egui_ctx.set_visuals(eframe::egui::Visuals::light()); //light theme
            //cc.egui_ctx.set_visuals(eframe::egui::Visuals::dark()); //dark theme (default)
            let app: Box<dyn eframe::App> = Box::new(DemoApp::new());
            Ok(app)
        }),
    )
}

// Optionally expose parts of the modules publicly
pub use demo::DemoApp;
pub use eframe::egui::vec2;
pub use gui_lib::{Button, Draw, Screen, custom_light_visuals};


//Aug7

// impl Draw for Rectangle {
//     fn draw(&self, ui: &mut Ui) {
//         ui.painter().rect(
//             //self.position,
//             //,
//             Rect::from_center_size(self.position, self.size),
//             0.0,
//             eframe::egui::Color32::from_rgb(100, 150, 250), // Blue circle
//             eframe::egui::Stroke::new(1.0, eframe::egui::Color32::BLACK), // Black border
//             Default::default(), // TextureOptions (5th parameter),
//         );
//         // fn draw(&self, ui: &mut Ui) {
//         //     ui.painter().rectangle(
//         //         Rect::from_center_size(Pos2::new(50.0, 50.0), vec2::new(40.0, 30.0)),
//         //         self.position,
//         //         self.radius,
//         //         eframe::egui::Color32::from_rgb(100, 150, 250), // Blue circle
//         //         eframe::egui::Stroke::new(2.0, eframe::egui::Color32::BLACK), // Black border
//         //     );
//     }
// }
// }