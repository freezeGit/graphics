//! ## module Canvas. Contains TheCanvas struct.
//! Declaration for struct [`TheCanvas`]:
//! A container for rendering and managing graphical shapes
//! and interactive widgets.

// canvas_gl

// Submodule under mod canvas.
pub mod seq_graph;

use std::cell::RefCell;
use std::rc::Rc;

use crate::canvas::seq_graph::SeqGraph;
use crate::ids::*;
use crate::inits;
use crate::world::TheWorld;
#[allow(unused_imports)]
use gui_lib::LineStyle::{Dashed, Dotted, Solid};
use gui_lib::{BasicCanvas, Button, Color32, Label, Line, Lines, Pos2, Shape, Space, Text, Vec2};

#[derive(Debug)]
pub struct ViewHandles {
    stxt_bits: Rc<RefCell<Text>>,
    stxt_ones: Rc<RefCell<Text>>,
    stxt_rule: Rc<RefCell<Text>>,
    stxt_frame: Rc<RefCell<Text>>,
    pub stxt_batch: Rc<RefCell<Text>>,
    pub stxt_scale: Rc<RefCell<Text>>,
    pub stxt_focus: Rc<RefCell<Text>>,
    sln2: Rc<RefCell<Line>>,
    pub sgr: Rc<RefCell<SeqGraph>>,
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

        let sgr: Rc<RefCell<SeqGraph>> =
           //Rc::new(RefCell::new(SeqGraph::new(egui::pos2(50.0, 600.0))));
            Rc::new(RefCell::new(SeqGraph::new(inits::SEQ_GRAPH_POSITION)));
        canvas.add_shape(sgr.clone());

        let stxt_bits: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(10.0, 10.0),
            format!("Bits: {}", inits::INITIAL_BITS_NUM),
        )));
        canvas.add_shape(stxt_bits.clone());

        let stxt_ones: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(175.0, 10.0),
            format!("Ones: {}", inits::INITIAL_ONES),
        )));
        canvas.add_shape(stxt_ones.clone());

        let stxt_rule: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(360.0, 10.0),
            format!("Rule: {}", inits::INITIAL_RULE),
        )));
        canvas.add_shape(stxt_rule.clone()); // coercion to ShapeHandle happens automatically

        // frame number.
        let stxt_frame: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            //eframe::egui::Pos2::new(250.0, 270.0),
            //egui::Pos2::new(200.0, 10.0),
            egui::Pos2::new(525.0, 10.0),
            format!("Interactions: {}", 0),
        )));
        canvas.add_shape(stxt_frame.clone()); // coercion to ShapeHandle happens automatically

        let stxt_batch: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(10.0, 45.0),
            format!("Batch: {}", inits::BATCH_SIZE),
        )));
        canvas.add_shape(stxt_batch.clone());

        let stxt_scale: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(200.0, 45.0),
            format!("Scale: {}", inits::SEQ_GRAPH_SCALE),
        )));
        canvas.add_shape(stxt_scale.clone());

        let stxt_focus: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(320.0, 45.0),
            format!("Focus: {}", inits::SEQ_GRAPH_FOCUS),
        )));
        canvas.add_shape(stxt_focus.clone());

        let sln1: Rc<RefCell<Line>> = Rc::new(RefCell::new(Line::new(
            Pos2::new(100.0, 705.0),
            Vec2::new(950.0, 0.0),
        )));
        sln1.borrow_mut().set_line_width(8.0);
        sln1.borrow_mut().set_color(Color32::LIGHT_GRAY);
        canvas.add_shape(sln1.clone());

        let sln2: Rc<RefCell<Line>> = Rc::new(RefCell::new(Line::new(
            Pos2::new(100.0, 705.0),
            Vec2::new(950.0, 0.0),
        )));
        sln2.borrow_mut().set_line_width(8.0);
        sln2.borrow_mut().set_color(Color32::DARK_BLUE);
        canvas.add_shape(sln2.clone());

        let tics: Rc<RefCell<Lines>> = Rc::new(RefCell::new(Lines::new(
            //Pos2::new(250.0, 705.0),
            Pos2::new(100.0, 705.0),
            vec![
                [Pos2::new(0.0, -16.0), Pos2::new(0.0, 16.0)],
                [Pos2::new(237.5, -16.0), Pos2::new(237.5, 16.0)],
                [Pos2::new(475.0, -16.0), Pos2::new(475.0, 16.0)],
                [Pos2::new(712.5, -16.0), Pos2::new(712.5, 16.0)],
                [Pos2::new(950.0, -16.0), Pos2::new(950.0, 16.0)],
            ],
        )));
        canvas.add_shape(tics.clone());

        ViewHandles {
            // Shapes as unique handles to a concrete struct (e.g. Rc<RefCell<Circle>>)
            stxt_bits,
            stxt_ones,
            stxt_rule,
            stxt_frame,
            stxt_batch,
            stxt_scale,
            stxt_focus,
            sln2,
            sgr,
        }
    }

    // Create and add widgets as Box<dyn Widget>
    fn init_widgets(canvas: &mut BasicCanvas) {
        // ---- Create and add widgets as Box<dyn Widget>
        canvas.add_widget(Box::new(Space::new(15.0)));

        let label1 = Label::new("App3", Color32::BLUE, 20.0);
        canvas.add_widget(Box::new(label1));

        canvas.add_widget(Box::new(Space::new(15.0)));

        let wb_specs = Button::new(BTN_NEW_SIM, "New Sim", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_specs));

        let wb_sim = Button::new(BTN_SIM, "Run Sim", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_sim));

        let wb_batch = Button::new(BTN_BATCH, "Batch", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_batch));

        let wb_zoom = Button::new(BTN_ZOOM, "Zoom", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_zoom));

        canvas.add_widget(Box::new(Space::new(25.0)));

        let wb_seq = Button::new(BTN_SEQ, "Sequence", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_seq));

        canvas.add_widget(Box::new(Space::new(250.0)));

        let wb_about = Button::new(BTN_ABOUT, "About", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_about));
    }

    // --------------------------------------
    //TDJ: not used. Should it be?
    // pub(crate) fn canvas(&self) -> &BasicCanvas {
    //     &self.canvas
    // }
    //TDJ: not used.  Should it be?
    // pub(crate) fn canvas_mut(&mut self) -> &mut BasicCanvas {
    //     &mut self.canvas
    // }

    /// Update the state of the canvas based on the current world state.
    ///
    /// Note that this method does not modify the world state.
    /// The world does not know about the canvas (nor about egui). This is important to keep the
    /// separation of concerns. Program data and logic is encapsulated in the [`TheWorld`] struct.
    pub fn update(&mut self, world: &TheWorld) {
        // Set stxt_bits to display bits number
        self.view_handles
            .stxt_bits
            .borrow_mut()
            .set_text(format!("Bits: {}", world.bits.len()));

        // Set stxt_ones to display ones number
        self.view_handles
            .stxt_ones
            .borrow_mut()
            .set_text(format!("Ones: {}", world.bits.ones_count()));

        // Set stxt_rule to display rule number
        self.view_handles
            .stxt_rule
            .borrow_mut()
            .set_text(format!("Rule: {}", world.rule.number()));

        // Set stxt_frame to display interactionss number
        self.view_handles
            .stxt_frame
            .borrow_mut()
            .set_text(format!("Interactions: {}", world.frame_number));

        // Update the sequence graph
        let val = world.bits.ones_fraction() as f32;
        self.view_handles.sgr.borrow_mut().add_val(val);

        // Update the line length
        let length = 950.0 * (world.bits.ones_fraction() as f32);
        self.view_handles.sln2.borrow_mut().set_length(length);
    }
} // end of impl TheCanvas
