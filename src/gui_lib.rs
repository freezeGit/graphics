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
// ---- Public re-exports ----
pub use eframe::egui::{
    self as egui, Button as EguiButton, Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Ui,
    Vec2, Visuals, pos2, vec2,
};
use eframe::egui::{CentralPanel, Context, RichText};
use std::{cell::RefCell, rc::Rc};
// ---- Submodules ----
pub mod ids;
pub mod shapes;
pub mod widgets;

// ---- Public API ----
pub use ids::{
    ButtonId, DialogId, DragFloatDlgId, DragFloatId, MessageBoxDlgId, SliderId, TextEntryDlgId,
    WidgetId, WidgetMsg,
};
pub use shapes::{Circle, LineStyle, Polyline, Rectangle, Shape, ShapeBase, Text};
pub use widgets::{Button, DragFloat, Label, Separator, Slider, Space, Widget};

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

// ------------------------------

pub trait Dialog: std::fmt::Debug {
    /// Returns true if it closed this frame.
    fn do_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool;
}

// -------------------------------------

#[derive(Debug)]
pub struct MessageBoxDlg {
    egui_id: egui::Id,
    title: String,
    text: String,
}

impl MessageBoxDlg {
    pub fn new(
        id: MessageBoxDlgId, // TDJ id is used to create unique egui::Id. Is this necessary?
        title: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            egui_id: egui::Id::new(("message_box_dialog", id)),
            title: title.into(),
            text: text.into(),
        }
    }
}

impl Dialog for MessageBoxDlg {
    fn do_modal(&mut self, ctx: &egui::Context, _out: &mut Vec<WidgetMsg>) -> bool {
        let mut close = false;

        egui::Modal::new(self.egui_id).show(ctx, |ui| {
            ui.heading(&self.title);
            ui.separator();

            ui.label(&self.text);

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    close = true;
                }
            });
        });

        close
    }
}

// -------------------------------------

#[derive(Debug)]
pub struct TextEntryDlg {
    egui_id: egui::Id,
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

// -------------------------------------------
#[derive(Debug)]
pub struct DragFloatDlg {
    egui_id: egui::Id,
    id: DragFloatDlgId,
    title: String,
    prompt: String,
    value: f32,
    decimal: usize,
    speed: f64,
}

impl DragFloatDlg {
    pub fn new(
        id: DragFloatDlgId,
        title: impl Into<String>,
        prompt: impl Into<String>,
        value: f32,
    ) -> Self {
        Self {
            egui_id: egui::Id::new(("text_entry_dialog", id)),
            id,
            title: title.into(),
            prompt: prompt.into(),
            value,
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
