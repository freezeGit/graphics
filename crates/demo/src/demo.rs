/// Demonstration module for an application with a custom UI.
///
/// This module showcases the implementation of a demo application using the `eframe`
/// framework and a custom `gui_lib` library to render various graphical components.
///
/// # Modules
/// - The `demo` module defines an application structure (`TheApp`) and its behavior.
/// - Uses utilities and components from the `gui_lib` module.
///
/// # Components
///
/// ## TheApp
/// The main structure and entry point of the application.
/// - Contains a `Canvas` for holding a collection of shapes.
/// - Provides methods for creating and updating the UI.
///
/// ## Canvas
/// A container for rendering and managing graphical shapes.
///
/// ## Shapes
/// Custom shapes implemented via the `gui_lib::Shape` trait:
/// - `Circle`: A circular shape with customizable size, fill color, and outline.
/// - `Rectangle`: A rectangular shape with customizable size, position, and fill color.
/// - `Polyline`: A series of connected line segments with customizable line width and color.
/// - `Text`: A text label with customizable position, color, and font size.
///  - Other Shapes can be added as needed.
///
/// # Animation
/// - Demonstrates basic animations and state toggles using time-based checks.
/// - Shapes on the canvas have their properties dynamically updated, e.g., blinking colors.
///
/// # Usage
///
/// ## Running the Application
/// Call the `run_the_app()` function to start the application.
/// It initializes an `eframe` native window and sets up the demo layout and visuals.
///
/// demo::run_the_app()
///
/// ## Modifying Shapes
/// The application supports dynamic modification of shape properties, such as:
/// - Color, size, and position.
/// These can be altered within the `update` method using the shape trait's API.
///
/// ## Extending Functionality
/// - Additional shapes and widgets can be added to the `Canvas`.
/// - Use the `Shape` trait to define custom graphical components.
///
/// # Example
/// use super::demo::run_the_app;
///
/// fn main() -> Result<(), eframe::Error> {
///    run_the_app()
/// }
///
/// # Notes
/// - `ctx.request_repaint_after()` ensures smooth interface by updating the frame at a fixed interval.
/// - Animations may be run with a second (slower) Timer loop.
///
/// # Modules Used:
/// - Uses core functionality from:
///   - `eframe::egui`
///   - `crate::gui_lib`
/// - Demonstrates integration with external modules like `gui_lib` for rendering and shapes.
///
/// # Errors
/// This application returns an `eframe::Error` if initialization or event handling fails.
///

// Demonstration module. App-specific code
// ------------------------------
/// Module containing the demo application implementation.
///
/// This module defines the demo application structure and its behavior,
/// using the components defined in the `gui_lib` module.

use crate::ids::*;
use crate::world::TheWorld;
use crate::world::Signal;
use crate::world::ThingState;
use crate::canvas::TheCanvas;


use ::gui_lib as gl;

// TDJ Cleanup imports
use gl::{BKG_EXAMPLE, BKG_WINDOWS};
use gl::LayoutStyle::{NoPanel, SidePanel, TopPanel};

use gl::{
    BasicCanvas, Button, Circle, Color32, DragFloat, DragFloatDlg, Label, MessageBoxDlg, Polyline,
    Rectangle, Separator, Slider, Space, Text, TextEntryDlg, Timer,
};

use gl::{
    ButtonId, DragFloatDlgId, DragFloatId, MessageBoxDlgId, Shape, ShapeHandle, SliderId,
    TextEntryDlgId, WidgetMsg,
};
//use gui_lib::{Dialog, DialogId, TextFont};
use gl::{Dialog, DialogId};
//use gl::{LineStyle::*, World};
use gl::{LineStyle::*};
use gl::egui::{Context, RichText};
use std::cell::RefCell;
use std::rc::Rc;

// const SLIDER_GAUGE: SliderId = SliderId(1);
// const SLIDER_ANOTHER: SliderId = SliderId(2); // Not used in this demo
//
// const DRAGFLOAT_GAUGE: DragFloatId = DragFloatId(1);
//
// const BTN_STATE_A: ButtonId = ButtonId(1);
// const BTN_STATE_B: ButtonId = ButtonId(2);
// const BTN_RUN_PAUSE: ButtonId = ButtonId(3);
// const BTN_ABOUT: ButtonId = ButtonId(4);
// const BTN_ENTER_NAME: ButtonId = ButtonId(5);
// const BTN_ENTER_VALUE: ButtonId = ButtonId(6);
//
// //const DLG_ENTER_NAME: DialogId = 1;
// const DLG_ABOUT: MessageBoxDlgId = MessageBoxDlgId(1);
// const DLG_ENTER_NAME: TextEntryDlgId = TextEntryDlgId(1);
// const DLG_ENTER_VALUE: DragFloatDlgId = DragFloatDlgId(1);

// #[derive(Debug)]
// struct Gauge {
//     pointer: f64,
// }
//
// impl Gauge {
//     fn new() -> Self {
//         Self { pointer: 0.0 }
//     }
//
//     fn pointer(&self) -> f64 {
//         self.pointer
//     }
//
//     fn set_pointer(&mut self, pointer: f64) {
//         self.pointer = pointer;
//     }
// }

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// enum Signal {
//     Stop,
//     Go,
// }
// #[derive(Debug)]
// struct TrafficLight {
//     state: Signal,
// }
//
// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// enum ThingState {
//     StateA,
//     StateB,
//     StateC,
// }
// #[derive(Debug)]
// struct Thing {
//     state: ThingState,
// }
//
// #[derive(Debug)]
// struct TheWorld {
//     //state: i32,
//     tl: TrafficLight,
//     thing: Thing,
//     gauge: Gauge,
//     name: String,
//     value: f64,
// }
//
// impl TheWorld {
//     fn new() -> Self {
//         Self {
//             //state: 0,
//             tl: TrafficLight {
//                 state: Signal::Stop,
//             },
//             thing: Thing {
//                 state: ThingState::StateC,
//             },
//             gauge: Gauge::new(),
//             name: "Steve".to_string(),
//             value: 0.0,
//         }
//     }
//
//     fn advance(&mut self) {
//         //self.state += 1;
//         self.toggle_light();
//     }
//
//     fn toggle_light(&mut self) {
//         self.tl.state = match self.tl.state {
//             Signal::Stop => Signal::Go,
//             Signal::Go => Signal::Stop,
//         };
//     }
// }
// #[derive(Debug)]
// pub struct TheCanvas {
//     canvas: BasicCanvas,
//     sc1: Rc<RefCell<Circle>>,
//     sc2: Rc<RefCell<Circle>>,
//     sr: Rc<RefCell<Rectangle>>,
//     sp: Rc<RefCell<Polyline>>,
//     arrow_head: Rc<RefCell<Polyline>>,
//     stxt: Rc<RefCell<Text>>,
//     stxtname: Rc<RefCell<Text>>,
//     stxtval: Rc<RefCell<Text>>,
// }
//
// impl TheCanvas {
//     pub fn new() -> Self {
//         // New empty BasicCanvas
//         let mut canvas = BasicCanvas::new(TopPanel, BKG_EXAMPLE);
//         //let mut canvas = BasicCanvas::new(SidePanel, BKG_EXAMPLE);
//         //let mut canvas = BasicCanvas::new(NoPanel, BKG_EXAMPLE);
//
//         // Add shapes without handles to the canvas
//         let mut y = 75.0;
//         for _ in 0..22 {
//             //note: vee will be lost. It will not be a field in Self
//             let vee: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
//                 eframe::egui::Pos2::new(150.0, y),
//                 [
//                     eframe::egui::Pos2::new(0.0, 0.0),
//                     eframe::egui::Pos2::new(10.0, 10.0),
//                     eframe::egui::Pos2::new(20.0, 0.0),
//                 ],
//             )));
//             let vee_cln: ShapeHandle = vee.clone();
//             canvas.add_shape(vee_cln as ShapeHandle);
//             y += 10.0;
//         }
//
//         // Add shape with handle
//         let sc1: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
//             eframe::egui::Pos2::new(200.0, 200.0),
//             75.0,
//         )));
//         sc1.borrow_mut().set_line_width(4.0);
//         sc1.borrow_mut().set_fill_color(Color32::GRAY);
//         let sc1_cln: ShapeHandle = sc1.clone();
//         canvas.add_shape(sc1_cln as ShapeHandle);
//
//         // Add shape with handle
//         let sc2: Rc<RefCell<Circle>> = Rc::new(RefCell::new(Circle::new(
//             eframe::egui::Pos2::new(200.0, 200.0),
//             10.0,
//         )));
//         sc2.borrow_mut().set_fill_color(Color32::RED);
//         let sc2_cln: ShapeHandle = sc2.clone();
//         canvas.add_shape(sc2_cln as ShapeHandle);
//
//         // Add shape with handle
//         let sr: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new_from_center(
//         //let sr: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new(
//             eframe::egui::Pos2::new(400.0, 200.0),
//             eframe::egui::Vec2::new(150.0, 100.0),
//         )));
//         sr.borrow_mut().set_fill_color(Color32::LIGHT_GRAY);
//         let sr_cln: ShapeHandle = sr.clone();
//         canvas.add_shape(sr_cln as ShapeHandle);
//
//         // Add shape with handle
//         let sp: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
//             eframe::egui::Pos2::new(550.0, 200.0),
//             [
//                 eframe::egui::Pos2::new(0.0, 0.0),
//                 eframe::egui::Pos2::new(25.0, 50.0),
//                 eframe::egui::Pos2::new(75.0, -50.0),
//                 eframe::egui::Pos2::new(125.0, 50.0),
//                 eframe::egui::Pos2::new(175.0, -50.0),
//                 eframe::egui::Pos2::new(225.0, 50.0),
//                 eframe::egui::Pos2::new(250.0, 0.0),
//             ],
//         )));
//         sp.borrow_mut().set_color(Color32::RED);
//         sp.borrow_mut().set_line_width(2.0);
//         sp.borrow_mut().set_line_width(4.0);
//         //sp.borrow_mut().set_line_style(Dashed);
//         sp.borrow_mut().set_line_style(Dotted);
//         //sp.borrow_mut().set_line_style(Solid);
//         let sp_cln: ShapeHandle = sp.clone();
//         canvas.add_shape(sp_cln as ShapeHandle);
//
//         // Add shape with handle
//         // TDJ: change to left upper corner when possible
//         //let gauge: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new(
//         let gauge: Rc<RefCell<Rectangle>> = Rc::new(RefCell::new(Rectangle::new_from_center(
//             eframe::egui::Pos2::new(500.0, 350.0),
//             eframe::egui::Vec2::new(850.0, 50.0),
//         )));
//         gauge.borrow_mut().set_fill_color(Color32::LIGHT_GRAY);
//         let gauge_cln: ShapeHandle = gauge.clone();
//         canvas.add_shape(gauge_cln as ShapeHandle);
//
//         // Add shape with handle
//         let arrow_head: Rc<RefCell<Polyline>> = Rc::new(RefCell::new(Polyline::new(
//             eframe::egui::Pos2::new(100.0, 369.0),
//             [
//                 eframe::egui::Pos2::new(-4.0, 0.0),
//                 eframe::egui::Pos2::new(0.0, -39.0),
//                 eframe::egui::Pos2::new(4.0, 0.0),
//             ],
//         )));
//         arrow_head.borrow_mut().set_line_width(2.0);
//         let arrow_head_cln: ShapeHandle = arrow_head.clone();
//         canvas.add_shape(arrow_head_cln as ShapeHandle);
//
//         // Add shape with handle
//         let stxt: Rc<RefCell<Text>> =
//             Rc::new(RefCell::new(Text::new(egui::Pos2::new(345.0, 175.0), "")));
//         stxt.borrow_mut().set_color(Color32::DARK_GREEN);
//         let stxt_cln: ShapeHandle = stxt.clone();
//         canvas.add_shape(stxt_cln as ShapeHandle);
//
//         // Add shape with handle
//         let stxtname: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
//             egui::Pos2::new(325.0, 60.0),
//             //egui::Pos2::new(300.0, 75.0),
//             "Name: Steve",
//         )));
//         let stxtname_cln: ShapeHandle = stxtname.clone();
//         canvas.add_shape(stxtname_cln as ShapeHandle);
//
//         // Add shape with handle
//         let stxtval: Rc<RefCell<Text>> = Rc::new(RefCell::new(Text::new(
//             eframe::egui::Pos2::new(325.0, 100.0),
//             //format!("{}{}", "Value: ", 0.0),
//             format!("{}{}", "Value: ", 0.0),
//         )));
//         //stxtval.borrow_mut().set_color(Color32::DARK_GREEN);
//         let stxtval_cln: ShapeHandle = stxtval.clone();
//         canvas.add_shape(stxtval_cln as ShapeHandle);
//
//         // ---- Create and add widgets as Box<dyn Widget>
//         canvas.add_widget(Box::new(Space::new(15.0)));
//
//         let label1 = Label::new("The App", Color32::RED, 20.0);
//         canvas.add_widget(Box::new(label1));
//
//         canvas.add_widget(Box::new(Space::new(15.0)));
//
//         let wb_run = Button::new(BTN_RUN_PAUSE, "Run/Pause", 120.0, 40.0);
//         //let wb_run = Button::new(BTN_RUN_PAUSE,None, 0.0, 0.0);
//         canvas.add_widget(Box::new(wb_run));
//
//         let wb_a = Button::new(BTN_STATE_A, "State A", 120.0, 40.0);
//         canvas.add_widget(Box::new(wb_a));
//
//         let wb_b = Button::new(BTN_STATE_B, "State B", 120.0, 40.0);
//         canvas.add_widget(Box::new(wb_b));
//
//         canvas.add_widget(Box::new(Separator::new()));
//
//         let mut wdf1 = DragFloat::new(DRAGFLOAT_GAUGE, "Gauge = ", 0.0, 0.0..=100.0);
//         wdf1.set_decimal(1);
//         //wdf1.set_speed(0.1);
//         canvas.add_widget(Box::new(wdf1));
//
//         let sep = Separator::new(); // sep consumed, so can be reused
//         canvas.add_widget(Box::new(sep));
//
//         let wb_enter_name = Button::new(BTN_ENTER_NAME, "Enter Name", 120.0, 40.0);
//         canvas.add_widget(Box::new(wb_enter_name));
//
//         let wb_enter_value = Button::new(BTN_ENTER_VALUE, "Enter Value", 120.0, 40.0);
//         canvas.add_widget(Box::new(wb_enter_value));
//
//         let wb_about = Button::new(BTN_ABOUT, "About", 120.0, 40.0);
//         canvas.add_widget(Box::new(wb_about));
//
//         //Create the TheCanvas
//         Self {
//             canvas,
//             sc1,
//             sc2,
//             sr,
//             sp,
//             arrow_head,
//             stxt,
//             stxtname,
//             stxtval,
//         }
//     }
//
//     pub fn canvas(&self) -> &BasicCanvas {
//         &self.canvas
//     }
//     pub fn canvas_mut(&mut self) -> &mut BasicCanvas {
//         &mut self.canvas
//     }
//
//     fn update(&mut self, world: &TheWorld) {
//         // Get state of traffic light and set appropriate color
//         let tlc = if world.tl.state == Signal::Stop {
//             Color32::RED
//         } else {
//             Color32::GREEN
//         };
//         self.sc2.borrow_mut().set_fill_color(tlc);
//
//         // Update gauge pointer
//         let mut ah_pos = self.arrow_head.borrow_mut().location();
//         ah_pos.x = 100.0 + 8.0 * (world.gauge.pointer() as f32);
//         self.arrow_head.borrow_mut().move_to(ah_pos);
//
//         // Update thing state, color coded
//         match world.thing.state {
//             ThingState::StateA => {
//                 self.sr.borrow_mut().set_fill_color(Color32::GOLD);
//                 self.stxt.borrow_mut().set_text("State A");
//             }
//             ThingState::StateB => {
//                 self.sr.borrow_mut().set_fill_color(Color32::CYAN);
//                 self.stxt.borrow_mut().set_text("State B");
//             }
//             _ => {}
//         }
//
//         //Update name
//         let name: String = "Name: ".to_owned() + &world.name.clone();
//         self.stxtname.borrow_mut().set_text(name);
//
//         //Update val_string
//         //let val = 42.3;
//         let val = world.value;
//         //let val_string: String = format!("{}{}", "Value: ", val);
//         let val_string: String = format!("{}{:.2}", "Value: ", val);
//         self.stxtval.borrow_mut().set_text(val_string);
//     }
// }

#[derive(Debug)]
enum ActiveDialog {
    None,
    About(MessageBoxDlg),
    EnterName(TextEntryDlg),
    EnterValue(DragFloatDlg),
}

impl ActiveDialog {
    pub fn is_active(&self) -> bool {
        !matches!(self, ActiveDialog::None)
    }
}

/// Main application structure.
///
/// Represents the root of the application and contains
/// the main canvas with all UI components
/// and if used, a World or Model struct containing program data and logic.
#[derive(Debug)]
struct TheApp {
    world: Box<TheWorld>,
    canvas: TheCanvas,
    msgs: Vec<WidgetMsg>,
    dialog: ActiveDialog,
    timer: Timer,
}

impl TheApp {
    /// Creates a new instance of the application
    /// intended to demonstrate usage of gui_lib.
    ///
    /// # Returns
    /// A new `TheApp` instance initialized with a canvas
    /// and world.

    pub fn new() -> Self {
        Self {
            world: Box::new(TheWorld::new()),
            canvas: TheCanvas::new(),
            msgs: Vec::new(),
            dialog: ActiveDialog::None,
            timer: Timer::new(0.5),
        }
    }

    //impl TheApp {
    fn handle_msg(&mut self, msg: WidgetMsg) {
        match msg {
            WidgetMsg::ButtonClicked(id) => {
                self.handle_button(id);
            }
            WidgetMsg::SliderChanged(id, value) => {
                self.handle_slider(id, value);
            }
            WidgetMsg::DragFloatChanged(id, value) => {
                self.handle_drag_float(id, value);
            }
            WidgetMsg::DialogAcceptedText(id, text) => {
                self.handle_text_entry(id, text);
            }
            WidgetMsg::DialogAcceptedDragFloat(id, val) => {
                self.handle_drag_float_dlg(id, val);
            }
            _ => {}
        }
    }
    // ui.label("gui_lib demo v0.1\n");
    // ui.label("Written in Rust + egui");
    fn handle_button(&mut self, id: ButtonId) {
        match id {
            BTN_ABOUT => {
                self.dialog = ActiveDialog::About(MessageBoxDlg::new(
                    DLG_ABOUT,
                    "About",
                    "gui_lib demo v0.1\nWritten in Rust + egui",
                ));
            }
            BTN_ENTER_NAME => {
                self.dialog = ActiveDialog::EnterName(TextEntryDlg::new(
                    //"enter_name_dialog",
                    DLG_ENTER_NAME,
                    "Enter name",
                    "Name:",
                    self.world.name.clone(),
                ));
            }
            BTN_ENTER_VALUE => {
                let mut dlg = DragFloatDlg::new(
                    DLG_ENTER_VALUE,
                    "Enter value",
                    "Value:",
                    self.world.value as f32,
                );
                dlg.set_speed(1.0);
                dlg.set_decimal(1);
                self.dialog = ActiveDialog::EnterValue(dlg);
            }
            BTN_RUN_PAUSE => {
                if self.timer.is_running() {
                    self.timer.pause();
                } else {
                    self.timer.run();
                }
            }
            BTN_STATE_A => {
                self.world.thing.state = ThingState::StateA;
            }
            BTN_STATE_B => {
                self.world.thing.state = ThingState::StateB;
            }

            _ => {}
        }
    }

    fn handle_slider(&mut self, id: SliderId, value: f32) {
        match id {
            SLIDER_GAUGE => {
                self.world.gauge.set_pointer(value.into());
            }
            SLIDER_ANOTHER => {
                //Do something else
            }
            _ => {}
        }
    }

    fn handle_drag_float(&mut self, id: DragFloatId, value: f32) {
        match id {
            DRAGFLOAT_GAUGE => {
                self.world.gauge.set_pointer(value.into());
            }
            _ => {}
        }
    }

    fn handle_text_entry(&mut self, id: TextEntryDlgId, text: String) {
        match id {
            DLG_ENTER_NAME => {
                self.world.name = text.clone();
            }
            _ => {}
        }
    }

    fn handle_drag_float_dlg(&mut self, id: DragFloatDlgId, val: f32) {
        match id {
            DLG_ENTER_VALUE => {
                self.world.value = val as f64;
            }
            _ => {}
        }
    }

    fn draw_dialog(&mut self, ctx: &egui::Context) {
        let mut close = false;

        match &mut self.dialog {
            ActiveDialog::None => {}

            ActiveDialog::About(dlg) => {
                close = dlg.do_modal(ctx, &mut self.msgs);
            }

            ActiveDialog::EnterName(dlg) => {
                close = dlg.do_modal(ctx, &mut self.msgs);
            }

            ActiveDialog::EnterValue(dlg) => {
                close = dlg.do_modal(ctx, &mut self.msgs);
            }

            _ => {}
        }
        if close {
            self.dialog = ActiveDialog::None;
        }
    }
}

/// The eframe::App trait is the bridge between your custom application logic
/// and the eframe framework that handles all the platform-specific details
/// of creating a window and running an event loop.
///
/// In this demonstration app a timer loop is used to advance the world at a rate
/// slower than the frame rate of the event loop. This allows better control of
/// running a simulation. For a simpler simulation the world might just advance
/// with the frame rate.
/// The frame rate should be set for smooth widget interaction. Typically 60 FPS,
/// (16 millisecond interval) but can be faster if the simulation is fast enough.
/// If there is no simulation there is no need to call world.advance.
/// For a basic program there is no need for a world object. All state and logic
/// can live directly in the TheApp.
impl eframe::App for TheApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // ----Simulation/animation. Not needed for all programs.
        if self.timer.is_time(ctx) {
            self.world.advance(); // advance world one tick
            self.canvas.update(&self.world); // update canvas
        }

        // -----Establish event loop
        self.msgs.clear(); // establish invariant: Belt and suspenders
        // Draw shapes and widgets on the canvas,
        // and collect all messages from widgets (pushes into self.msgs)
        self.canvas.canvas.render(ctx, &mut self.msgs);

        // ------- collect messages from the active dialog (pushes into self.msgs)
        // Must be after render and before handle_msg,
        // so that dialogs can be closed by the user.
        self.draw_dialog(ctx);

        // ------------ Handle messages if any exist
        if !self.msgs.is_empty() {
            // Move msgs out of self so we can mutably borrow self inside the loop.
            let mut msgs = std::mem::take(&mut self.msgs);
            // Handle messages
            for msg in msgs.drain(..) {
                self.handle_msg(msg);
            }
            // --------- Put the buffer back (empty, but keeps its capacity).
            self.msgs = msgs;

            // ----- Update canvas once after all state changes:
            self.canvas.update(&self.world);
        }
        // schedule the next frame redraw after 16 milliseconds (60 FPS)
        // Frame rate can be set faster or slower than 60 FPS.
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }
}

pub fn run_the_app() -> Result<(), eframe::Error> {
    eframe::run_native(
        "GUI Draw Example",
        gl::native_options(),
        //eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(eframe::egui::Visuals::light()); //light theme
            let app = Box::new(TheApp::new());
            Ok(app)
        }),
    )
}
