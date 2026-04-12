//! ## Application. struct TheApp is the main structure and entry point of the application.
//! - Contains a `Canvas` for holding a collection of shapes.
//! - Provides methods for creating and updating the UI.
//! - May contain a 'World" (or 'Model' or 'Document')
//!   which contains all  non-gui program data and logic

// app.rs

use std::time::Duration;
use ::gui_lib as gl;
use egui::Context;
use gui_lib::{
    ButtonId, Dialog, DragFloatDlg, DragFloatDlgId, DragFloatId, MessageBoxDlg, MultiTextEntryDlg,
    MultiTextEntryDlgId, NilDlg, SliderId, TextEntryDlg, TextEntryDlgId, TextEntryField, Timer,
    WidgetMsg, app_gl,
};

use crate::canvas::TheCanvas;
// use crate::ids::{
//     BTN_ABOUT, BTN_ENTER_NAME, BTN_ENTER_VALUE, BTN_RUN_PAUSE, BTN_STATE_A, BTN_STATE_B, DLG_ABOUT,
//     DLG_ENTER_NAME, DLG_ENTER_VALUE, DRAGFLOAT_GAUGE, SLIDER_ANOTHER, SLIDER_GAUGE,
// };
use crate::ids::*;
use crate::world::{TheWorld, ThingState};

/// Main application structure.
///
/// Represents the root of the application and contains
/// the main canvas with all UI components
/// and if used, a world or model struct containing program data and logic.
#[derive(Debug)]
pub struct TheApp {
    world: Box<TheWorld>,
    canvas: TheCanvas,
    msgs: Vec<WidgetMsg>,
    sim_timer: Timer,
}

impl TheApp {
    // pub fn new(): implemented in trait run::UserApp

    // Handle messages --------------------------

    /// What to do with [`WidgetMsg`] messages from widgets and dialogs.
    /// This is the only communication between the GUI and the program code.
    /// Program data and logic are encapsulated in struct [`TheWorld`].
    fn handle_msg(&mut self, msg: WidgetMsg) {
        match msg {
            WidgetMsg::ButtonClicked(id) => {
                self.handle_button(id);
            }
            WidgetMsg::DragFloatChanged(id, value) => {
                self.handle_drag_float(id, value);
            }
            WidgetMsg::DialogAcceptedText(id, text) => {
                self.handle_text_entry(id, text);
            }
            WidgetMsg::DialogAcceptedMultiTextEntry(id, values) => {
                self.handle_multi_text_entry(id, values);
            }
            WidgetMsg::DialogAcceptedDragFloat(id, val) => {
                self.handle_drag_float_dlg(id, val);
            }
            _ => {}
        }
    }

    /// Handle button messages
    fn handle_button(&mut self, id: ButtonId) {
        match id {
            BTN_ABOUT => {
                self.canvas.canvas.set_dialog(Box::new(MessageBoxDlg::new(
                    DLG_ABOUT,
                    "About",
                    "Demonstration app using the gui_lib library.\n\
                    Can be used as a template to get started with gui_lib.\n\
                    Written in Rust + egui.",
                )));
            }

            BTN_PERSON => {
                self.canvas
                    .canvas
                    .set_dialog(Box::new(MultiTextEntryDlg::new(
                        DLG_ENTER_PERSON,
                        "Enter person data",
                        [
                            TextEntryField::new("name", "Name", self.world.person.name.clone()),
                            TextEntryField::new("city", "City", self.world.person.city.clone()),
                            TextEntryField::new(
                                "address",
                                "Address",
                                self.world.person.address.clone(),
                            ),
                        ],
                    )));
            }

            BTN_ENTER_NAME => {
                self.canvas.canvas.set_dialog(Box::new(TextEntryDlg::new(
                    DLG_ENTER_NAME,
                    "Enter name",
                    "Name:",
                    self.world.name.clone(),
                )));
            }

            BTN_ENTER_VALUE => {
                let mut dlg = DragFloatDlg::new(
                    DLG_ENTER_VALUE,
                    "Enter value",
                    //"Value:",
                    self.world.value as f32,
                );
                dlg.set_speed(1.0);
                dlg.set_decimal(1);
                self.canvas.canvas.set_dialog(Box::new(dlg));
            }

            BTN_RUN_PAUSE => {
                if self.sim_timer.is_running() {
                    self.sim_timer.pause();
                } else {
                    self.sim_timer.run();
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

    /// Handle drag float messages
    fn handle_drag_float(&mut self, id: DragFloatId, value: f32) {
        match id {
            DRAGFLOAT_GAUGE => {
                self.world.gauge.set_pointer(value.into());
            }

            _ => {}
        }
    }

    /// Handle text entry messages
    fn handle_text_entry(&mut self, id: TextEntryDlgId, text: String) {
        match id {
            DLG_ENTER_NAME => {
                self.world.name = text.clone();
            }

            _ => {}
        }
    }

    fn handle_multi_text_entry(&mut self, id: MultiTextEntryDlgId, values: Vec<(String, String)>) {
        match id {
            DLG_ENTER_PERSON => {
                for item in values {
                    let (item_id, text) = item;
                    match item_id.as_str() {
                        "name" => {
                            self.world.person.name = text;
                        }
                        "city" => {
                            self.world.person.city = text;
                        }
                        "address" => {
                            self.world.person.address = text;
                        }
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }

    /// Handle drag float dialog messages
    fn handle_drag_float_dlg(&mut self, id: DragFloatDlgId, val: f32) {
        match id {
            DLG_ENTER_VALUE => {
                self.world.value = val as f64;
            }

            _ => {}
        }
    }

    // Helper functions for App::update() --------------------------

    /// Establish event loop.
    ///
    /// Invoke active dialog and collect emitted message in [`Self::msgs`].
    ///
    /// Run simulation logic when dialog is not open (if program includes a simulation).
    ///
    /// Render canvas and collect any emitted widgets messages in [`Self::msgs`].

    fn event_loop(&mut self, ctx: &Context) {
        self.msgs.clear(); // establish invariant: Belt and suspenders

        // Draw shapes and widgets on the canvas.
        // Collect all messages from widgets into self.msgs.
        self.canvas.canvas.render(ctx, &mut self.msgs);

        // Draw active dialog.
        // When the dialog is closed push its message into self.msgs.
        // Pause simulation while dialog is open.
        if self.invoked_dialog_closed(ctx) {
            // If the active dialog has been closed, set the dialog to nil
            self.canvas.canvas.set_dialog(Box::new(NilDlg));
            // Simulation/animation. Not needed for many programs.
            self.run_simulation(ctx); // Skip this line if there is no simulation.
        }
    }

    /// Calls [`Dialog::invoke_modal`] to draw and get a message from a modal dialog.
    ///
    /// Parameter `ctx`: A reference to the [`Context`] object.
    ///
    /// Returns `true` if the user has closed the dialog,
    /// or `false` if the dialog is still open.
    fn invoked_dialog_closed(&mut self, ctx: &Context) -> bool {
        self.canvas
            .canvas
            .get_mut_dialog()
            .invoke_modal(ctx, &mut self.msgs)
    }

    /// Executes the simulation logic.
    /// This method is not required for many programs. It is only needed
    /// in case a simulation is run.
    ///
    /// This method checks if the simulation timer indicates that it's time
    /// to run the next simulation step. If so, it advances the state of the
    /// simulation's world model by one step by calling [`TheWorld::advance`] and then
    /// updates the canvas to reflect the world’s new state by calling [`TheCanvas::update`].
    ///
    /// Parameter `ctx`: A reference to the [`Context`] object.
    // fn run_simulation(&mut self, ctx: &Context) {
    //     if self.timer.is_time(ctx) {
    //         self.world.advance(); // advance world one tick
    //         self.canvas.update(&self.world); // update canvas
    //     }
    // }
    pub fn run_simulation(&mut self, ctx: &egui::Context) {
        if !self.sim_timer.is_running() {
            return;
        }

        // if !self.simulation_allowed() { // TDJ: debug
        //     return;
        // }

        // if self.fast_forward { //
        //     self.run_fast_forward_batch();
        //     self.canvas.update(&self.world);
        //     ctx.request_repaint();
        //     return;
        // }

        let steps = self.sim_timer.ready_count().min(4);

        for _ in 0..steps {
            self.world.advance();
        }

        if steps > 0 {
            self.canvas.update(&self.world);
        }

        ctx.request_repaint_after(self.sim_timer.remaining());
    }
    // fn run_simulation(&mut self, ctx: &Context) {
    //     if self.timer.is_time(ctx) {
    //         //println!("Time: {}", ctx.input(|i| i.time));  // TDJ: debug
    //         self.world.advance(); // advance world one tick
    //         self.canvas.update(&self.world); // update canvas
    //     }
    // }
    // ------------------------------------------------

    /// Handle messages if any exist
    /// # Related Methods
    /// - [`handle_msg`]: Called for each individual message in the `msgs` buffer.
    /// - [`canvas.update`]: Updates the canvas to reflect changes in the `world`.
    fn handle_emitted_messages(&mut self) {
        // Handle messages if any exist
        if !self.msgs.is_empty() {
            // Move msgs out of self so we can mutably borrow self inside the loop.
            let mut msgs = std::mem::take(&mut self.msgs);
            // Handle messages and drain the buffer.
            for msg in msgs.drain(..) {
                self.handle_msg(msg);
            }
            // Put the buffer back (empty, but keeps its capacity).
            self.msgs = msgs;

            // Update canvas to reflect all state changes:
            self.canvas.update(&self.world);
        }
    }
} // end impl TheApp

// eframe::App trait -------------------------------

/// The eframe::App trait is the bridge between the user's custom application logic
/// and the eframe framework that handles all the platform-specific details
/// of creating a window and running an event loop.
///
/// Function `update` is called each time the UI needs repainting:
/// [fn update](https://docs.rs/eframe/latest/eframe/trait.App.html#tymethod.update)
///
/// If there are background processes or animation:
/// you can schedule the next frame redraw after 16 milliseconds (60 FPS)
/// for smooth responsiveness.
///
/// In this demonstration app a timer loop is used to advance the world at a rate
/// slower than the frame rate of the event loop. This allows better control of
/// running a simulation. For a simpler simulation the world might just advance
/// with the frame rate.
/// The frame rate should be set for smooth interaction. Typically 60 FPS,
/// (16 millisecond interval) but can be faster if the simulation is fast enough.
///
/// If there is no simulation there is no need to call or define [`TheWorld::advance`].
/// By default (if [`Context::request_repaint()`] or [`Context::request_repaint_after()`] is not called)
/// egui is reactive, meaning it only repaints when there's an input event
/// (like mouse movement or a key press).
/// See: <https://docs.rs/egui/latest/egui/struct.Context.html#method.request_repaint_after>
///
/// For a basic program there is no need for a world object. All state and logic
/// can live directly in the TheApp.
///
/// # Parameters
/// - `ctx`: A reference to the [`Context`] object, which provides the necessary environment.
/// - `frame`: A reference to the [`eframe::Frame`] object. Not used in this demo.
impl eframe::App for TheApp {
    /// Called each time the UI needs repainting.
    /// Often 60 FPS, set by calling [`Context::request_repaint_after()`].
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        //println!("Update: {}", ctx.input(|i| i.time));  // TDJ: debug
        // Establish event loop
        self.event_loop(ctx);

        // Handle messages if any exist
        self.handle_emitted_messages();

        // Redraw after 16 milliseconds (60 FPS). Useful for animation.
        // If there is no animation, you can skip this line.
        // See the comment in the App trait above.
        // TDJ: How to request repaint
        //ctx.request_repaint_after(std::time::Duration::from_millis(1000));
        //ctx.request_repaint_after(std::time::Duration::from_millis(16));
        //ctx.request_repaint();
    }
} // end impl eframe::App

// use std::time::Duration;
// impl eframe::App for TheApp {
//     fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
//         let now = std::time::Instant::now();
//
//         // Use your own timing, not ctx.input().time
//         if let Some(last) = self.last_update {
//             let delta = now - last;
//             println!("Delta since last update: {:?}", delta);
//         }
//         self.last_update = Some(now);
//
//         // Request repaint exactly 1 second from NOW
//         ctx.request_repaint_after(Duration::from_secs(1));
//     }
// }

/// A trait representing a user-defined application that extends the functionality
/// of the `eframe::App` framework.
///
/// This trait is designed to provide a flexible and standardized way for users to define
/// and initialize their custom applications when using the `eframe` framework.
/// The `new()` function must have an empty parameter list. This guarantees that
/// the application `new()` constructor will have the correct signature to be called by the
/// `run_the_app()` function.
impl app_gl::UserApp for TheApp {
    /// Creates a new instance of TheApp application.
    /// It is intended to demonstrate usage of gui_lib.
    ///
    /// # Returns
    /// A new `TheApp` instance initialized with a canvas and world
    /// as well as a vector for messages, and a timer.
    fn new() -> Self {
        Self {
            world: Box::new(TheWorld::new()),
            canvas: TheCanvas::new(),
            msgs: Vec::new(),
            // TDJ: use constant instead of 500?
            sim_timer: Timer::new(Duration::from_millis(500)),
        }
    }
} // end impl run::UserApp
