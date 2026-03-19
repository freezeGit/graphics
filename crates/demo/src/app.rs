//! ## Application. struct TheApp is the main structure and entry point of the application.
//! - Contains a `Canvas` for holding a collection of shapes.
//! - Provides methods for creating and updating the UI.
//! - May contain a 'World" (or 'Model' or 'Document')
//!   which contains all  non-gui program data and logic

// app.rs

use ::gui_lib as gl;
use egui::Context;
use gui_lib::{
    ButtonId, Dialog, DragFloatDlg, DragFloatDlgId, DragFloatId, MessageBoxDlg, SliderId,
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
    /// A new `TheApp` instance initialized with a canvas and wold
    /// as well as a vec of messages, an active dialog, and a timer.

    fn new() -> Self {
        Self {
            world: Box::new(TheWorld::new()),
            canvas: TheCanvas::new(),
            msgs: Vec::new(),
            dialog: ActiveDialog::None,
            timer: Timer::new(0.5),
        }
    }

    /// What to do with messages from widgets and dialogs.
    fn handle_msg(&mut self, msg: WidgetMsg) {
        match msg {
            WidgetMsg::ButtonClicked(id) => {
                self.handle_button(id);
            },
            WidgetMsg::SliderChanged(id, value) => {
                self.handle_slider(id, value);
            },
            WidgetMsg::DragFloatChanged(id, value) => {
                self.handle_drag_float(id, value);
            },
            WidgetMsg::DialogAcceptedText(id, text) => {
                self.handle_text_entry(id, text);
            },
            WidgetMsg::DialogAcceptedDragFloat(id, val) => {
                self.handle_drag_float_dlg(id, val);
            },
            _ => {},
        }
    }

    fn handle_button(&mut self, id: ButtonId) {
        match id {
            BTN_ABOUT => {
                self.dialog = ActiveDialog::About(MessageBoxDlg::new(
                    DLG_ABOUT,
                    "About",
                    "gui_lib demo v0.1\nWritten in Rust + egui",
                ));
            },
            BTN_ENTER_NAME => {
                self.dialog = ActiveDialog::EnterName(TextEntryDlg::new(
                    //"enter_name_dialog",
                    DLG_ENTER_NAME,
                    "Enter name",
                    "Name:",
                    self.world.name.clone(),
                ));
            },
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
            },
            BTN_RUN_PAUSE => {
                if self.timer.is_running() {
                    self.timer.pause();
                } else {
                    self.timer.run();
                }
            },
            BTN_STATE_A => {
                self.world.thing.state = ThingState::StateA;
            },
            BTN_STATE_B => {
                self.world.thing.state = ThingState::StateB;
            },

            _ => {},
        }
    }

    fn handle_slider(&mut self, id: SliderId, value: f32) {
        match id {
            SLIDER_GAUGE => {
                self.world.gauge.set_pointer(value.into());
            },
            SLIDER_ANOTHER => {
                //Do something else
            },
            _ => {},
        }
    }

    fn handle_drag_float(&mut self, id: DragFloatId, value: f32) {
        match id {
            DRAGFLOAT_GAUGE => {
                self.world.gauge.set_pointer(value.into());
            },
            _ => {},
        }
    }

    fn handle_text_entry(&mut self, id: TextEntryDlgId, text: String) {
        match id {
            DLG_ENTER_NAME => {
                self.world.name = text.clone();
            },
            _ => {},
        }
    }

    fn handle_drag_float_dlg(&mut self, id: DragFloatDlgId, val: f32) {
        match id {
            DLG_ENTER_VALUE => {
                self.world.value = val as f64;
            },
            _ => {},
        }
    }

    /// Draw the active dialog, if any.
    /// Note: Simulation will continue to run while the dialog is open.
    /// If this is not desired, use button wb_run to pause the simulation first.
    fn draw_dialog(&mut self, ctx: &egui::Context) {
        let mut close = false;

        match &mut self.dialog {
            ActiveDialog::None => {},

            ActiveDialog::About(dlg) => {
                close = dlg.invoke_modal(ctx, &mut self.msgs);
            },

            ActiveDialog::EnterName(dlg) => {
                close = dlg.invoke_modal(ctx, &mut self.msgs);
            },

            ActiveDialog::EnterValue(dlg) => {
                close = dlg.invoke_modal(ctx, &mut self.msgs);
            },

            _ => {},
        }
        if close {
            self.dialog = ActiveDialog::None;
        }
    }
}

/// The eframe::App trait is the bridge between your custom application logic
/// and the eframe framework that handles all the platform-specific details
/// of creating a window and running an event loop.
/// It is called each time the UI needs repainting
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
/// By default (if ctx.request_repaint() or ctx.request_repaint_after() is not called)
/// egui is reactive, meaning it only repaints when there's an input event
/// (like mouse movement or a key press).
///
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
        // Redraw after 16 milliseconds (60 FPS)
        ctx.request_repaint_after(std::time::Duration::from_millis(16));

    }
}

// -----------------------------
/// enum ActiveDialog holds the currently active dialog.
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

// ----------------------------------

/// function run_the_app() starts a native (desktop) app.
/// Calls eframe::run_native() to create TheApp
/// Change constants xv and yv to adjust the width and height of the viewport.
const xwvp: f32 = 1200.0;  // Width of viewport in pixels.
const ywvp: f32 = 800.0;  // Height of viewport in pixels.
pub fn run_the_app() -> Result<(), eframe::Error> {
    eframe::run_native(
        "gui_lib sbx",
        gl::native_options(xwvp, ywvp),
        //eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(eframe::egui::Visuals::light()); //light theme
            let app = Box::new(TheApp::new());
            Ok(app)
        }),
    )
}
