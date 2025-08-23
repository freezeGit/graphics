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

pub mod gui_lib {
    pub use eframe::egui::{
        Button as EguiButton, Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2,
        Visuals, pos2, vec2,
    };

    /// Creates a custom light theme.
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

    /// Trait for  something that can be drawn in the UI.
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

    /// Trait for  anything that can be drawn in the UI.
    ///
    /// Implement this trait for any component that needs to be rendered
    /// in the application's user interface.
    ///
    /// Is used as a supertrait for shapes and widgets.
    ///
    /// # Trait Implementer’s Note
    /// This trait requires `Debug` to be implemented for all types.
    /// Use `#[derive(Debug)]` or manually implement `std::fmt::Debug`.
    pub trait Drawable: std::fmt::Debug {
        fn draw(&self, ui: &mut Ui);
    }

    /// Trait for any widget.
    ///
    /// Rendered with supertrait Drawable
     ///
    /// # Trait Implementer’s Note
    /// This trait requires `Debug` to be implemented for all types.
    /// Use `#[derive(Debug)]` or manually implement `std::fmt::Debug`.
    pub trait Widget: Drawable + std::fmt::Debug {
        // `draw` is provided by Drawable.

        // Specific methods for widgets:
        fn widget_print(&self, ui: &mut Ui);
        // fn layout(&mut self, ctx: &mut LayoutCtx);
        // fn event(&mut self, ctx: &mut EventCtx, event: &Event);
        // fn set_focused(&mut self, focused: bool);

        // Example of a different draw function
        // fn draw_with_highlight(&self, ctx: &mut PaintCtx) {
        //     ctx.set_highlight(true);
        //     self.draw(ctx);
        //     ctx.set_highlight(false);
       }

    /// A container for drawable components.
    ///
    /// Screen acts as a container that can hold and manage multiple
    /// UI components that implement the `Draw` trait.
    // #[derive(Debug)]
    // pub struct Screen {
    //     pub components: Vec<Box<dyn Draw>>,
    // }

    #[derive(Debug)]
    pub struct Screen {
        pub shapes: Vec<Box<dyn Shape>>,
        pub widgets: Vec<Box<dyn Widget>>,
    }

    impl Screen {
        /// Renders all components contained in the screen.
        ///
        /// # Arguments
        /// * `ui` - Mutable reference to the UI context
        pub fn run(&self, ui: &mut Ui) {
            for shape in &self.shapes {
                shape.draw(ui);
            }
            for widget in &self.widgets {
                widget.draw(ui);
                //widget.widget_print(ui);
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

    impl Drawable for Button {
        fn draw(&self, ui: &mut Ui) {
            let size = vec2(self.width, self.height);
            ui.add_sized(size, EguiButton::new(&self.label));
        }
    }

    impl Widget for Button {
        fn widget_print(&self, _ui: &mut Ui) {
            println!("Button: {:?}", self);
        }
    }

    /// Trait for any shape.
    ///
    /// Rendered on screen with supertrait Drawable
    ///
    /// # Trait Implementer’s Note
    /// This trait requires `Debug` to be implemented for all types.
    /// Use `#[derive(Debug)]` or manually implement `std::fmt::Debug`.
    pub trait Shape: Drawable + std::fmt::Debug {
        // `draw()` is provided by Drawable.

        // Specific methods for widgets:
        fn shape_print(&self, ui: &mut Ui);
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

    // Implement Draw trait for Circle
    impl Drawable for Circle {
        fn draw(&self, ui: &mut Ui) {
            ui.painter().circle(
                self.position,
                self.radius,
                eframe::egui::Color32::from_rgb(100, 150, 250), // Blue circle
                eframe::egui::Stroke::new(2.0, eframe::egui::Color32::BLACK), // Black border
            );
        }
    }

    impl Shape for Circle {
        fn shape_print(&self, _ui: &mut Ui) {
            println!("Circle: {:?}", self);
        }
    }

    #[derive(Debug, Default)]
    pub struct Rectangle {
        pub position: eframe::egui::Pos2,
        pub size: Vec2,
    }

    impl Drawable for Rectangle {
        fn draw(&self, ui: &mut Ui) {
            let rect = Rect::from_center_size(self.position, self.size);
            ui.painter().rect(
                rect,
                CornerRadius::ZERO, // or CornerRadius::same(r)
                //Color32::from_rgb(100, 150, 250),   // fill
                Color32::WHITE,                   // fill
                Stroke::new(1.0, Color32::BLACK), // border
                StrokeKind::Middle,               // Outside / Inside / Middle
            );
        }
    }

    impl Shape for Rectangle {
        fn shape_print(&self, _ui: &mut Ui) {
            println!("Rectangle: {:?}", self);
        }
    }
} //gui_lib

// ------------------------------
// Demonstration module. App-specific code
// ------------------------------
/// Module containing the demo application implementation.
///
/// This module defines the demo application structure and its behavior,
/// utilizing the components defined in the `gui_lib` module.
pub mod demo {
    use super::gui_lib::{Button, Circle, Rectangle, Screen, Vec2};
    use crate::{custom_light_visuals, vec2};
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
                    shapes: vec![
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
                    widgets: vec![
                        Box::new(Button {
                            width: 120.0,
                            height: 40.0,
                            label: "Click Me!".to_string(),
                        }),
                    ],
                },
            }
        }
    }

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
} // module demo

pub use demo::DemoApp;
/// Exposed publically
pub use eframe::egui::vec2;
pub use gui_lib::{Button, Draw, Screen, custom_light_visuals};
