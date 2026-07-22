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
    BasicCanvas, Button, Circle, Color32, DragFloat, Label, Polyline, Rectangle, Separator, Shape,
    Space, Text,
};
//use crate::world::emerge::BitArray;

#[derive(Debug)]
struct ViewHandles {
    stxt_frame: Rc<RefCell<Text>>,
    stxt_rule: Rc<RefCell<Text>>,
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
                let xpx = 75.0 + ((x % GRID_WIDTH) as f32) * 10.0;
                let ypx = 75.0 + y as f32 * 10.0;
                let bit_disp: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new(
                    egui::Pos2::new(xpx, ypx),
                    egui::Vec2::new(10.0, 10.0),
                )));
                bit_disp.borrow_mut().set_color(Color32::GRAY);
                bit_disp.borrow_mut().set_fill_color(Color32::BLACK);
                canvas.add_shape(bit_disp.clone());
            }
        }
        // ---------------------------------

        let stxt_rule: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            //eframe::egui::Pos2::new(250.0, 270.0),
            egui::Pos2::new(10.0, 10.0),
            format!("Rule: _"),
        )));
        canvas.add_shape(stxt_rule.clone()); // coercion to ShapeHandle happens automatically

        // frame number.
        let stxt_frame: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            //eframe::egui::Pos2::new(250.0, 270.0),
            egui::Pos2::new(200.0, 10.0),
            format!("Interactions: {}", 0),
        )));
        canvas.add_shape(stxt_frame.clone()); // coercion to ShapeHandle happens automatically




        ViewHandles {
            // Shapes as unique handles to a concrete struct (e.g. Rc<RefCell<Circle>>)
            stxt_rule,
            stxt_frame,
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

        // Set stxt_rule to display rule number
        self.view_handles
            .stxt_rule
            .borrow_mut()
            .set_text(format!("Rule: {}", world.rule.number()));


            // Set stxt_frame to display frame number
            self.view_handles
                .stxt_frame
                .borrow_mut()
                .set_text(format!("Interactions: {}", world.frame_number));
        }
} // end of impl TheCanvas
