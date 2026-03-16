//! ## module Canvas
//! Declation for struct TheCanvas:
//! A container for rendering and managing graphical shapes
//! and interactive widgets.

// canvas.rs

use std::cell::RefCell;
use std::rc::Rc;

use gui_lib::LayoutStyle::TopPanel;
use gui_lib::{
    BKG_EXAMPLE, BasicCanvas, Button, Circle, Color32, DragFloat, Label, Line, Polyline, Rectangle,
    Separator, Shape, ShapeHandle, Space, Text,
};
use gui_lib::{LineStyle::Dashed, LineStyle::Dotted, LineStyle::Solid};
use gui_lib::{Pos2, Vec2};

use crate::ids::{
    BTN_ABOUT, BTN_ENTER_NAME, BTN_ENTER_VALUE, BTN_RUN_PAUSE, BTN_STATE_A, BTN_STATE_B,
    DRAGFLOAT_GAUGE,
};
use crate::world::{Signal, TheWorld, ThingState};

/// ## struct Canvas
/// A container for rendering and managing graphical shapes
/// and interactive widgets.
/// - Manages a collection of shapes using the `Shape` trait.
/// - Supports dynamic updates of shape properties.
/// - Manages a collection of widgets using the `Widget` trait.
/// - Integrates with the `gui_lib` library for rendering.
#[derive(Debug)]
pub(crate) struct TheCanvas {
    pub(crate) canvas: BasicCanvas,
    circle1: Rc<RefCell<Circle>>,
    circle2: Rc<RefCell<Circle>>,
    rect: Rc<RefCell<Rectangle>>,
    sp: Rc<RefCell<Polyline>>,
    arrow_head: Rc<RefCell<Polyline>>,
    stxt: Rc<RefCell<Text>>,
    stxtname: Rc<RefCell<Text>>,
    stxtval: Rc<RefCell<Text>>,
    pub(crate) line_test: Rc<RefCell<Line>>,
}

impl TheCanvas {
    //! Constructor for TheCanvas.
    //!
    //! This is where Shapes and Widgets are added to create the original graphical display
    pub(crate) fn new() -> Self {
        // New empty BasicCanvas
        let mut canvas = BasicCanvas::new(TopPanel, BKG_EXAMPLE);
        // Other possibilities:
        //let mut canvas = BasicCanvas::new(SidePanel, BKG_EXAMPLE);
        //let mut canvas = BasicCanvas::new(NoPanel, BKG_EXAMPLE);

        // Add shapes as ShapeHandle's to the canvas (in BasicCanvas::Vec<ShapeHandle>)
        let mut y = 75.0;
        for _ in 0..22 {
            //note: vee will be lost. It will not be a field in Self
            let vee: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
                Pos2::new(150.0, y),
                [
                    Pos2::new(0.0, 0.0),
                    Pos2::new(10.0, 10.0),
                    Pos2::new(20.0, 0.0),
                ],
            )));
            // Add shape as handle to the canvas (in Vec shapes)
            let vee_cln: ShapeHandle = vee.clone();
            canvas.add_shape(vee_cln as ShapeHandle);
            y += 10.0;
        }

        // Add shape with handle. Also declared as a field in TheCanvas
        let circle1: Rc<RefCell<Circle>> =
            Rc::new(RefCell::new(Circle::new(Pos2::new(200.0, 200.0), 75.0)));
        circle1.borrow_mut().set_line_width(4.0);
        circle1.borrow_mut().set_fill_color(Color32::GRAY);
        let circle1_cln: ShapeHandle = circle1.clone();
        canvas.add_shape(circle1_cln as ShapeHandle);

        let circle2: Rc<RefCell<Circle>> =
            Rc::new(RefCell::new(Circle::new(Pos2::new(200.0, 200.0), 10.0)));
        circle2.borrow_mut().set_fill_color(Color32::RED);
        let circle2_cln: ShapeHandle = circle2.clone();
        canvas.add_shape(circle2_cln as ShapeHandle);

        let rect: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new_from_center(
            Pos2::new(400.0, 200.0),
            Vec2::new(150.0, 100.0),
        )));
        rect.borrow_mut().set_fill_color(Color32::LIGHT_GRAY);
        let rect_cln: ShapeHandle = rect.clone();
        canvas.add_shape(rect_cln as ShapeHandle);

        let sp: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
            Pos2::new(550.0, 200.0),
            [
                Pos2::new(0.0, 0.0),
                Pos2::new(25.0, 50.0),
                Pos2::new(75.0, -50.0),
                Pos2::new(125.0, 50.0),
                Pos2::new(175.0, -50.0),
                Pos2::new(225.0, 50.0),
                Pos2::new(250.0, 0.0),
            ],
        )));
        sp.borrow_mut().set_color(Color32::RED);
        //sp.borrow_mut().set_line_width(2.0);
        sp.borrow_mut().set_line_width(4.0);
        //sp.borrow_mut().set_line_style(Dashed);
        sp.borrow_mut().set_line_style(Dotted);
        //sp.borrow_mut().set_line_style(Solid);
        let sp_cln: ShapeHandle = sp.clone();
        canvas.add_shape(sp_cln as ShapeHandle);

        let gauge: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new_from_center(
            Pos2::new(500.0, 350.0),
            Vec2::new(850.0, 50.0),
        )));
        gauge.borrow_mut().set_fill_color(Color32::LIGHT_GRAY);
        let gauge_cln: ShapeHandle = gauge.clone();
        canvas.add_shape(gauge_cln as ShapeHandle);

        let arrow_head: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
            Pos2::new(100.0, 369.0),
            [
                Pos2::new(-4.0, 0.0),
                Pos2::new(0.0, -39.0),
                Pos2::new(4.0, 0.0),
            ],
        )));
        arrow_head.borrow_mut().set_line_width(2.0);
        let arrow_head_cln: ShapeHandle = arrow_head.clone();
        canvas.add_shape(arrow_head_cln as ShapeHandle);

        let stxt: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(Pos2::new(345.0, 175.0), "")));
        stxt.borrow_mut().set_color(Color32::DARK_GREEN);
        let stxt_cln: ShapeHandle = stxt.clone();
        canvas.add_shape(stxt_cln as ShapeHandle);

        let stxtname: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            Pos2::new(325.0, 60.0),
            "Name: Steve",
        )));
        let stxtname_cln: ShapeHandle = stxtname.clone();
        canvas.add_shape(stxtname_cln as ShapeHandle);

        let stxtval: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            Pos2::new(325.0, 100.0),
            format!("{}{:.2}", "Value: ", 0.0),
        )));
        let stxtval_cln: ShapeHandle = stxtval.clone();
        canvas.add_shape(stxtval_cln as ShapeHandle);

        let line_test: Rc<RefCell<Line>> = Rc::new(RefCell::new(Line::new(
            Pos2::new(100.0, 600.0),
            Vec2::new(200.0, -100.0),
        )));
        line_test.borrow_mut().set_line_width(4.0);
        //line_test.borrow_mut().set_line_style(Dashed);
        line_test.borrow_mut().set_line_style(Dotted);
        line_test.borrow_mut().set_color(Color32::RED);
        let line_test_cln: ShapeHandle = line_test.clone();
        canvas.add_shape(line_test_cln as ShapeHandle);

        let line_test2: Rc<RefCell<Line>> = Rc::new(RefCell::new(Line::new(
            Pos2::new(300.0, 500.0),
            Vec2::new(200.0, -100.0),
        )));
        line_test2.borrow_mut().set_line_width(4.0);
        //line_test2.borrow_mut().set_line_style(Dashed);
        //line_test2.borrow_mut().set_line_style(Dotted);
        line_test2.borrow_mut().set_color(Color32::RED);
        let line_test2_cln: ShapeHandle = line_test2.clone();
        canvas.add_shape(line_test2_cln as ShapeHandle);

        let circle_test: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
            Pos2::new(700.0, 500.0),
            100.0,
        )));
        circle_test.borrow_mut().set_line_width(4.0);
        circle_test.borrow_mut().set_line_style(Dashed);
        //circle_test.borrow_mut().set_line_style(Dotted);
        circle_test.borrow_mut().set_color(Color32::RED);
        circle_test.borrow_mut().set_fill_color(Color32::LIGHT_BLUE);

        circle_test.borrow_mut().move_to(Pos2::new(900.0, 600.0));

        let circle_test_cln: ShapeHandle = circle_test.clone();
        canvas.add_shape(circle_test_cln as ShapeHandle);


        // ---- Create and add widgets as Box<dyn Widget>
        canvas.add_widget(Box::new(Space::new(15.0)));

        let label1 = Label::new("Sandbox", Color32::BLUE, 20.0);
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

        let wb_enter_name = Button::new(BTN_ENTER_NAME, "Enter Name", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_enter_name));

        let wb_enter_value = Button::new(BTN_ENTER_VALUE, "Enter Value", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_enter_value));

        let wb_about = Button::new(BTN_ABOUT, "About", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_about));

        //Create the TheCanvas
        Self {
            canvas,
            circle1,
            circle2,
            rect,
            sp,
            arrow_head,
            stxt,
            stxtname,
            stxtval,
            line_test,
        }
    }

    //TDJ: not used. Wat is fn for?
    pub(crate) fn canvas(&self) -> &BasicCanvas {
        &self.canvas
    }
    //TDJ: not used. Wat is fn for?
    pub fn canvas_mut(&mut self) -> &mut BasicCanvas {
        &mut self.canvas
    }

    /// Update the state of the canvas based on the current world state.
    /// This method is called by the `TheApp` to update the canvas with the latest world state.
    /// Note that this method does not modify the world state.
    /// The world does not know about the canvas (nor about egui). This is important to keep the
    /// separation of concerns. Program data and logic is encapsulated in the `TheWorld` struct.
    pub(crate) fn update(&mut self, world: &TheWorld) {
        // Get state of traffic light and set appropriate color
        let tlc = if world.tl.state == Signal::Stop {
            Color32::RED
        } else {
            Color32::GREEN
        };
        self.circle2.borrow_mut().set_fill_color(tlc);

        // Update gauge pointer
        let mut ah_pos = self.arrow_head.borrow_mut().location();
        ah_pos.x = 100.0 + 8.0 * (world.gauge.pointer() as f32);
        self.arrow_head.borrow_mut().move_to(ah_pos);

        // Update thing state, color coded
        match world.thing.state {
            ThingState::StateA => {
                self.rect.borrow_mut().set_fill_color(Color32::GOLD);
                self.stxt.borrow_mut().set_text("State A");
                self.stxt.borrow_mut().set_vertical();
                self.line_test.borrow_mut().set_length( 400.0);
                self.line_test.borrow_mut().set_angle( 0.0 );
                //self.rect.borrow_mut().set_width( 400.0 );
            },
            ThingState::StateB => {
                self.rect.borrow_mut().set_fill_color(Color32::CYAN);
                self.stxt.borrow_mut().set_text("State B");
                self.stxt.borrow_mut().set_horizontal();
                self.stxt.borrow_mut().set_size(48.0);
                self.line_test.borrow_mut().set_length(100.0);
                self.line_test.borrow_mut().set_angle( -1.5 );
                //self.rect.borrow_mut().move_to( Pos2::new(400.0, 500.0) );
            },
            _ => {},
        }

        //Update name
        let name: String = "Name: ".to_owned() + &world.name.clone();
        self.stxtname.borrow_mut().set_text(name);

        //Update val_string
        let val = world.value;
        let val_string: String = format!("{}{:.2}", "Value: ", val);
        self.stxtval.borrow_mut().set_text(val_string);
    }
} // end of impl TheCanvas
