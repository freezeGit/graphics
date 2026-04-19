//! ## Module canvas contains the BasicCanvas struct.
//!
//! This struct is intended to be contained (by composition) in any application canvas.
// canvas_gl

use std::cell::RefCell;
use std::rc::Rc;

use crate::egui;
use eframe::egui::{CentralPanel, Context, RichText};

use crate::{Color32, Dialog, NilDlg, Shape, Widget, WidgetMsg};

/// Handle for Shapes in BasicCanvas::shapes: Vec<ShapeHandle>
///
/// [`ShapeHandle`] = Rc<RefCell<dyn Shape>>
/// ShapeHandle is a smart pointer that can be cloned.
/// The RefCell interior mutability allows interior mutability.
pub type ShapeHandle = Rc<RefCell<dyn Shape>>;

/// enum for canvas layout styles
#[derive(Debug)]
pub enum LayoutStyle {
    TopPanel,
    SidePanel,
    NoPanel,
}

// Examples of background colors. Any Color32 will do.
pub const BKG_DEFAULT: Color32 = Color32::from_rgb(200, 200, 210);
pub const BKG_WINDOWS: Color32 = Color32::from_rgb(240, 240, 240);

//------------------------------------------------

/// A container for drawable components.
///
/// Canvas acts as a container that can hold and manage multiple
/// UI components that implement the `Draw` trait.
#[derive(Debug)]
pub struct BasicCanvas {
    layout: LayoutStyle,
    pub background_color: Color32,
    shapes: Vec<ShapeHandle>, // Vec<Rc<RefCell<dyn Shape>>>
    widgets: Vec<Box<dyn Widget>>,
    pub active_dialog: Box<dyn Dialog>,
}

/// BasicCanvas provides underlying structure and functionality for any user canvas.
/// Shapes are stored in BasicCanvas::shapes: Vec<ShapeHandle> (type ShapeHandle = Rc<RefCell<dyn Shape>>)
/// and are drawn dynamically by iterating through the vector

impl BasicCanvas {
    pub fn new(layout: LayoutStyle, bkg: Color32) -> Self {
        BasicCanvas {
            layout,
            background_color: bkg,
            shapes: Vec::new(),
            widgets: Vec::new(),
            active_dialog: Box::new(NilDlg),
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

    /// Add a [`Shape`] to the canvas.
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

    /// Add a [`Widget`] to the canvas
    pub fn add_widget(&mut self, w: Box<dyn Widget>) {
        self.widgets.push(w);
    }

    // Dialog in canvas --------------------------------------------------
    /// Set the ative [`Dialog`] in the canvas.
    pub fn set_dialog(&mut self, dlg: Box<dyn Dialog>) {
        self.active_dialog = dlg;
    }

    pub fn get_dialog(&self) -> &dyn Dialog {
        self.active_dialog.as_ref()
    }

    pub fn get_mut_dialog(&mut self) -> &mut dyn Dialog {
        self.active_dialog.as_mut()
    }

    // Rendering canvas ---------------------------------------------

    /// Renders all widgets and shapes.
    ///
    /// Modifies the vector `out`
    /// to hold a sequence of tagged messages of type [`WidgetMsg`].
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
//- -------------------------
