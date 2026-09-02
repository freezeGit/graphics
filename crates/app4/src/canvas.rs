//! ## module Canvas. Contains TheCanvas struct.
//! Declaration for struct [`TheCanvas`]:
//! A container for rendering and managing graphical shapes
//! and interactive widgets.

// canvas_gl

// Submodule under mod canvas.
pub mod delta_graph;

use std::cell::RefCell;
use std::rc::Rc;

use crate::canvas::delta_graph::DeltaGraph;
use crate::ids::*;
use crate::inits;
use crate::world::TheWorld;
#[allow(unused_imports)]
use gui_lib::LineStyle::{Dashed, Dotted, Solid};
use gui_lib::{BasicCanvas, Button, Color32, Label, Line, Lines, Pos2, Shape, Space, Text, Vec2};

#[derive(Debug)]
pub struct ViewHandles {
    stxt_rule: Rc<RefCell<Text>>,
    stxt_sample_size: Rc<RefCell<Text>>,
    pub dgr: Rc<RefCell<DeltaGraph>>,
}

/// ## struct Canvas
/// A container for rendering and managing graphical shapes
/// and interactive widgets.
///
/// Owns the app1's BasicCanvas and selected concrete view handles.
/// Builds the visual scene and updates selected elements from TheWorld.

#[derive(Debug)]
pub struct TheCanvas {
    // BasicCanvas provides underlying canvas structure and functionality.
    // Shapes are stored in BasicCanvas::shapes: Vec<ShapeHandle>
    // (pub type ShapeHandle = Rc<RefCell<dyn Shape>> to allow dynamic update.)
    // Widgets are stored in BasicCanvas::Vec<Box<dyn Widget>>
    pub canvas: BasicCanvas, // From gui_lib

    // ViewHandles fields are concrete shapes as unique handles of type Rc<RefCell<T>>
    pub view_handles: ViewHandles,
}

impl TheCanvas {
    /// Constructor for TheCanvas.
    ///
    /// Creates and initializes a BasicCanvas
    /// Creates and initializes all shapes and widgets
    pub fn new() -> Self {
        let mut canvas = BasicCanvas::new(inits::LAYOUT_STYLE, inits::BACKGROUND_COLOR);
        Self::init_widgets(&mut canvas);
        let view_handles = Self::init_shapes(&mut canvas);

        Self {
            canvas,
            view_handles,
        }
    }

    fn init_shapes(canvas: &mut BasicCanvas) -> ViewHandles {
        // ---- Create shapes as Rc<RefCell<T>> and push clone into BasicCanvas::shapes: Vec<ShapeHandle>
        // Note: Rc<RefCell<T>> is a smart pointer that can be cloned.
        //       - The RefCell interior mutability allows interior mutability.
        //       - This is useful for updating properties of shapes.
        // Shapes are stored in BasicCanvas::shapes: Vec<ShapeHandle
        // (pub type ShapeHandle = Rc<RefCell<dyn Shape>> to allow dynamic update.)
        // Rc<RefCell<T>> coercion to ShapeHandle happens automatically
        // --------------------------

        let dgr: Rc<RefCell<DeltaGraph>> =
            Rc::new(RefCell::new(DeltaGraph::new(inits::DELTA_GRAPH_POSITION)));
        canvas.add_shape(dgr.clone());

        let stxt_rule: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(200.0, 10.0),
            format!("Rule: {}", inits::INITIAL_RULE),
        )));
        canvas.add_shape(stxt_rule.clone()); // coercion to ShapeHandle happens automatically

        let stxt_sample_size: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            //egui::Pos2::new(360.0, 10.0),
            egui::Pos2::new(350.0, 10.0),
            format!("Sample size: {}", inits::INITIAL_SAMPLE_SIZE),
        )));
        canvas.add_shape(stxt_sample_size.clone()); // coercion to ShapeHandle happens automatically

        ViewHandles {
            // Shapes as unique handles to a concrete struct (e.g. Rc<RefCell<Circle>>)
            stxt_rule,
            stxt_sample_size,
            dgr,
        }
    }

    // Create and add widgets as Box<dyn Widget>
    fn init_widgets(canvas: &mut BasicCanvas) {
        // ---- Create and add widgets as Box<dyn Widget>
        canvas.add_widget(Box::new(Space::new(15.0)));

        let label1 = Label::new("App4", Color32::BLUE, 20.0);
        canvas.add_widget(Box::new(label1));
        canvas.add_widget(Box::new(Space::new(15.0)));

        let wb_delta = Button::new(BTN_DELTAS, "Deltas", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_delta));

        canvas.add_widget(Box::new(Space::new(350.0)));

        let wb_about = Button::new(BTN_ABOUT, "About", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_about));
    }

    /// Update the state of the canvas based on the current world state.
    ///
    /// Note that this method does not modify the world state.
    /// The world does not know about the canvas (nor about egui). This is important to keep the
    /// separation of concerns. Program data and logic is encapsulated in the [`TheWorld`] struct.
    pub fn update(&mut self, world: &mut TheWorld) {
        // Set stxt_rule to display rule number
        self.view_handles
            .stxt_rule
            .borrow_mut()
            .set_text(format!("Rule: {}", world.rule.number()));

        self.view_handles
            .stxt_sample_size
            .borrow_mut()
            .set_text(format!("Sample size: {}", world.sample_size));
    }
} // end of impl TheCanvas
