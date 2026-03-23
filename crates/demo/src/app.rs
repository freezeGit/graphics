//! ## Application. struct TheApp is the main structure and entry point of the application.
//! - Contains a `Canvas` for holding a collection of shapes.
//! - Provides methods for creating and updating the UI.
//! - May contain a 'World" (or 'Model' or 'Document')
//!   which contains all  non-gui program data and logic

// app.rs

use ::gui_lib as gl;
use egui::Context;
use gui_lib::{
    ButtonId, Dialog, DragFloatDlg, DragFloatDlgId, DragFloatId, MessageBoxDlg, NilDlg, SliderId,
    TextEntryDlg, TextEntryDlgId, Timer, WidgetMsg,
};

use crate::canvas::TheCanvas;
use crate::ids::{
    BTN_ABOUT, BTN_ENTER_NAME, BTN_ENTER_VALUE, BTN_RUN_PAUSE, BTN_STATE_A, BTN_STATE_B, DLG_ABOUT,
    DLG_ENTER_NAME, DLG_ENTER_VALUE, DRAGFLOAT_GAUGE, SLIDER_ANOTHER, SLIDER_GAUGE,
};
use crate::world::{TheWorld, ThingState};

/// Main application structure.
///
/// Represents the root of the application and contains
/// the main canvas with all UI components
/// and if used, a World or Model struct containing program data and logic.
#[derive(Debug)]
pub struct TheApp {
    world: Box<TheWorld>,
    canvas: TheCanvas,
    msgs: Vec<WidgetMsg>,
    //dialog: ActiveDialog,
    timer: Timer,
}

impl TheApp {
    /// Creates a new instance of the application
    /// intended to demonstrate usage of gui_lib.
    ///
    /// # Returns
    /// A new `TheApp` instance initialized with a canvas and wold
    /// as well as a vec of messages, an active dialog, and a timer.

    pub fn new() -> Self {
        Self {
            world: Box::new(TheWorld::new()),
            canvas: TheCanvas::new(),
            msgs: Vec::new(),
            //dialog: ActiveDialog::None,
            timer: Timer::new(0.5),
        }
    }

    // Handle messages --------------------------

    /// What to do with [`WidgetMsg`] messages from widgets and dialogs.
    /// This is the only communication between the GUI and the program code.
    /// In this demo app program data and logic are encapsulated in struct TheWorld.
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
            WidgetMsg::DialogAcceptedDragFloat(id, val) => {
                self.handle_drag_float_dlg(id, val);
            }
            _ => {}
        }
    }

    fn handle_button(&mut self, id: ButtonId) {
        match id {
            BTN_ABOUT => {
                self.canvas.canvas.set_dialog(Box::new(MessageBoxDlg::new(
                    DLG_ABOUT,
                    "About",
                    "gui_lib demo v0.1\nWritten in Rust + egui",
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
                    "Value:",
                    self.world.value as f32,
                );
                dlg.set_speed(1.0);
                dlg.set_decimal(1);
                self.canvas.canvas.set_dialog(Box::new(dlg));
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
    //}

    /// Calls [`Dialog::invoke_modal`] to invoke a modal dialog.
    /// # Parameters
    /// - `ctx`: A reference to the [`Context`] object, which provides the necessary
    ///   environment for rendering and interaction with the modal dialog.
    /// # Returns
    /// - `bool`: Returns `true` if the dialog is closed by the user,
    ///   or `false` if the dialog is still open.
    fn invoked_dialog_closed(&mut self, ctx: &Context) -> bool {
        self.canvas
            .canvas
            .get_mut_dialog()
            .invoke_modal(ctx, &mut self.msgs)
    }

    /// Executes the simulation logic during each cycle.
    ///
    /// This method is not required for many programs. It is only needed
    /// in case a simulation is run.
    ///
    /// This method checks if the simulation timer indicates that it's time
    /// to run the next simulation step. If so, it advances the state of the
    /// simulation's world model by one step by calling [`TheWorld::advance`] and updates
    /// the canvas to reflect the world’s new state by calling [`TheCanvas::update`].
    ///
    /// # Parameters
    /// - `ctx`: A reference to the [`Context`] object.
    fn run_simulation(&mut self, ctx: &Context) {
        if self.timer.is_time(ctx) {
            self.world.advance(); // advance world one tick
            self.canvas.update(&self.world); // update canvas
        }
    }
} // end impl TheApp

// eframe::App trait -------------------------------

/// The eframe::App trait is the bridge between your custom application logic
/// and the eframe framework that handles all the platform-specific details
/// of creating a window and running an event loop.
///
/// Function update is called each time the UI needs repainting
///
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
/// The frame rate should be set for smooth widget interaction. Typically 60 FPS,
/// (16 millisecond interval) but can be faster if the simulation is fast enough.
///
/// If there is no simulation there is no need to call world.advance.
/// By default (if [`Context::request_repaint()`] or [`Context::request_repaint_after()`] is not called)
/// egui is reactive, meaning it only repaints when there's an input event
/// (like mouse movement or a key press).
///
/// For a basic program there is no need for a world object. All state and logic
/// can live directly in the TheApp.
/// ///
/// # Parameters
/// - `ctx`: A reference to the [`Context`] object, which provides the necessary
///   environment for rendering and interaction with the modal dialog.
/// - `frame`: A reference to the [`Frame`] object. Not used in this demo.

impl eframe::App for TheApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // ----------- Establish event loop
        self.msgs.clear(); // establish invariant: Belt and suspenders
        // Draw active dialog over the canvas.
        // When the dialog is closed push its message into self.msgs.
        // Pause simulation while dialog is open.
        if self.invoked_dialog_closed(ctx) {
            // If the active dialog has been closed, set the dialog to nil
            self.canvas.canvas.set_dialog(Box::new(NilDlg));
            // Simulation/animation. Not needed for many programs.
            self.run_simulation(ctx);
        }
        // Draw shapes and widgets on the canvas.
        // Collect all messages from widgets into self.msgs.
        self.canvas.canvas.render(ctx, &mut self.msgs);

        // ------------ Handle messages if any exist
        if !self.msgs.is_empty() {
            // Move msgs out of self so we can mutably borrow self inside the loop.
            let mut msgs = std::mem::take(&mut self.msgs);
            // Handle messages
            for msg in msgs.drain(..) {
                self.handle_msg(msg);
            }
            // Put the buffer back (empty, but keeps its capacity).
            self.msgs = msgs;
            // Update canvas once after all state changes:
            self.canvas.update(&self.world);
        }
        // Redraw after 16 milliseconds (60 FPS)
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }
}