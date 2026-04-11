//! ## module Canvas. Contains TheCanvas struct.
//! Declaration for struct [`TheCanvas`]:
//! A container for rendering and managing graphical shapes
//! and interactive widgets.

// canvas_gl

use std::cell::RefCell;
use std::rc::Rc;

use crate::ids::*;
use crate::world::{Signal, TheWorld, ThingState};
use gui_lib::LayoutStyle::{NoPanel, SidePanel, TopPanel};
use gui_lib::LineStyle::{Dashed, Dotted, Solid};
use gui_lib::{
    BKG_EXAMPLE, BasicCanvas, Button, Circle, Color32, DragFloat, Label, Polyline, Rectangle,
    Separator, Shape, Space, Text,
};

/// ## struct Canvas
/// A container for rendering and managing graphical shapes
/// and interactive widgets.
/// - Manages a collection of shapes using the `Shape` trait.
/// - Supports dynamic drawing of shapes.
/// - Supports updates of shape properties.
/// - Manages a collection of widgets using the `Widget` trait.
/// - Integrates with the `gui_lib` library.
#[derive(Debug)]
pub(crate) struct TheCanvas {
    // BasicCanvas provides underlying structure and functionality for any user canvas.
    // Shapes are stored in BasicCanvas::shapes: Vec<ShapeHandle>
    // (pub type ShapeHandle = Rc<RefCell<dyn Shape>> to allow dynamic update.)
    pub(crate) canvas: BasicCanvas, // From gui_lib

    // Concrete shapes   (e.g. Circle)
    // are stored in TheCanvas as fields of type Rc<RefCell<T>>
    tl_circle2: Rc<RefCell<Circle>>,
    //tl_circle2: SharedMut<Circle>,
    rect: Rc<RefCell<Rectangle>>,
    //rect: SharedMut<Rectangle>,
    arrow_head: Rc<RefCell<Polyline>>,
    stxt: Rc<RefCell<Text>>,
    stxtname: Rc<RefCell<Text>>,
    stxtcity: Rc<RefCell<Text>>,
    stxtaddress: Rc<RefCell<Text>>,
    stxtval: Rc<RefCell<Text>>,
}

impl TheCanvas {
    /// Constructor for TheCanvas.
    ///
    /// This is where Shapes and Widgets are added to create the initial graphical display.
    ///
    /// Set layout and background color in [`BasicCanvas`] constructor.
    pub(crate) fn new() -> Self {
        // New empty BasicCanvas
        // --- Other possibilities: //TDJ: maybe use costant for layout style
        // and background color?
        //let mut canvas = BasicCanvas::new(SidePanel, BKG_EXAMPLE);
        //let mut canvas = BasicCanvas::new(NoPanel, BKG_EXAMPLE);
        let mut canvas = BasicCanvas::new(TopPanel, BKG_EXAMPLE);

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
        canvas.add_shape(rect.clone());

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
            canvas.add_shape(vee.clone()); // coercion happens automatically
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
        canvas.add_shape(tl_circle1.clone()); // coercion happens automatically

        // Add the top traffic light circle to the canvas
        let tl_circle2: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
            eframe::egui::Pos2::new(200.0, 200.0),
            10.0,
        )));
        tl_circle2.borrow_mut().set_fill_color(Color32::RED);
        // Will be drawn on top of tl_circle1 because of z-order
        canvas.add_shape(tl_circle2.clone()); // coercion happens automatically

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
        poly.borrow_mut().set_line_width(2.0);
        poly.borrow_mut().set_line_width(4.0);
        poly.borrow_mut().set_line_style(Dotted);
        canvas.add_shape(poly); // coercion happens automatically

        // Add gauge rectangle
        let gauge: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new_from_center(
            eframe::egui::Pos2::new(500.0, 350.0),
            eframe::egui::Vec2::new(850.0, 50.0),
        )));
        gauge.borrow_mut().set_fill_color(Color32::LIGHT_GRAY);
        canvas.add_shape(gauge); // coercion happens automatically

        let arrow_head: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
            eframe::egui::Pos2::new(100.0, 369.0),
            [
                eframe::egui::Pos2::new(-4.0, 0.0),
                eframe::egui::Pos2::new(0.0, -39.0),
                eframe::egui::Pos2::new(4.0, 0.0),
            ],
        )));
        arrow_head.borrow_mut().set_line_width(2.0);
        canvas.add_shape(arrow_head.clone()); // coercion happens automatically

        // Add text to describe the state of the thing
        let stxt = Rc::new(RefCell::new(Text::new(egui::Pos2::new(345.0, 175.0), "")));
        canvas.add_shape(stxt.clone()); // coercion happens automatically

        // Add text to display name.
        let stxtname: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(325.0, 33.0),
            "Name: Steve",
        )));
        canvas.add_shape(stxtname.clone()); // coercion happens automatically

        let stxtcity: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(325.0, 65.0),
            "City: Birtle",
        )));
        canvas.add_shape(stxtcity.clone()); // coercion happens automatically

        let stxtaddress: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(325.0, 97.0),
            "Address: 123 Main St",
        )));
        canvas.add_shape(stxtaddress.clone()); // coercion happens automatically

        // Add text to display value.
        let stxtval: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            //eframe::egui::Pos2::new(325.0, 100.0),
            eframe::egui::Pos2::new(650.0, 33.0),
            format!("{}{:.2}", "Value: ", 0.0),
        )));
        canvas.add_shape(stxtval.clone()); // coercion happens automatically

        // ---- Create and add widgets as Box<dyn Widget>
        canvas.add_widget(Box::new(Space::new(15.0)));

        let label1 = Label::new("The App", Color32::RED, 20.0);
        canvas.add_widget(Box::new(label1));

        canvas.add_widget(Box::new(Space::new(15.0)));

        let wb_run = Button::new(BTN_RUN_PAUSE, "Run/Pause", 120.0, 40.0);
        //let wb_run = Button::new(BTN_RUN_PAUSE,None, 0.0, 0.0);
        canvas.add_widget(Box::new(wb_run));

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

        //Create the TheCanvas
        Self {
            // BasicCanvas
            canvas,
            // Shapes as unique handles to a concrete struct (e.g. Rc<RefCell<Circle>>)
            tl_circle2,
            rect,
            arrow_head,
            stxt,
            stxtname,
            stxtcity,
            stxtaddress,
            stxtval,
        }
    }

    //TDJ: not used. Should it be?
    // pub(crate) fn canvas(&self) -> &BasicCanvas {
    //     &self.canvas
    // }
    //TDJ: not used.  Should it be?
    // pub(crate) fn canvas_mut(&mut self) -> &mut BasicCanvas {
    //     &mut self.canvas
    // }

    /// Update the state of the canvas based on the current world state.
    /// This method is called by the [`TheApp`] to update the canvas with the latest world state.
    /// Note that this method does not modify the world state.
    /// The world does not know about the canvas (nor about egui). This is important to keep the
    /// separation of concerns. Program data and logic is encapsulated in the [`TheWorld`] struct.
    /// Smaller programs may nat have a world object, in which case this update function will
    /// take a parameter [`TheApp`] instead of TheWorld.
    pub(crate) fn update(&mut self, world: &TheWorld) {
        // Get state of traffic light and set appropriate color
        let tlc = if world.tl.state == Signal::Stop {
            Color32::RED
        } else {
            Color32::GREEN
        };
        self.tl_circle2.borrow_mut().set_fill_color(tlc);

        // Update gauge pointer
        let mut ah_pos = self.arrow_head.borrow_mut().location();
        ah_pos.x = 100.0 + 8.0 * (world.gauge.pointer() as f32);
        self.arrow_head.borrow_mut().move_to(ah_pos);

        // Update thing state, color coded
        match world.thing.state {
            ThingState::StateA => {
                self.rect.borrow_mut().set_fill_color(Color32::GOLD);
                self.stxt.borrow_mut().set_text("State A");
            }
            ThingState::StateB => {
                self.rect.borrow_mut().set_fill_color(Color32::CYAN);
                self.stxt.borrow_mut().set_text("State B");
            }
            _ => {}
        }

        //Update name
        let name: String = "Name: ".to_owned() + &world.name.clone();
        self.stxtname.borrow_mut().set_text(name);

        //let person_name: String = "Name: ".to_owned() + &world.person.name.clone();
        let person_name: String = "Name: ".to_owned() + &world.person.name;
        self.stxtname.borrow_mut().set_text(person_name);

        let person_city: String = "City: ".to_owned() + &world.person.city;
        self.stxtcity.borrow_mut().set_text(person_city);

        let person_address: String = "Address: ".to_owned() + &world.person.address;
        self.stxtaddress.borrow_mut().set_text(person_address);

        //Update val_string
        //let val = 42.3;
        let val = world.value;
        //let val_string: String = format!("{}{}", "Value: ", val);
        let val_string: String = format!("{}{:.2}", "Value: ", val);
        self.stxtval.borrow_mut().set_text(val_string);
    }
} // end of impl TheCanvas
