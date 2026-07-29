//! ## module Canvas. Contains TheCanvas struct.
//! Declaration for struct [`TheCanvas`]:
//! A container for rendering and managing graphical shapes
//! and interactive widgets.

// canvas_gl

use std::cell::RefCell;
use std::rc::Rc;

use crate::ids::*;
use crate::inits;
//use crate::app_inits::{LAYOUT_STYLE, BACKGROUND_COLOR};
use crate::world::TheWorld;
//use crate::world::{Signal, TheWorld, ThingState};
//use crate::world::world_demo::{Signal, ThingState};
//use gui_lib::LayoutStyle::{NoPanel, SidePanel, TopPanel};
#[allow(unused_imports)]
use gui_lib::LineStyle::{Dashed, Dotted, Solid};
use gui_lib::{
    BasicCanvas, Button, Color32, Label, Line, Lines, Pos2, Rectangle, Separator, Shape, Space,
    Text, Vec2,
};
//use crate::world::emerge::BitArray;

#[derive(Debug)]
struct ViewHandles {
    stxt_bits: Rc<RefCell<Text>>,
    stxt_ones: Rc<RefCell<Text>>,
    stxt_rule: Rc<RefCell<Text>>,
    stxt_frame: Rc<RefCell<Text>>,
    sln2: Rc<RefCell<Line>>,
}

/// ## struct Canvas
/// A container for rendering and managing graphical shapes
/// and interactive widgets.
///
/// Owns the app1's BasicCanvas and selected concrete view handles.
/// Builds the visual scene and updates selected elements from TheWorld.

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 60;
const GRID_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
const CELL_SIZE: f32 = 10.0;
#[derive(Debug)]
pub(crate) struct TheCanvas {
    // BasicCanvas provides underlying canvas structure and functionality.
    // Shapes are stored in BasicCanvas::shapes: Vec<ShapeHandle>
    // (pub type ShapeHandle = Rc<RefCell<dyn Shape>> to allow dynamic update.)
    // Widgets are stored in BasicCanvas::Vec<Box<dyn Widget>>
    pub(crate) canvas: BasicCanvas, // From gui_lib

    // ViewHandles fields are concrete shapes as unique handles of type Rc<RefCell<T>>
    view_handles: ViewHandles,
}

impl TheCanvas {
    /// Constructor for TheCanvas.
    ///
    /// Creates and initializes a BasicCanvas
    /// Creates and initializes all shapes and widgets
    pub(crate) fn new() -> Self {
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
        // ----

        // --------------------------
        // TDJ: Must go first. Some day I will figure out how to do this better.
        // Create a grid of squares.
        // TDJ: Maybe run update to show sim specs
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let xpx = 75.0 + ((x % GRID_WIDTH) as f32) * CELL_SIZE;
                let ypx = 75.0 + y as f32 * CELL_SIZE;
                let bit_disp: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new(
                    egui::Pos2::new(xpx, ypx),
                    egui::Vec2::new(CELL_SIZE, CELL_SIZE),
                )));
                bit_disp.borrow_mut().set_color(Color32::GRAY);
                bit_disp.borrow_mut().set_fill_color(Color32::GRAY);
                //bit_disp.borrow_mut().set_fill_color(Color32::BLACK);
                canvas.add_shape(bit_disp.clone());
            }
        }
        // ---------------------------------

        let stxt_bits: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(10.0, 10.0),
            format!("Bits: {}", inits::INITIAL_BITS_NUM),
        )));
        canvas.add_shape(stxt_bits.clone());

        let stxt_ones: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(175.0, 10.0),
            format!("Ones: {}", 0),
            //format!("Ones: {}", inits::INITIAL_ONES),
        )));
        canvas.add_shape(stxt_ones.clone());

        let stxt_rule: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            //egui::Pos2::new(10.0, 10.0),
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
            sln2,
        }
    }

    // Create and add widgets as Box<dyn Widget>
    fn init_widgets(canvas: &mut BasicCanvas) {
        // ---- Create and add widgets as Box<dyn Widget>
        canvas.add_widget(Box::new(Space::new(15.0)));

        let label1 = Label::new("App2", Color32::BLUE, 20.0);
        canvas.add_widget(Box::new(label1));

        canvas.add_widget(Box::new(Space::new(15.0)));

        let wb_specs = Button::new(BTN_NEW_SIM, "New Sim", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_specs));

        let wb_sim = Button::new(BTN_SIM, "Run Sim", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_sim));

        let wb_batch = Button::new(BTN_BATCH, "Batch", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_batch));


        canvas.add_widget(Box::new(Space::new(300.0)));

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
    pub(crate) fn update(&mut self, world: &TheWorld) {
        let n = world.bits.len().min(GRID_SIZE);
        for i in 0..n {
            //let bit = world.bits.get(i);
            let bit = world.bits.get(i);
            //let col = if bit { Color32::LIGHT_RED } else { Color32::LIGHT_BLUE };
            let col = if bit { Color32::WHITE } else { Color32::BLACK };
            self.canvas.shapes[i].borrow_mut().set_fill_color(col);
        }
        for i in n..GRID_SIZE {
            self.canvas.shapes[i]
                .borrow_mut()
                .set_fill_color(Color32::GRAY);
        }

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
        let length = 950.0 * (world.bits.ones_fraction() as f32);

        self.view_handles
            .sln2
            .borrow_mut()
            .set_length(length);
    }
} // end of impl TheCanvas
