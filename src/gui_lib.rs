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
/// including buttons, canvas and visual styling utilities. It implements
/// a custom drawing system through the `Draw` trait. It implements a custom widget system through the
/// 'Widget' trait, and a custom modal dialog system through th 'do_modal' trait.

pub use eframe::egui::{
    self as egui,
    Button as EguiButton,
    Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2, Visuals,
    pos2, vec2,
};

use egui::{CentralPanel, Context, RichText};
use std::{cell::RefCell, rc::Rc};

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

#[derive(Debug)]
pub struct Timer {
    interval: f64,
    last_time: f64,
    running: bool,
}

impl Timer {
    pub fn new(interval: f64) -> Self {
        Timer {
            interval,
            last_time: 0.0,
            running: false,
        }
    }

    pub fn is_time(&mut self, ctx: &Context) -> bool {
        let mut retn = false;
        if self.running {
            let now = ctx.input(|i| i.time);
            if now - self.last_time >= self.interval {
                self.last_time = now;
                retn = true;
            }
        }
        retn
    }
    pub fn interval(&self) -> f64 {
        self.interval
    }
    pub fn set_interval(&mut self, interval: f64) {
        self.interval = interval;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    // Frozen time
    // pub fn run(&mut self, ctx: &Context) {
    //     self.running = true;
    //     self.last_time = ctx.input(|i| i.time);
    // }

    // Not using "frozen time" because ctx may not be available easily.
    pub fn run(&mut self) {
        self.running = true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }
}

//------------------------------------

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
    widgets: Vec<Box<dyn Widget>>,
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
                //ui.heading("Controls");  // only if you want side panel to be labelled
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
pub struct ButtonId(pub WidgetId);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SliderId(pub WidgetId);
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragFloatId(pub WidgetId);

// -------------------------------

pub type DialogId = u32; // TDJd

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextEntryDlgId(pub DialogId);
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragFloatDlgId(pub DialogId);

#[derive(Debug, Clone, PartialEq)]
pub enum WidgetMsg {
    // Widget outcomes:
    ButtonClicked(ButtonId),
    SliderChanged(SliderId, f32),
    DragFloatChanged(DragFloatId, f32),
    // Dialog outcomes:
    DialogAcceptedText(TextEntryDlgId, String),
    //DialogCanceled(TextEntryDlgId),  // TDJd
    DialogAcceptedDragFloat(DragFloatDlgId, f32), // TDJd
}

/// Trait for invoking any widget in the UI.
pub trait Widget: std::fmt::Debug {
    fn invoke(&mut self, ui: &mut egui::Ui, out: &mut Vec<WidgetMsg>);
}

#[derive(Debug, Default)]
pub struct Space {
    pub size: f32,
}

impl Space {
    pub fn new(size: f32) -> Self {
        Self { size }
    }
}

impl Widget for Space {
    fn invoke(&mut self, ui: &mut egui::Ui, _out: &mut Vec<WidgetMsg>) {
        ui.add_space(self.size);
    }
}

#[derive(Debug, Default)]
pub struct Separator;

impl Separator {
    pub fn new() -> Self {
        Self {}
    }
}
impl Widget for Separator {
    fn invoke(&mut self, ui: &mut egui::Ui, _out: &mut Vec<WidgetMsg>) {
        ui.separator();
    }
}

#[derive(Debug, Default)]
pub struct Label {
    pub text: String,
    pub color: Color32,
    pub size: f32,
}

impl Label {
    pub fn new(text: impl Into<String>, color: Color32, size: f32) -> Self {
        Self {
            text: text.into(),
            color,
            size,
        }
    }
}

impl Widget for Label {
    fn invoke(&mut self, ui: &mut egui::Ui, _out: &mut Vec<WidgetMsg>) {
        ui.label(RichText::new(&self.text).color(self.color).size(self.size));
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
            egui::Button::new(egui::RichText::new(&self.label).size(14.0).strong()),
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
        let resp = ui.add(egui::Slider::new(&mut self.value, self.range.clone()).text(&self.label));

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

// ------------------------------

pub trait Dialog: std::fmt::Debug {
    /// Returns true if it closed this frame.
    fn do_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool;
}

#[derive(Debug)]
pub struct TextEntryDlg {
    egui_id: egui::Id, // What for? Why are fields pub?
    id: TextEntryDlgId,
    title: String,
    prompt: String,
    text: String,
}

impl TextEntryDlg {
    pub fn new(
        id: TextEntryDlgId,
        title: impl Into<String>,
        prompt: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            egui_id: egui::Id::new(("text_entry_dialog", id)),
            id,
            title: title.into(),
            prompt: prompt.into(),
            text: text.into(),
        }
    }
}

impl Dialog for TextEntryDlg {
    fn do_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool {
        let mut close = false;

        egui::Modal::new(self.egui_id).show(ctx, |ui| {
            ui.heading(&self.title);
            ui.separator();

            ui.label(&self.prompt);
            ui.text_edit_singleline(&mut self.text);

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    //out.push(WidgetMsg::DialogAcceptedText(self.id, self.text.clone()));
                    out.push(WidgetMsg::DialogAcceptedText(self.id, self.text.clone()));
                    close = true;
                }
                if ui.button("Cancel").clicked() {
                    close = true;
                }
            });
        });

        close
    }
}

// #[derive(Debug)]
// pub struct DragFloatDlg {
//     egui_id: egui::Id,
//     id: DragFloatDlgId,
//     title: String,
//     prompt: String,
//     value: f32,
//     range: std::ops::RangeInclusive<f32>,
//     decimal: usize,
//     speed: f64,
// }

#[derive(Debug)]
pub struct DragFloatDlg {
    egui_id: egui::Id,
    id: DragFloatDlgId,
    title: String,
    prompt: String,
    value: f32,
    //range: std::ops::RangeInclusive<f32>,
    decimal: usize,
    speed: f64,
}

impl DragFloatDlg {
    // pub fn new(
    //     id: DragFloatDlgId,
    //     title: impl Into<String>,
    //     prompt: impl Into<String>,
    //     value: f32,
    //     range: std::ops::RangeInclusive<f32>,
    // ) -> Self {
    //     Self {
    //         egui_id: egui::Id::new(("text_entry_dialog", id)),
    //         id,
    //         title: title.into(),
    //         prompt: prompt.into(),
    //         value,
    //         range,
    //         decimal: 0,
    //         speed: 1.0,
    //     }
    // }

    pub fn new(
        id: DragFloatDlgId,
        title: impl Into<String>,
        prompt: impl Into<String>,
        value: f32,
        //range: std::ops::RangeInclusive<f32>,
    ) -> Self {
        Self {
            egui_id: egui::Id::new(("text_entry_dialog", id)),
            id,
            title: title.into(),
            prompt: prompt.into(),
            value,
            //range,
            decimal: 0,
            speed: 1.0,
        }
    }

    pub fn set_decimal(&mut self, decimal: usize) {
        self.decimal = decimal;
    }
    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }
}

impl Dialog for DragFloatDlg {
    fn do_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool {
        let mut close = false;

        egui::Modal::new(self.egui_id).show(ctx, |ui| {
            ui.heading(&self.title);
            ui.separator();
            ui.label(&self.prompt);
            ui.add(
                egui::DragValue::new(&mut self.value)
                    //.range(self.range.clone())
                    .fixed_decimals(self.decimal)
                    .speed(self.speed),
            );
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    out.push(WidgetMsg::DialogAcceptedDragFloat(self.id, self.value));
                    close = true;
                }
                if ui.button("Cancel").clicked() {
                    close = true;
                }
            });
        });

        close
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
#[derive(Debug)]
pub enum TextFont {
    Proportional,
    Monospace,
}
//#[derive(Debug, Default)]
#[derive(Debug)]
pub struct Text {
    base: ShapeBase,
    text: String,
    color: Color32,
    size: f32,
    font: TextFont,
}

impl Text {
    pub fn new(top_left: Pos2, text: impl Into<String>) -> Self {
        Self {
            base: {
                ShapeBase {
                    location: top_left,
                    ..Default::default()
                }
            },
            text: text.into(),
            color: Color32::BLACK,
            size: 24.0,
            font: TextFont::Proportional,
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn set_color(&mut self, color: Color32) {
        self.color = color;
    }

    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }

    pub fn set_font(&mut self, font: TextFont) {
        self.font = font;
    }
}

impl Shape for Text {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        //let center = self.base.location + canvas_offset;
        let tl = self.base.location + canvas_offset;
        let font_id = match self.font {
            TextFont::Proportional => egui::FontId::proportional(self.size),
            TextFont::Monospace => egui::FontId::monospace(self.size),
        };

        painter.text(
            tl,
            egui::Align2::LEFT_TOP,
            self.text.as_str(),
            //FontId::proportional(20.0),
            font_id,
            //Color32::BLACK,
            self.color,
        );
    }
}
