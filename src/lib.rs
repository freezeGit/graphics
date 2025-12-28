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
/// including buttons, canvass and visual styling utilities. It implements
/// a custom drawing system through the `Draw` trait.

pub mod gui_lib {
    use eframe::egui::Response;
    pub use eframe::egui::{
        Button as EguiButton, Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2,
        Visuals, pos2, vec2,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    pub type ShapeHandle = Rc<RefCell<dyn Shape>>;

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

    /// Constructs and returns a customized instance of `eframe::NativeOptions`.
    ///
    /// This function initializes a default `eframe::NativeOptions` object and modifies its viewport to have
    /// an inner size of 1200x800 pixels. The customized `NativeOptions` object is returned for further use.
    ///
    /// # Returns
    /// * `eframe::NativeOptions` - An instance of `eframe::NativeOptions` with the specified viewport size.
    ///
    /// # Example

    /// Use instead of `eframe::NativeOptions::default()` to set a custom viewport size.
    pub fn native_options() -> eframe::NativeOptions {
        let mut native_options = eframe::NativeOptions::default();
        native_options.viewport = native_options.viewport.with_inner_size(vec2(1200.0, 800.0));
        native_options
    }

    /// Trait for  anything that can be drawn in the UI.
    ///
    /// Implement this trait for any component that needs to be rendered
    /// in the application's user interface.
    ///
    /// Is used as a super trait for shapes and widgets.
    ///
    /// # Trait Implementer’s Note
    /// This trait requires `Debug` to be implemented for all types.
    /// Use `#[derive(Debug)]` or manually implement `std::fmt::Debug`.

    /// Trait for any widget.
    ///
    /// Rendered with supertrait Drawable
    ///
    /// # Trait Implementer’s Note
    /// This trait requires `Debug` to be implemented for all types.
    /// Use `#[derive(Debug)]` or manually implement `std::fmt::Debug`.
    pub trait Widget: std::fmt::Debug {
        fn invoke(&mut self, ui: &mut Ui) -> eframe::egui::Response;

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
    /// Canvas acts as a container that can hold and manage multiple
    /// UI components that implement the `Draw` trait.
    #[derive(Debug)]
    pub struct BasicCanvas {
        shapes: Vec<ShapeHandle>,
        pub widgets: Vec<Box<dyn Widget>>, // TDJ: make private
    }

    impl BasicCanvas {
        pub fn new() -> Self {
            BasicCanvas {
                shapes: Vec::new(),
                widgets: Vec::new(),
            }
        }

        /// Renders all components contained in the canvas.
        /// pub fn run(&mut self, ui: &mut Ui) {

        pub fn run(&mut self, ui: &mut Ui) {
            for shape in &self.shapes {
                shape.borrow().draw(ui);
            }
            for widget in &mut self.widgets {
                widget.invoke(ui);
            }
        }

        /// Returns a mutable reference to a shape handle at the given index.
        pub fn get_shape_mut(&mut self, index: usize) -> Option<&mut ShapeHandle> {
            self.shapes.get_mut(index)
        }

        pub fn add_shape(&mut self, s: ShapeHandle) {
            self.shapes.push(s);
        }
        pub fn add_widget(&mut self, w: Box<dyn Widget>) {
            self.widgets.push(w);
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

    impl Button {
        // Constructor method
        pub fn new(width: f32, height: f32, label: String) -> Self {
            Self {
                width,
                height,
                label,
            }
        }
    }

    impl Widget for Button {
        fn invoke(&mut self, ui: &mut Ui) -> Response {
            let size = vec2(self.width, self.height);
            ui.add_sized(size, EguiButton::new(&self.label))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum LineStyle {
        Solid,
        Dashed,
        Dotted,
        //Dashed { dash: f32, gap: f32 },
        //Dotted { spacing: f32, radius: f32 },
    }

    /// Base struct for all shapes.
    #[derive(Debug)]
    pub struct ShapeBase {
        location: Pos2,
        points: Vec<Pos2>,
        color: Color32,
        fill_color: Color32,
        line_width: f32,
        line_style: LineStyle,
    }

    /// Trait for any shape.
    ///
    /// Rendered on canvas with supertrait Drawable
    ///
    /// # Trait Implementer’s Note
    /// This trait requires `Debug` to be implemented for all types.
    /// Use `#[derive(Debug)]` or manually implement `std::fmt::Debug`.

    pub trait Shape: std::fmt::Debug {
        fn base(&self) -> &ShapeBase;
        fn base_mut(&mut self) -> &mut ShapeBase;

        fn draw(&self, ui: &mut Ui);

        fn move_to(&mut self, location: Pos2) {
            self.base_mut().move_to(location)
        }

        fn color(&self) -> Color32 {
            self.base().color()
        }
        fn set_color(&mut self, col: Color32) {
            self.base_mut().set_color(col)
        }

        fn fill_color(&self) -> Color32 {
            self.base().fill_color()
        }
        fn set_fill_color(&mut self, col: Color32) {
            self.base_mut().set_fill_color(col)
        }

        fn line_width(&self) -> f32 {
            self.base().line_width()
        }
        fn set_line_width(&mut self, lw: f32) {
            self.base_mut().set_line_width(lw)
        }
    }

    impl Default for ShapeBase {
        fn default() -> Self {
            Self {
                location: Pos2::default(),
                points: Vec::new(),
                color: Color32::BLACK,
                fill_color: Color32::TRANSPARENT,
                line_width: 2.0,
                //line_style: LineStyle::Solid,
                //line_style: LineStyle::Dashed { dash: 8.0, gap: 4.0 },
                //line_style: LineStyle::Dashed,
                //line_style: LineStyle::Dotted { spacing: 8.0, radius: 2.0 },
                line_style: LineStyle::Dotted,
            }
        }
    }

    impl ShapeBase {
        /// Create a new, empty ShapeBase with default values.
        // pub fn new() -> Self {
        //     Self::default()
        // }

        pub fn move_to(&mut self, location: Pos2) {
            self.location = location;
        }
        pub fn color(&self) -> Color32 {
            self.color
        }
        pub fn set_color(&mut self, col: Color32) {
            self.color = col;
        }

        pub fn fill_color(&self) -> Color32 {
            self.fill_color
        }
        pub fn set_fill_color(&mut self, col: Color32) {
            self.fill_color = col;
        }

        pub fn line_width(&self) -> f32 {
            self.line_width
        }
        pub fn set_line_width(&mut self, lw: f32) {
            self.line_width = lw;
        }

        pub(crate) fn points_translated(&self, offset: Vec2) -> Vec<Pos2> {
            self.points.iter().map(|p| *p + offset).collect()
        }

        pub(crate) fn dash_length(&self) -> f32 {
            4.0 * self.line_width
        }
        pub(crate) fn dash_gap(&self) -> f32 {
            1.0 + (2.0 * self.line_width)
        }
        pub(crate) fn dot_radius(&self) -> f32 {
            self.line_width / 2.0
        }
        pub(crate) fn dot_spacing(&self) -> f32 {
            1.0 + (2.0 * self.line_width)
        }
    }

    /// A customizable Polyline component.
    ///
    /// # Fields
    /// * `position` - position of the circle center (: eframe::egui::Pos2)
    /// * `radius` - The radius of the button
    #[derive(Debug, Default)]
    pub struct Polyline {
        base: ShapeBase,
    }

    impl Polyline {
        pub fn new(location: Pos2, points: impl IntoIterator<Item = Pos2>) -> Self {
            Self {
                base: ShapeBase {
                    location,
                    points: points.into_iter().collect(),
                    ..Default::default()
                },
            }
        }
    }

    impl Shape for Polyline {
        fn base(&self) -> &ShapeBase {
            &self.base
        }
        fn base_mut(&mut self) -> &mut ShapeBase {
            &mut self.base
        }

        fn draw(&self, ui: &mut Ui) {
            let painter = ui.painter();

            let points = self.base.points_translated(self.base.location.to_vec2());
            let stroke = Stroke::new(self.base.line_width, self.base.color);

            match self.base.line_style {
                LineStyle::Solid => {
                    painter.add(eframe::epaint::PathShape::line(points, stroke)); // :contentReference[oaicite:4]{index=4}
                }
                LineStyle::Dashed => {
                    let shapes = eframe::egui::Shape::dashed_line(
                        &points,
                        stroke,
                        self.base.dash_length(),
                        self.base.dash_gap(),
                    ); // :contentReference[oaicite:5]{index=5}
                    painter.extend(shapes); // :contentReference[oaicite:6]{index=6}
                }

                LineStyle::Dotted => {
                    let shapes = eframe::egui::Shape::dotted_line(
                        &points,
                        self.base.color,
                        self.base.dot_spacing(),
                        self.base.dot_radius(),
                    ); // :contentReference[oaicite:7]{index=7}
                    painter.extend(shapes); // :contentReference[oaicite:8]{index=8}
                }
            }
        }
    }

    /// A customizable Circle component.
    ///
    /// # Fields
    /// * `position` - position of the circle center (: eframe::egui::Pos2)
    /// * `radius` - The radius of the button
    #[derive(Debug, Default)]
    pub struct Circle {
        base: ShapeBase,
        pub radius: f32,
    }

    impl Circle {
        // Constructor method
        pub fn new(center: Pos2, radius: f32) -> Self {
            Self {
                base: {
                    ShapeBase {
                        location: center,
                        ..Default::default()
                    }
                },
                radius: radius,
            }
        }
    }

    impl Shape for Circle {
        fn base(&self) -> &ShapeBase {
            &self.base
        }
        fn base_mut(&mut self) -> &mut ShapeBase {
            &mut self.base
        }

        fn draw(&self, ui: &mut Ui) {
            ui.painter().circle(
                self.base.location,
                self.radius,
                self.base.fill_color,
                Stroke::new(self.base.line_width, self.base.color), // Black border
            );
        }
    }

    #[derive(Debug, Default)]
    pub struct Rectangle {
        base: ShapeBase,
        pub size: Vec2,
    }
    impl Rectangle {
        pub fn new(center: Pos2, size: Vec2) -> Self {
            Rectangle {
                base: {
                    ShapeBase {
                        location: center,
                        ..Default::default()
                    }
                },
                //location: center,
                size: size,
            }
        }
    }

    impl Shape for Rectangle {
        fn base(&self) -> &ShapeBase {
            &self.base
        }
        fn base_mut(&mut self) -> &mut ShapeBase {
            &mut self.base
        }

        fn draw(&self, ui: &mut Ui) {
            let rect = Rect::from_center_size(self.base.location, self.size);
            ui.painter().rect(
                rect,
                CornerRadius::ZERO,   // or CornerRadius::same(r)
                self.base.fill_color, // fill
                Stroke::new(self.base.line_width, self.base.color), // border
                StrokeKind::Outside,  // Outside / Inside / Middle
            );
        }
    }
} // closes mod gui_lib

///
/// Demonstration module for an application with a custom UI.
///
/// This module showcases the implementation of a demo application using the `eframe`
/// framework and a custom `gui_lib` library to render various graphical components.
///
/// - The application initializes and displays a set of graphical shapes on a canvas.
/// - It features basic animation and user interaction behaviors.
/// - Components such as circles, rectangles, and polylines are created, styled, and animated.
/// - Demonstrates the use of time-based updates (`ctx.input(|i| i.time)`) for animations.
///
/// # Main Features
/// - Central panel UI using `eframe::egui`.
/// - Customizable shapes rendered on a canvas.
/// - The ability to toggle visual properties (e.g., color) over time.
/// - Integration with `eframe::App` for interacting with the `egui` context.
///
/// # Modules
/// - The `demo` module defines an application structure (`DemoApp`) and its behavior.
/// - Uses utilities and components from the `gui_lib` module.
///
/// # Components
///
/// ## DemoApp
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
///
/// # Animation
/// - Demonstrates basic animations and state toggles using time-based checks.
/// - Shapes on the canvas have their properties dynamically updated, e.g., blinking colors.
///
/// # Usage
///
/// ## Running the Application
/// Call the `run_demo()` function to start the application.
/// It initializes an `eframe` native window and sets up the demo layout and visuals.
///
/// demo::run_demo()
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
/// use super::demo::run_demo;
///
/// fn main() -> Result<(), eframe::Error> {
///     run_demo()
/// }
///
/// # Notes
/// - The `custom_light_visuals` function is used to define a custom theme for the UI.
/// - `ctx.request_repaint_after()` ensures smooth animations by updating the frame at a fixed interval.
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
pub mod demo {
    //use super::gui_lib::Shape;
    //use super::gui_lib::Widget;
    //use crate::gui_lib::Widget;
    //use super::gui_lib::{Button, Circle, Color32, Polyline, Rectangle, Canvas, Vec2};
    use super::gui_lib::{BasicCanvas, Button, Circle, Color32, Polyline, Rectangle};
    //use crate::{custom_light_visuals, native_options, vec2};
    //use crate::{custom_light_visuals};
    use crate::custom_light_visuals;
    use crate::gui_lib::{Shape, ShapeHandle, Widget};
    use eframe::egui::{CentralPanel, Context};
    use std::cell::RefCell;
    use std::rc::Rc;

    //use crate::{custom_light_visuals, gui_lib::Shape, gui_lib::Widget, gui_lib::ShapeHandle};
    //use eframe::egui::{vec2, CentralPanel, Context};

    #[derive(Debug)]
    pub struct DemoCanvas {
        pub canvas: BasicCanvas,
        pub sc1: ShapeHandle,
        pub sc2: ShapeHandle,
        pub sr: ShapeHandle,
        pub sp: ShapeHandle,
    }

    impl DemoCanvas {
        // Build the BasicCanvas field, shapes, and widgets together, return fully initialized Self
        pub fn new() -> Self {
            // New empty canvas
            let mut canvas = BasicCanvas::new();

            //Create and add shapes as Rc<RefCell<T>
            let sc1: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
                eframe::egui::Pos2::new(200.0, 200.0),
                75.0,
            )));
            sc1.borrow_mut().set_line_width(4.0);
            sc1.borrow_mut().set_fill_color(Color32::DARK_RED);
            canvas.add_shape(sc1.clone());

            let sc2: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
                eframe::egui::Pos2::new(200.0, 200.0),
                10.0,
            )));
            canvas.add_shape(sc2.clone());

            let sr: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new(
                eframe::egui::Pos2::new(400.0, 200.0),
                eframe::egui::Vec2::new(150.0, 100.0),
            )));
            sr.borrow_mut().set_fill_color(Color32::GOLD);
            canvas.add_shape(sr.clone());

            let sp: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
                eframe::egui::Pos2::new(550.0, 200.0),
                [
                    eframe::egui::Pos2::new(0.0, 0.0),
                    eframe::egui::Pos2::new(25.0, 50.0),
                    eframe::egui::Pos2::new(75.0, -50.0),
                    eframe::egui::Pos2::new(125.0, 50.0),
                    eframe::egui::Pos2::new(175.0, -50.0),
                    eframe::egui::Pos2::new(225.0, 50.0),
                    eframe::egui::Pos2::new(250.0, 0.0),
                ],
            )));
            sp.borrow_mut().set_line_width(2.0);
            sp.borrow_mut().set_color(Color32::RED);
            canvas.add_shape(sp.clone());

            // Create and add widgets as Box<dyn Widget>
            let mut wb = Button::new(120.0, 40.0, "Push me".to_string());
            canvas.widgets.push(Box::new(wb));

            //Create the DemoCanvas
            Self {
                canvas,
                sc1,
                sc2,
                sr,
                sp,
            }
        }
        pub fn canvas(&self) -> &BasicCanvas {
            &self.canvas
        }
        pub fn canvas_mut(&mut self) -> &mut BasicCanvas {
            &mut self.canvas
        }
    }

    /// Main application structure.
    ///
    /// Represents the root of the application and contains
    /// the main canvas with all UI components.
    #[derive(Debug)]
    struct DemoApp {
        canvas: DemoCanvas,
        last_toggle: f64,
        is_red: bool,
    }

    // fn base(&self) -> &ShapeBase;
    // fn base_mut(&mut self) -> &mut ShapeBase;

    impl DemoApp {
        /// Creates a new instance of the application.
        ///
        /// # Returns
        /// A new `DemoApp` instance initialized with a default canvas
        /// containing several shapes
        /// and containing a sample button.
        pub fn new() -> Self {
            Self {
                canvas: DemoCanvas::new(),
                last_toggle: 0.0, //For time-gating
                is_red: true,
            }
        }
    }

    pub fn run_demo() -> Result<(), eframe::Error> {
        eframe::run_native(
            "GUI Draw Example",
            super::gui_lib::native_options(),
            Box::new(|cc| {
                cc.egui_ctx.set_visuals(custom_light_visuals()); //custom_light_visuals() lib.rs
                //cc.egui_ctx.set_visuals(eframe::egui::Visuals::light()); //light theme
                //cc.egui_ctx.set_visuals(eframe::egui::Visuals::dark()); //dark theme (default)
                let app = Box::new(DemoApp::new());
                //app.canvas.shapes[0].set_fill_color(Color32::GREEN); // Shape can be changed here
                Ok(app)
            }),
        )
    }

    // The eframe::App trait is the bridge between your custom application logic
    // and the eframe framework that handles all the platform-specific details
    // of creating a window and running an event loop.

    /// The eframe::App trait is the bridge between your custom application logic
    /// and the eframe framework that handles all the platform-specific details
    /// of creating a window and running an event loop.

    impl eframe::App for DemoApp {
        fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
            // Demonstrate access to Shape sp
            self.canvas
                .sp
                .borrow_mut()
                .move_to(eframe::egui::Pos2::new(550.0, 400.0));

            //if using index instead of handle
            // if let Some(s) = self.canvas.canvas.get_shape_mut(3) {
            //     s.borrow_mut()
            //         .move_to(eframe::egui::Pos2::new(550.0, 400.0));
            // }

            // Test of basic simulation/animation
            let now = ctx.input(|i| i.time);
            if now - self.last_toggle >= 0.5 {
                self.last_toggle = now;
                self.is_red = !self.is_red;
                let c = if self.is_red {
                    Color32::RED
                } else {
                    Color32::BLUE
                };
                self.canvas.sc2.borrow_mut().set_fill_color(c);
            }

            CentralPanel::default().show(ctx, |ui| {
                //self.canvas.run(ui);
                self.canvas.canvas.run(ui);
            });

            ctx.request_repaint_after(std::time::Duration::from_millis(16));
            // or: ctx.request_repaint_after(Duration::from_millis(500)) if you truly only want periodic frames
        }
    }
} // module demo

/// Exposed publicly
//pub use demo::DemoApp;
pub use eframe::egui::vec2;
//pub use gui_lib::{Button, Draw, Canvas, custom_light_visuals};
pub use gui_lib::{BasicCanvas, Button, custom_light_visuals};

//Changed again
