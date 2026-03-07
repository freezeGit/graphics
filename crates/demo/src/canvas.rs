//! ## Canvas
//! A container for rendering and managing graphical shapes
//! and interactive widgets.
//! - Manages a collection of shapes using the `Shape` trait.
//! - Supports dynamic updates of shape properties.
//! - Manages a collection of widgets using the `Widget` trait.
//! - Integrates with the `gui_lib` library for rendering.

// canvas.rs

use std::cell::RefCell;
use std::rc::Rc;

use gui_lib::LayoutStyle::TopPanel;
use gui_lib::LineStyle::Dotted;
use gui_lib::{
    BKG_EXAMPLE, BasicCanvas, Button, Circle, Color32, DragFloat, Label, Polyline, Rectangle,
    Separator, Shape, ShapeHandle, Space, Text,
};

use crate::ids::{
    BTN_ABOUT, BTN_ENTER_NAME, BTN_ENTER_VALUE, BTN_RUN_PAUSE, BTN_STATE_A, BTN_STATE_B,
    DRAGFLOAT_GAUGE,
};
use crate::world::{Signal, TheWorld, ThingState};

#[derive(Debug)]
pub(crate) struct TheCanvas {
    pub(crate) canvas: BasicCanvas,
    sc1: Rc<RefCell<Circle>>,
    sc2: Rc<RefCell<Circle>>,
    sr: Rc<RefCell<Rectangle>>,
    sp: Rc<RefCell<Polyline>>,
    arrow_head: Rc<RefCell<Polyline>>,
    stxt: Rc<RefCell<Text>>,
    stxtname: Rc<RefCell<Text>>,
    stxtval: Rc<RefCell<Text>>,
}

impl TheCanvas {
    pub(crate) fn new() -> Self {
        // New empty BasicCanvas
        let mut canvas = BasicCanvas::new(TopPanel, BKG_EXAMPLE);
        // Other possibilities:
        //let mut canvas = BasicCanvas::new(SidePanel, BKG_EXAMPLE);
        //let mut canvas = BasicCanvas::new(NoPanel, BKG_EXAMPLE);

        // Add shapes without handles to the canvas
        let mut y = 75.0;
        for _ in 0..22 {
            //note: vee will be lost. It will not be a field in Self
            let vee: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
                eframe::egui::Pos2::new(150.0, y),
                [
                    eframe::egui::Pos2::new(0.0, 0.0),
                    eframe::egui::Pos2::new(10.0, 10.0),
                    eframe::egui::Pos2::new(20.0, 0.0),
                ],
            )));
            let vee_cln: ShapeHandle = vee.clone();
            canvas.add_shape(vee_cln as ShapeHandle);
            y += 10.0;
        }

        // Add shape with handle
        let sc1: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
            eframe::egui::Pos2::new(200.0, 200.0),
            75.0,
        )));
        sc1.borrow_mut().set_line_width(4.0);
        sc1.borrow_mut().set_fill_color(Color32::GRAY);
        let sc1_cln: ShapeHandle = sc1.clone();
        canvas.add_shape(sc1_cln as ShapeHandle);

        // Add shape with handle
        let sc2: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
            eframe::egui::Pos2::new(200.0, 200.0),
            10.0,
        )));
        sc2.borrow_mut().set_fill_color(Color32::RED);
        let sc2_cln: ShapeHandle = sc2.clone();
        canvas.add_shape(sc2_cln as ShapeHandle);

        // Add shape with handle
        let sr: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new_from_center(
            //let sr: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new(
            eframe::egui::Pos2::new(400.0, 200.0),
            eframe::egui::Vec2::new(150.0, 100.0),
        )));
        sr.borrow_mut().set_fill_color(Color32::LIGHT_GRAY);
        let sr_cln: ShapeHandle = sr.clone();
        canvas.add_shape(sr_cln as ShapeHandle);

        // Add shape with handle
        let sp: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
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
        sp.borrow_mut().set_color(Color32::RED);
        sp.borrow_mut().set_line_width(2.0);
        sp.borrow_mut().set_line_width(4.0);
        //sp.borrow_mut().set_line_style(Dashed);
        sp.borrow_mut().set_line_style(Dotted);
        //sp.borrow_mut().set_line_style(Solid);
        let sp_cln: ShapeHandle = sp.clone();
        canvas.add_shape(sp_cln as ShapeHandle);

        // Add shape with handle
        // TDJ: change to left upper corner when possible
        //let gauge: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new(
        let gauge: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new_from_center(
            eframe::egui::Pos2::new(500.0, 350.0),
            eframe::egui::Vec2::new(850.0, 50.0),
        )));
        gauge.borrow_mut().set_fill_color(Color32::LIGHT_GRAY);
        let gauge_cln: ShapeHandle = gauge.clone();
        canvas.add_shape(gauge_cln as ShapeHandle);

        // Add shape with handle
        let arrow_head: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
            eframe::egui::Pos2::new(100.0, 369.0),
            [
                eframe::egui::Pos2::new(-4.0, 0.0),
                eframe::egui::Pos2::new(0.0, -39.0),
                eframe::egui::Pos2::new(4.0, 0.0),
            ],
        )));
        arrow_head.borrow_mut().set_line_width(2.0);
        let arrow_head_cln: ShapeHandle = arrow_head.clone();
        canvas.add_shape(arrow_head_cln as ShapeHandle);

        // Add shape with handle
        let stxt: Rc<RefCell<Text>> =
            Rc::new(RefCell::new(Text::new(egui::Pos2::new(345.0, 175.0), "")));
        stxt.borrow_mut().set_color(Color32::DARK_GREEN);
        let stxt_cln: ShapeHandle = stxt.clone();
        canvas.add_shape(stxt_cln as ShapeHandle);

        // Add shape with handle
        let stxtname: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            egui::Pos2::new(325.0, 60.0),
            //egui::Pos2::new(300.0, 75.0),
            "Name: Steve",
        )));
        let stxtname_cln: ShapeHandle = stxtname.clone();
        canvas.add_shape(stxtname_cln as ShapeHandle);

        // Add shape with handle
        let stxtval: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
            eframe::egui::Pos2::new(325.0, 100.0),
            //format!("{}{}", "Value: ", 0.0),
            format!("{}{}", "Value: ", 0.0),
        )));
        //stxtval.borrow_mut().set_color(Color32::DARK_GREEN);
        let stxtval_cln: ShapeHandle = stxtval.clone();
        canvas.add_shape(stxtval_cln as ShapeHandle);

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

        let wb_enter_name = Button::new(BTN_ENTER_NAME, "Enter Name", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_enter_name));

        let wb_enter_value = Button::new(BTN_ENTER_VALUE, "Enter Value", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_enter_value));

        let wb_about = Button::new(BTN_ABOUT, "About", 120.0, 40.0);
        canvas.add_widget(Box::new(wb_about));

        //Create the TheCanvas
        Self {
            canvas,
            sc1,
            sc2,
            sr,
            sp,
            arrow_head,
            stxt,
            stxtname,
            stxtval,
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

    pub(crate) fn update(&mut self, world: &TheWorld) {
        // Get state of traffic light and set appropriate color
        let tlc = if world.tl.state == Signal::Stop {
            Color32::RED
        } else {
            Color32::GREEN
        };
        self.sc2.borrow_mut().set_fill_color(tlc);

        // Update gauge pointer
        let mut ah_pos = self.arrow_head.borrow_mut().location();
        ah_pos.x = 100.0 + 8.0 * (world.gauge.pointer() as f32);
        self.arrow_head.borrow_mut().move_to(ah_pos);

        // Update thing state, color coded
        match world.thing.state {
            ThingState::StateA => {
                self.sr.borrow_mut().set_fill_color(Color32::GOLD);
                self.stxt.borrow_mut().set_text("State A");
            }
            ThingState::StateB => {
                self.sr.borrow_mut().set_fill_color(Color32::CYAN);
                self.stxt.borrow_mut().set_text("State B");
            }
            _ => {}
        }

        //Update name
        let name: String = "Name: ".to_owned() + &world.name.clone();
        self.stxtname.borrow_mut().set_text(name);

        //Update val_string
        //let val = 42.3;
        let val = world.value;
        //let val_string: String = format!("{}{}", "Value: ", val);
        let val_string: String = format!("{}{:.2}", "Value: ", val);
        self.stxtval.borrow_mut().set_text(val_string);
    }
} // end of impl TheCanvas
