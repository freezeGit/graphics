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
    //use eframe::egui::Response;
    pub use eframe::egui::{
        Button as EguiButton, Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2,
        Visuals, pos2, vec2,
    };
    use egui::{CentralPanel, Context};
    use std::cell::RefCell;
    use std::rc::Rc;

    // Handle for Shapes in BasicCanvas::Vec<ShapeHandle>
    pub type ShapeHandle = Rc<RefCell<dyn Shape>>;

    // enum for canvas layoutstyles
    #[derive(Debug)]
    pub enum LayoutStyle {
        TopPanel,
        SidePanel,
        NoPanel,
    }

    // Examples of background colors. Any Color32 will do.
    pub const BKG_EXAMPLE: Color32 = Color32::from_rgb(200, 200, 210);
    pub const BKG_WINDOWS: Color32 = Color32::from_rgb(240, 240, 240);

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
    //----------------------------------------------------------

    /// A trait that represents a world with the ability to advance its state.
    pub trait World: std::fmt::Debug {
        fn advance(&mut self);
    }
    //------------------------------------------------

    /// A container for drawable components.
    ///
    /// Canvas acts as a container that can hold and manage multiple
    /// UI components that implement the `Draw` trait.
    #[derive(Debug)]
    pub struct BasicCanvas {
        layout: LayoutStyle,
        background_color: Color32,
        shapes: Vec<ShapeHandle>,
        pub widgets: Vec<Box<dyn Widget>>, // TDJ: make private
    }

    impl BasicCanvas {
        pub fn new(layout: LayoutStyle, bkg: Color32) -> Self {
            BasicCanvas {
                layout: layout,
                background_color: bkg,
                shapes: Vec::new(),
                widgets: Vec::new(),
            }
        }

        // shapes in canvas-----------------------------------------

        // --- internal helper: convert any concrete Rc<RefCell<T>> into a ShapeHandle
        fn erase_handle<T: Shape + 'static>(rc: &Rc<RefCell<T>>) -> ShapeHandle {
            rc.clone() // unsize coercion to Rc<RefCell<dyn Shape>>
        }

        // --- internal helper: find index by pointer identity (ShapeHandle -> ShapeHandle)
        fn index_of_handle(&self, target: &ShapeHandle) -> Option<usize> {
            self.shapes.iter().position(|h| Rc::ptr_eq(h, target))
        }

        pub fn set_background_color(&mut self, color: Color32) {
            self.background_color = color;
        }

        /// Add a shape to the canvas.
        pub fn add_shape(&mut self, s: ShapeHandle) {
            self.shapes.push(s);
        }

        // Returns a mutable reference to a shape handle at the given index.
        // TDJ: is this needed?
        pub fn get_shape_mut(&mut self, index: usize) -> Option<&mut ShapeHandle> {
            self.shapes.get_mut(index)
        }
        //  Returns a mutable reference to the top-most shape handle (last added).
        // TDJ:: is this needed?
        pub fn get_top_shape_mut(&mut self) -> Option<&mut ShapeHandle> {
            self.shapes.last_mut()
        }

        /// Put shape `a` on top (i.e., draw last).
        /// Returns false if shape is not found in `self.shapes`.
        //pub fn put_on_top();
        pub fn put_on_top<TA>(&mut self, a: &Rc<RefCell<TA>>) -> bool
        where
            TA: Shape + 'static,
        {
            let a_h: ShapeHandle = Self::erase_handle(a);
            self.put_on_top_handle(&a_h)
        }

        /// Same as `put_on_top`, but takes erased handles directly.
        pub fn put_on_top_handle(&mut self, a: &ShapeHandle) -> bool {
            let Some(i) = self.shapes.iter().position(|h| Rc::ptr_eq(h, a)) else {
                return false;
            };
            if i + 1 == self.shapes.len() {
                return true;
            } // already top
            let entry = self.shapes.remove(i);
            self.shapes.push(entry);
            true
        }

        /// Put shape `a` on top of shape `b` (i.e., draw `a` after `b`).
        /// Returns false if either shape is not found in `self.shapes`.
        pub fn put_on_top_of<TA, TB>(&mut self, a: &Rc<RefCell<TA>>, b: &Rc<RefCell<TB>>) -> bool
        where
            TA: Shape + 'static,
            TB: Shape + 'static,
        {
            let a_h: ShapeHandle = Self::erase_handle(a);
            let b_h: ShapeHandle = Self::erase_handle(b);
            self.put_on_top_of_handle(&a_h, &b_h)
        }

        /// Same as `put_on_top_of`, but takes erased handles directly.
        pub fn put_on_top_of_handle(&mut self, a: &ShapeHandle, b: &ShapeHandle) -> bool {
            let ia = self.index_of_handle(a);
            let ib = self.index_of_handle(b);
            let (Some(ia), Some(mut ib)) = (ia, ib) else {
                return false;
            };

            if ia == ib {
                return true;
            }

            // Remove A first
            let entry = self.shapes.remove(ia);

            // If A was before B, B shifts left by 1 after removal
            if ia < ib {
                ib -= 1;
            }

            // Insert A after B so it draws "over" B
            self.shapes.insert(ib + 1, entry);
            true
        }

        /// Remove a shape by identity, using your concrete handle (e.g. &self.sc2).
        /// Returns true if removed.
        pub fn remove_shape<T: Shape + 'static>(&mut self, s: &Rc<RefCell<T>>) -> bool {
            let s_h: ShapeHandle = Self::erase_handle(s);
            self.remove_shape_handle(&s_h)
        }

        /// Remove a shape by identity, using an erased handle.
        /// Returns true if removed.
        pub fn remove_shape_handle(&mut self, s: &ShapeHandle) -> bool {
            if let Some(i) = self.index_of_handle(s) {
                self.shapes.remove(i);
                true
            } else {
                false
            }
        }

        // Widgets in canvas --------------------------------------------------

        /// Add a widget to the canvas
        pub fn add_widget(&mut self, w: Box<dyn Widget>) {
            self.widgets.push(w);
        }

        // Rendering canvas ---------------------------------------------

        /// Renders all widgets and shapes and modifies Vec<WidgetMsg>
        ///  to hold a sequence of tagged messages from any widgets invoked.
        pub fn render(&mut self, ctx: &Context, out: &mut Vec<WidgetMsg>) {
            match self.layout {
                LayoutStyle::TopPanel => self.render_with_top_panel(ctx, out),
                LayoutStyle::SidePanel => self.render_with_side_panel(ctx, out),
                LayoutStyle::NoPanel => self.render_with_no_panel(ctx, out),
            }
        }

        /// Renders all widgets in TopBottomPanel and shapes in the CentralPanel.
        fn render_with_top_panel(&mut self, ctx: &Context, out: &mut Vec<WidgetMsg>) {
            egui::TopBottomPanel::top("toolbar")
                .resizable(true)
                .default_height(48.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        for widget in &mut self.widgets {
                            widget.invoke(ui, out);
                        }
                    });
                });

            CentralPanel::default().show(ctx, |ui| {
                let (response, painter) =
                    ui.allocate_painter(ui.available_size(), egui::Sense::hover());
                let rect = response.rect;
                painter.rect_filled(rect, 0.0, self.background_color);
                // (response.rect).min is the top-left corner position
                // of the rectangular area returned by ui.available_size()
                let offset = response.rect.min.to_vec2(); // to top-left corner
                for shape in &self.shapes {
                    shape.borrow().draw_at(&painter, offset);
                }
            });
        }

        /// Renders all widgets in SidePanel and shapes in the CentralPanel.
        fn render_with_side_panel(&mut self, ctx: &Context, out: &mut Vec<WidgetMsg>) {
            egui::SidePanel::left("controls")
                .resizable(true)
                .default_width(180.0)
                .show(ctx, |ui| {
                    //ui.heading("Controls");  // TDJ: only if you want side panel to be labelled
                    for widget in &mut self.widgets {
                        //widget.invoke(ui);
                        widget.invoke(ui, out);
                    }
                });

            CentralPanel::default().show(ctx, |ui| {
                let (response, painter) =
                    ui.allocate_painter(ui.available_size(), egui::Sense::hover());
                let rect = response.rect;
                painter.rect_filled(rect, 0.0, self.background_color);
                // (response.rect).min is the top-left corner position
                // of the rectangular area returned by ui.available_size()
                let offset = response.rect.min.to_vec2(); // to top-left corner
                for shape in &self.shapes {
                    shape.borrow().draw_at(&painter, offset);
                }
            });
        }

        /// Renders all shapes and widgets in the CentralPanel.
        fn render_with_no_panel(&mut self, ctx: &Context, out: &mut Vec<WidgetMsg>) {
            CentralPanel::default().show(ctx, |ui| {
                let painter = ui.painter();
                let rect = ui.available_rect_before_wrap();
                painter.rect_filled(rect, 0.0, self.background_color);

                for shape in &self.shapes {
                    shape.borrow().draw(&painter);
                }
                for widget in &mut self.widgets {
                    //widget.invoke(ui);
                    widget.invoke(ui, out);
                }
            });
        }
    }

    //-------------------------------------------------------------------

    pub type WidgetId = u32;

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    //pub struct SpaceId(pub WidgetId);
    pub struct SpaceId;

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ButtonId(pub WidgetId);

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SliderId(pub WidgetId);
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DragFloatId(pub WidgetId);

    #[derive(Debug, Clone, PartialEq)]
    pub enum WidgetMsg {
        ButtonClicked(ButtonId),
        SliderChanged(SliderId, f32),
        DragFloatChanged(DragFloatId, f32),
    }

    /// Trait for invoking any widget in the UI.
    pub trait Widget: std::fmt::Debug {
        fn invoke(&mut self, ui: &mut egui::Ui, out: &mut Vec<WidgetMsg>);
    }

    #[derive(Debug, Default)]
    pub struct Space {
        pub id: SpaceId,
        pub size: f32,
    }

    impl Space {
        pub fn new(id: SpaceId, size: f32) -> Self {
            Self {
                id,
                size,
            }
        }
    }

    impl Widget for Space {
        fn invoke(&mut self, ui: &mut egui::Ui, _out: &mut Vec<WidgetMsg>) {
            ui.add_space(self.size);
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
        pub id: ButtonId,
        pub label: String,
        pub width: f32,
        pub height: f32,
    }

    impl Button {
        // Constructor method
        pub fn new(id: ButtonId, label: impl Into<String>, width: f32, height: f32) -> Self {
            Self {
                id,
                label: label.into(),
                width,
                height,
            }
        }
    }

    impl Widget for Button {
        fn invoke(&mut self, ui: &mut egui::Ui, out: &mut Vec<WidgetMsg>) {
            let resp = ui.add_sized(
                egui::vec2(self.width, self.height),
                egui::Button::new(&self.label),
            );

            if resp.clicked() {
                out.push(WidgetMsg::ButtonClicked(self.id));
            }
        }
    }

    #[derive(Debug)]
    pub struct Slider {
        id: SliderId,
        label: String,
        value: f32,
        range: std::ops::RangeInclusive<f32>,
    }
    impl Slider {
        pub fn new(
            id: SliderId,
            label: impl Into<String>,
            value: f32,
            range: std::ops::RangeInclusive<f32>,
        ) -> Self {
            Self {
                id,
                label: label.into(),
                value,
                range,
            }
        }

        pub fn value(&self) -> f32 {
            self.value
        }
    }

    impl Widget for Slider {
        fn invoke(&mut self, ui: &mut egui::Ui, out: &mut Vec<WidgetMsg>) {
            let resp =
                ui.add(egui::Slider::new(&mut self.value, self.range.clone()).text(&self.label));

            if resp.changed() {
                out.push(WidgetMsg::SliderChanged(self.id, self.value));
            }
        }
    }

    #[derive(Debug)]
    pub struct DragFloat {
        id: DragFloatId,
        label: String,
        value: f32,
        range: std::ops::RangeInclusive<f32>,
        decimal: usize,
        speed: f64,
    }
    impl DragFloat {
        pub fn new(
            id: DragFloatId,
            label: impl Into<String>,
            value: f32,
            range: std::ops::RangeInclusive<f32>,
        ) -> Self {
            Self {
                id,
                label: label.into(),
                value,
                range,
                decimal: 0,
                speed: 1.0,
            }
        }

        pub fn value(&self) -> f32 {
            self.value
        }
        pub fn set_decimal(&mut self, decimal: usize) {
            self.decimal = decimal;
        }
        pub fn set_speed(&mut self, speed: f64) {
            self.speed = speed;
        }
    }

    impl Widget for DragFloat {
        fn invoke(&mut self, ui: &mut egui::Ui, out: &mut Vec<WidgetMsg>) {
            let resp = ui.add(
                egui::DragValue::new(&mut self.value)
                    .range(self.range.clone())
                    .prefix(&self.label)
                    .fixed_decimals(self.decimal)
                    .speed(self.speed),
            );

            if resp.changed() {
                out.push(WidgetMsg::DragFloatChanged(self.id, self.value));
            }
        }
    }

    //---------------------------------------------------------------------------

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

    pub trait Shape: std::fmt::Debug {
        fn base(&self) -> &ShapeBase;
        fn base_mut(&mut self) -> &mut ShapeBase;

        /// Draw in *canvas-local* coordinates, translated by `canvas_offset`
        /// where `canvas_offset` is the screen-space top-left of the canvas.
        fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2);

        /// Convenience: draw with canvas at (0,0)
        fn draw(&self, painter: &egui::Painter) {
            self.draw_at(painter, egui::Vec2::ZERO);
        }

        fn location(&self) -> Pos2 {
            self.base().location()
        }

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
        fn set_line_style(&mut self, ls: LineStyle) {
            self.base_mut().set_line_style(ls)
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
                line_style: LineStyle::Solid,
                //line_style: LineStyle::Dashed { dash: 8.0, gap: 4.0 },
                //line_style: LineStyle::Dashed,
                //line_style: LineStyle::Dotted { spacing: 8.0, radius: 2.0 },
                //line_style: LineStyle::Dotted,
            }
        }
    }

    impl ShapeBase {
        /// Create a new, empty ShapeBase with default values.
        // pub fn new() -> Self {
        //     Self::default()
        // }
        pub fn location(&self) -> Pos2 {
            self.location
        }
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

        pub fn set_line_style(&mut self, ls: LineStyle) {
            self.line_style = ls;
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

        fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
            let translation = self.base.location.to_vec2() + canvas_offset;

            let points = self.base.points_translated(translation);
            let stroke = egui::Stroke::new(self.base.line_width, self.base.color);

            match self.base.line_style {
                LineStyle::Solid => {
                    painter.add(eframe::epaint::PathShape::line(points, stroke));
                }
                LineStyle::Dashed => {
                    let shapes = eframe::egui::Shape::dashed_line(
                        &points,
                        stroke,
                        self.base.dash_length(),
                        self.base.dash_gap(),
                    );
                    painter.extend(shapes);
                }
                LineStyle::Dotted => {
                    let shapes = eframe::egui::Shape::dotted_line(
                        &points,
                        self.base.color,
                        self.base.dot_spacing(),
                        self.base.dot_radius(),
                    );
                    painter.extend(shapes);
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

        fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
            let center = self.base.location + canvas_offset;

            painter.circle(
                center,
                self.radius,
                self.base.fill_color,
                egui::Stroke::new(self.base.line_width, self.base.color),
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

        fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
            let rect = Rect::from_center_size(self.base.location + canvas_offset, self.size);
            painter.rect(
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
    use crate::gui_lib::LayoutStyle::{NoPanel, SidePanel, TopPanel};
    use crate::gui_lib::{BKG_EXAMPLE, BKG_WINDOWS};
    use crate::gui_lib::{
        BasicCanvas, Button, Circle, Color32, Polyline, Rectangle, Slider, DragFloat, Space,
    };
    use crate::gui_lib::{ButtonId, DragFloatId, Shape, ShapeHandle, SliderId, SpaceId, WidgetMsg};
    use crate::gui_lib::{LineStyle::*, World};
    use eframe::egui::Context;
    use std::cell::RefCell;
    use std::rc::Rc;

    //const SPACE: SpaceId = SpaceId(1);
    const SPACE: SpaceId = SpaceId;
    //const SPACE: SpaceId;
    const SLIDER_GAUGE: SliderId = SliderId(1);
    const SLIDER_ANOTHER: SliderId = SliderId(2); // Not used in this demo
    const DRAGFLOAT_GAUGE: DragFloatId = DragFloatId(1);

    const BTN_STATE_A: ButtonId = ButtonId(1);
    const BTN_STATE_B: ButtonId = ButtonId(2);

    #[derive(Debug)]
    struct Gauge {
        pointer: f64,
    }

    impl Gauge {
        fn new() -> Self {
            Self { pointer: 0.0 }
        }
        fn pointer(&self) -> f64 {
            self.pointer
        }

        fn set_pointer(&mut self, pointer: f64) {
            self.pointer = pointer;
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Signal {
        Stop,
        Go,
    }
    #[derive(Debug)]
    struct TrafficLight {
        state: Signal,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum ThingState {
        StateA,
        StateB,
        StateC,
    }
    #[derive(Debug)]
    struct Thing {
        state: ThingState,
    }

    #[derive(Debug)]
    struct TheWorld {
        state: i32,
        tl: TrafficLight,
        thing: Thing,
        gauge: Gauge,
    }

    impl World for TheWorld {
        fn advance(&mut self) {
            self.state += 1;
            self.toggle_light();
        }
    }

    impl TheWorld {
        fn new() -> Self {
            Self {
                state: 0,
                tl: TrafficLight {
                    state: Signal::Stop,
                },
                thing: Thing {
                    state: ThingState::StateC,
                },
                gauge: Gauge::new(),
            }
        }

        fn toggle_light(&mut self) {
            self.tl.state = match self.tl.state {
                Signal::Stop => Signal::Go,
                Signal::Go => Signal::Stop,
            };
        }
    }
    #[derive(Debug)]
    pub struct TheCanvas {
        canvas: BasicCanvas,
        sc1: Rc<RefCell<Circle>>,
        sc2: Rc<RefCell<Circle>>,
        sr: Rc<RefCell<Rectangle>>,
        sp: Rc<RefCell<Polyline>>,
        arrow_head: Rc<RefCell<Polyline>>,
    }

    impl TheCanvas {
        pub fn new() -> Self {
            // New empty BasicCanvas
            let mut canvas = BasicCanvas::new(TopPanel, BKG_EXAMPLE);

            // Add shapes without handles to the canvas
            let mut y = 75.0;
            for _ in 0..22 {
                //note: vee will be lost. It will not be a field in Self
                let vee: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
                    eframe::egui::Pos2::new(150.0, y),
                    [
                        eframe::egui::Pos2::new(0.0, 0.0),
                        eframe::egui::Pos2::new(10.0, 10.0),
                        eframe::egui::Pos2::new(20.0, 0.0),
                    ],
                )));
                //vee.borrow_mut().set_line_width(4.0);
                let vee_cln: ShapeHandle = vee.clone();
                canvas.add_shape(vee_cln);
                y += 10.0;
            }

            // Add shape with handle
            let sc1: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
                eframe::egui::Pos2::new(200.0, 200.0),
                //eframe::egui::Pos2::new(0.0, 0.0),  // to test origin
                75.0,
            )));
            sc1.borrow_mut().set_line_width(4.0);
            sc1.borrow_mut().set_fill_color(Color32::GRAY);
            let sc1_cln: ShapeHandle = sc1.clone();
            canvas.add_shape(sc1_cln);

            // Add shape with handle
            let sc2: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
                eframe::egui::Pos2::new(200.0, 200.0),
                10.0,
            )));
            let sc2_cln: ShapeHandle = sc2.clone();
            canvas.add_shape(sc2_cln);

            // Add shape with handle
            let sr: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new(
                eframe::egui::Pos2::new(400.0, 200.0),
                eframe::egui::Vec2::new(150.0, 100.0),
            )));
            sr.borrow_mut().set_fill_color(Color32::LIGHT_GRAY);
            let sr_cln: ShapeHandle = sr.clone();
            canvas.add_shape(sr_cln);

            // Add shape with handle
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
            sp.borrow_mut().set_color(Color32::RED);
            sp.borrow_mut().set_line_width(2.0);
            sp.borrow_mut().set_line_width(4.0);
            //sp.borrow_mut().set_line_style(Dashed);
            sp.borrow_mut().set_line_style(Dotted);
            //sp.borrow_mut().set_line_style(Solid);
            let sp_cln: ShapeHandle = sp.clone();
            canvas.add_shape(sp_cln);

            // Add shape with handle
            // TDJ: change to left upper corner when possible
            let gauge: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new(
                eframe::egui::Pos2::new(500.0, 350.0),
                eframe::egui::Vec2::new(850.0, 50.0),
            )));
            gauge.borrow_mut().set_fill_color(Color32::LIGHT_GRAY);
            let gauge_cln: ShapeHandle = gauge.clone();
            canvas.add_shape(gauge_cln);

            // Add shape with handle
            let arrow_head: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
                eframe::egui::Pos2::new(100.0, 369.0),
                [
                    eframe::egui::Pos2::new(-4.0, 0.0),
                    eframe::egui::Pos2::new(0.0, -39.0),
                    eframe::egui::Pos2::new(4.0, 0.0),
                ],
            )));
            arrow_head.borrow_mut().set_line_width(2.0);
            let arrow_head_cln: ShapeHandle = arrow_head.clone();
            canvas.add_shape(arrow_head_cln);

            // Create and add widgets as Box<dyn Widget>
            // let ws1 = Slider::new(SLIDER_GAUGE, "Gauge", 0.0, 0.0..=100.0);
            // canvas.add_widget(Box::new(ws1));

            let wb_a = Button::new(BTN_STATE_A, "State A", 120.0, 40.0);
            canvas.add_widget(Box::new(wb_a));

            let spc = Space::new(SPACE, 50.0);
            canvas.add_widget(Box::new(spc));

            let wb_b = Button::new(BTN_STATE_B, "State B", 120.0, 40.0);
            canvas.add_widget(Box::new(wb_b));

            let mut wdf1 = DragFloat::new(DRAGFLOAT_GAUGE, "Gauge = ", 0.0, 0.0..=100.0);
            //wdf1.set_decimal(1);
            //wdf1.set_speed(0.1);
            canvas.add_widget(Box::new(wdf1));

            //Create the TheCanvas
            Self {
                canvas,
                sc1,
                sc2,
                sr,
                sp,
                arrow_head,
            }
        }

        pub fn canvas(&self) -> &BasicCanvas {
            &self.canvas
        }
        pub fn canvas_mut(&mut self) -> &mut BasicCanvas {
            &mut self.canvas
        }

        fn update(&mut self, world: &TheWorld) {
            // Get state of traffic light and set appropriate color
            let tlc = if world.tl.state == Signal::Stop {
                Color32::RED
            } else {
                Color32::GREEN
            };
            self.sc2.borrow_mut().set_fill_color(tlc);

            // Update gauge pointer
            let mut ah_pos = self.arrow_head.borrow_mut().location();
            ah_pos.x = 100.0 + 8.0 * (world.gauge.pointer() as f32);
            self.arrow_head.borrow_mut().move_to(ah_pos);

            // Update thing state, color coded
            match world.thing.state {
                ThingState::StateA => {
                    self.sr.borrow_mut().set_fill_color(Color32::GOLD);
                }
                ThingState::StateB => {
                    self.sr.borrow_mut().set_fill_color(Color32::CYAN);
                }
                _ => {}
            }
        }
    }

    /// Main application structure.
    ///
    /// Represents the root of the application and contains
    /// the main canvas with all UI components.
    #[derive(Debug)]
    struct TheApp {
        world: Box<TheWorld>,
        canvas: TheCanvas,
        msgs: Vec<WidgetMsg>,
        last_toggle: f64,
    }

    impl TheApp {
        /// Creates a new instance of the application.
        ///
        /// # Returns
        /// A new `DemoApp` instance initialized with a default canvas
        /// containing several shapes
        /// and containing a sample button.
        pub fn new() -> Self {
            Self {
                world: Box::new(TheWorld::new()),
                canvas: TheCanvas::new(),
                msgs: Vec::new(), //TDJ:wid is this good
                last_toggle: 0.0, //For time-gating
                                  //is_red: true,
            }
        }
        //}

        //impl TheApp {
        fn handle_msg(&mut self, msg: WidgetMsg) {
            match msg {
                WidgetMsg::ButtonClicked(id) => {
                    self.handle_button(id);
                }
                WidgetMsg::SliderChanged(id, value) => {
                    self.handle_slider(id, value);
                }
                WidgetMsg::DragFloatChanged(id, value) => {
                    self.handle_drag_float(id, value);
                }
            }
        }

        //impl TheApp {
        fn handle_button(&mut self, id: ButtonId) {
            match id {
                BTN_STATE_A => {
                    self.world.thing.state = ThingState::StateA;
                }
                BTN_STATE_B => {
                    self.world.thing.state = ThingState::StateB;
                }
                _ => {}
            }
        }

        fn handle_slider(&mut self, id: SliderId, value: f32) {
            match id {
                SLIDER_GAUGE => {
                    self.world.gauge.set_pointer(value.into());
                }
                SLIDER_ANOTHER => {
                    //Do something else
                }
                _ => {}
            }
        }

        fn handle_drag_float(&mut self, id: DragFloatId, value: f32) {
            match id {
                DRAGFLOAT_GAUGE => {
                    self.world.gauge.set_pointer(value.into());
                }
                _ => {}
            }
        }
    }

    // The eframe::App trait is the bridge between your custom application logic
    // and the eframe framework that handles all the platform-specific details
    // of creating a window and running an event loop.
    impl eframe::App for TheApp {
        fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
            //Test of basic simulation/animation  //TDJ
            let now = ctx.input(|i| i.time);
            if now - self.last_toggle >= 0.5 {
                self.last_toggle = now;
                self.world.advance(); // advance world one tick
                self.canvas.update(&self.world); // update canvas
            }

            self.msgs.clear(); // establish invariant: Belt and suspenders
            // Draw shapes and widgets on the canvas, and collect all messages from widgets
            self.canvas.canvas.render(ctx, &mut self.msgs);

            if !self.msgs.is_empty() {
                // Move msgs out of self so we can mutably borrow self inside the loop.
                let mut msgs = std::mem::take(&mut self.msgs);

                // Handle messages
                for msg in msgs.drain(..) {
                    self.handle_msg(msg);
                }
                // Put the buffer back (empty, but keeps its capacity).
                self.msgs = msgs;

                // Update canvas once after all state changes:
                self.canvas.update(&self.world);
            }
            // schedule the next frame redraw after 16 milliseconds (60 FPS)
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
    }

    pub fn run_the_app() -> Result<(), eframe::Error> {
        eframe::run_native(
            "GUI Draw Example",
            crate::gui_lib::native_options(),
            Box::new(|cc| {
                cc.egui_ctx.set_visuals(eframe::egui::Visuals::light()); //light theme
                //cc.egui_ctx.set_visuals(eframe::egui::Visuals::dark()); //dark theme
                let app = Box::new(TheApp::new());
                Ok(app)
            }),
        )
    }
} // module demo

// /// Exposed publicly
//pub use demo::TheApp;
//pub use eframe::egui::vec2;
//pub use gui_lib::{Button, Draw, Canvas, custom_light_visuals};
//pub use gui_lib::{BasicCanvas, Button, custom_light_visuals};

// Jan 28. Space
