//! ## module Canvas. Contains TheCanvas struct.
//! Declaration for struct [`TheCanvas`]:
//! A container for rendering and managing graphical shapes
//! and interactive widgets.

// canvas_gl

use std::cell::RefCell;
use std::rc::Rc;

use crate::ids::*;
use crate::world::TheWorld;
//use crate::world::{Signal, TheWorld, ThingState};
use crate::world::world_demo::{Signal, ThingState};
use gui_lib::LayoutStyle::{NoPanel, SidePanel, TopPanel};
use gui_lib::LineStyle::{Dashed, Dotted, Solid};
use gui_lib::{
    BKG_DEFAULT, BasicCanvas, Button, Circle, Color32, DragFloat, Label, Polyline, Rectangle,
    Separator, Shape, Space, Text,
};

#[derive(Debug)]
struct ViewHandles {
    tl_circle2: Rc<RefCell<Circle>>,
    stxt_frame: Rc<RefCell<Text>>,
    rect: Rc<RefCell<Rectangle>>,
    arrow_head: Rc<RefCell<Polyline>>,
    stxt: Rc<RefCell<Text>>,
    stxtname: Rc<RefCell<Text>>,
    stxtcity: Rc<RefCell<Text>>,
    stxtaddress: Rc<RefCell<Text>>,
    stxtval: Rc<RefCell<Text>>,
}

// Layout styles: TopPanel, SidePanel, NoPanel
//const LAYOUT_STYLE: gui_lib::LayoutStyle = TopPanel;
const LAYOUT_STYLE: gui_lib::LayoutStyle = SidePanel;
// Background colors: BKG_DEFAULT, BKG_WINDOWS, any Color32
const BACKGROUND_COLOR: Color32 = gui_lib::BKG_DEFAULT;

/// ## struct Canvas
/// A container for rendering and managing graphical shapes
/// and interactive widgets.
///
/// Owns the app1's BasicCanvas and selected concrete view handles.
/// Builds the visual scene and updates selected elements from TheWorld.
#[derive(Debug)]
pub(crate) struct TheCanvas {
    // BasicCanvas provides underlying canvas structure and functionality.
    // Shapes are stored in BasicCanvas::shapes: Vec<ShapeHandle>
    // (pub type ShapeHandle = Rc<RefCell<dyn Shape>> to allow dynamic update.)
    // Widgets are stored in  BasicCanvas::Vec<Box<dyn Widget>>
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
        let mut canvas = BasicCanvas::new(LAYOUT_STYLE, BACKGROUND_COLOR);
        Self::init_widgets(&mut canvas);
        let view_handles = Self::init_shapes(&mut canvas);

        Self { canvas, view_handles }
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

        // Add a Rectangle to the canvas
        // rect is a Rc<RefCell<T>> pointing to a concrete struct (in this case, a Rectangle)
        let rect: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new_from_center(
            eframe::egui::Pos2::new(400.0, 200.0),
            eframe::egui::Vec2::new(150.0, 100.0),
        )));
        rect.borrow_mut().set_fill_color(Color32::LIGHT_GRAY); // using RefCell interior mutability
        // cloning increases the ref count of the Rc
        // coercion to ShapeHandle happens automatically
        // pushed into BasicCanvas::shapes: Vec<ShapeHandle>
        canvas.add_shape(rect.clone()); // coercion to ShapeHandle happens automatically

        // Add a series of Polylines to the canvas
        let mut y = 75.0;
        for _ in 0..22 {
            //note: vee will be lost. It will not be used to initialize a field in Self
            let vee: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
                eframe::egui::Pos2::new(150.0, y),
                [
                    eframe::egui::Pos2::new(0.0, 0.0),
                    eframe::egui::Pos2::new(10.0, 10.0),
                    eframe::egui::Pos2::new(20.0, 0.0),
                ],
            )));
            // Push each polyline sequentially into BasicCanvas::shapes: Vec<ShapeHandle>
            canvas.add_shape(vee.clone()); // coercion to ShapeHandle happens automatically
            y += 10.0;
        }

        // Add the bottom traffic light circle to the canvas
        // tl_circle1 is not used to initialize a field in Self
        // and it will go out of scope and be dropped.
        let tl_circle1: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
            eframe::egui::Pos2::new(200.0, 200.0),
            75.0,
        )));
        tl_circle1.borrow_mut().set_line_width(4.0);
        tl_circle1.borrow_mut().set_fill_color(Color32::GRAY);
        canvas.add_shape(tl_circle1.clone()); // coercion to ShapeHandle happens automatically

        // Add the top traffic light circle to the canvas
        let tl_circle2: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
            eframe::egui::Pos2::new(200.0, 200.0),
            10.0,
        )));
        tl_circle2.borrow_mut().set_fill_color(Color32::RED);
        // Will be drawn on top of tl_circle1 because of z-order
        canvas.add_shape(tl_circle2.clone()); // coercion to ShapeHandlehappens automatically

        // Add text to frame number.
        let stxt_frame: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            eframe::egui::Pos2::new(250.0, 270.0),
            format!("{}", 0),
        )));
        stxt_frame.borrow_mut().set_size(36.0);
        stxt_frame.borrow_mut().set_color(Color32::BLUE);
        canvas.add_shape(stxt_frame.clone()); // coercion to ShapeHandle happens automatically

        // Add a dotted polyline to the canvas
        //let poly: ShapeHandle = Rc::new(RefCell::new(Polyline::new(
        //let poly: Rc<RefCell<dyn Shape>> = Rc::new(RefCell::new(Polyline::new(
        let poly: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
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
        poly.borrow_mut().set_color(Color32::RED);
        poly.borrow_mut().set_line_width(4.0);
        poly.borrow_mut().set_line_style(Dotted);
        canvas.add_shape(poly); // coercion to ShapeHandle happens automatically

        // Add gauge rectangle
        let gauge: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new_from_center(
            eframe::egui::Pos2::new(500.0, 350.0),
            eframe::egui::Vec2::new(850.0, 50.0),
        )));
        gauge.borrow_mut().set_fill_color(Color32::LIGHT_GRAY);
        canvas.add_shape(gauge); // coercion to ShapeHandle happens automatically

        let arrow_head: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
            eframe::egui::Pos2::new(100.0, 369.0),
            [
                eframe::egui::Pos2::new(-4.0, 0.0),
                eframe::egui::Pos2::new(0.0, -39.0),
                eframe::egui::Pos2::new(4.0, 0.0),
            ],
        )));
        arrow_head.borrow_mut().set_line_width(2.0);
        canvas.add_shape(arrow_head.clone()); // coercion to ShapeHandle happens automatically

        // Add text to describe the state of the thing
        let stxt = Rc::new(RefCell::new(Text::new(egui::Pos2::new(345.0, 175.0), "")));
        canvas.add_shape(stxt.clone()); // coercion to ShapeHandle happens automatically

        // Add text to display name.
        let stxtname: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(325.0, 33.0),
            "Name: Steve",
        )));
        canvas.add_shape(stxtname.clone()); // coercion to ShapeHandle happens automatically

        let stxtcity: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(325.0, 65.0),
            "City: Birtle",
        )));
        canvas.add_shape(stxtcity.clone()); // coercion to ShapeHandle happens automatically

        let stxtaddress: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(325.0, 97.0),
            "Address: 123 Main St",
        )));
        canvas.add_shape(stxtaddress.clone()); // coercion to ShapeHandle happens automatically

        // Add text to display value.
        let stxtval: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            eframe::egui::Pos2::new(650.0, 33.0),
            format!("{}{:.2}", "Value: ", 0.0),
        )));
        canvas.add_shape(stxtval.clone()); // coercion to ShapeHandle happens automatically

        ViewHandles {
            // Shapes as unique handles to a concrete struct (e.g. Rc<RefCell<Circle>>)
            tl_circle2,
            stxt_frame,
            rect,
            arrow_head,
            stxt,
            stxtname,
            stxtcity,
            stxtaddress,
            stxtval,
        }
    }

    // Create and add widgets as Box<dyn Widget>
    fn init_widgets(canvas: &mut BasicCanvas) {
        // ---- Create and add widgets as Box<dyn Widget>
        canvas.add_widget(Box::new(Space::new(15.0)));

        let label1 = Label::new("Application", Color32::BLUE, 26.0);
        canvas.add_widget(Box::new(label1));

        canvas.add_widget(Box::new(Space::new(15.0)));

        let wb_run = Button::new(BTN_RUN_PAUSE, "Run/Pause", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_run));

        let wb_speed = Button::new(BTN_SLOW_FAST, "Slow/Fast", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_speed));

        let wb_a = Button::new(BTN_STATE_A, "State A", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_a));

        let wb_b = Button::new(BTN_STATE_B, "State B", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_b));

        canvas.add_widget(Box::new(Separator::new()));

        let mut wdf1 = DragFloat::new(DRAGFLOAT_GAUGE, "Gauge = ", 0.0, 0.0..=100.0);
        wdf1.set_decimal(1);
        //wdf1.set_speed(0.1);
        canvas.add_widget(Box::new(wdf1));

        let sep = Separator::new(); // sep consumed, so can be reused
        canvas.add_widget(Box::new(sep));

        // let wb_enter_name = Button::new(BTN_ENTER_NAME, "Enter Name", 120.0, 40.0);
        // canvas.add_widget(Box::new(wb_enter_name));

        let wb_person = Button::new(BTN_PERSON, "Enter Person", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_person));

        let wb_enter_value = Button::new(BTN_ENTER_VALUE, "Enter Value", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_enter_value));

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
        // Set stxt_frame to display frame number
        self.view_handles
            .stxt_frame
            .borrow_mut()
            .set_text(format!("{}", world.frame_number));
        // Get state of traffic light and set appropriate color
        let tlc = if world.tl.state == Signal::Stop {
            Color32::RED
        } else {
            Color32::GREEN
        };
        self.view_handles.tl_circle2.borrow_mut().set_fill_color(tlc);

        // Update gauge pointer
        let mut arrow_head = self.view_handles.arrow_head.borrow_mut();
        let mut ah_pos = arrow_head.location();
        let pointer = world.gauge.pointer() as f32;
        ah_pos.x = 100.0 + 8.0 * pointer;
        arrow_head.move_to(ah_pos);

        // Update thing state, color coded
        match world.thing.state {
            ThingState::StateA => {
                self.view_handles.rect.borrow_mut().set_fill_color(Color32::GOLD);
                self.view_handles.stxt.borrow_mut().set_text("State A");
            }
            ThingState::StateB => {
                self.view_handles.rect.borrow_mut().set_fill_color(Color32::CYAN);
                self.view_handles.stxt.borrow_mut().set_text("State B");
            }
            _ => {}
        }

        //Update name
        // let name = format!("Name: {}", world.name);
        // self.stxtname.borrow_mut().set_text(name);

        // Update person
        let person_name = format!("Name: {}", world.person.name);
        self.view_handles.stxtname.borrow_mut().set_text(person_name);

        let person_city = format!("City: {}", world.person.city);
        self.view_handles.stxtcity.borrow_mut().set_text(person_city);

        let person_address = format!("Address: {}", world.person.address);
        self.view_handles.stxtaddress.borrow_mut().set_text(person_address);

        //Update val_string
        let val = world.value;
        let val_string = format!("Value: {:.2}", val);
        self.view_handles.stxtval.borrow_mut().set_text(val_string);
    }
} // end of impl TheCanvas
