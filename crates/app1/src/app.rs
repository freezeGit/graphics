//! ## Application. struct TheApp is the main structure and entry point of the application.
//! - Contains a `Canvas` for holding a collection of shapes.
//! - Provides methods for creating and updating the UI.
//! - All method handing methods in this module need application specific customizations.
//! - Contains a 'World" which contains all non-gui program data and logic.
//!
//! This demo app1 is intended to demonstrate usage of gui_lib, and for use as a template.

// app1.rs

mod app_internal; // internal functions that do not require application specific customizations

use ::gui_lib as gl;
use egui::Context;
use gui_lib::{
    ButtonId, Dialog, DragFloatDlg, DragFloatDlgId, DragFloatId, MessageBoxDlg, MultiTextEntryDlg,
    MultiTextEntryDlgId, NilDlg, SimTimer, SliderId, TextEntryDlg, TextEntryDlgId, TextEntryField,
    WidgetMsg, app_gl,
};
use std::time::Duration;

use crate::canvas::TheCanvas;
// use crate::ids::{
//     BTN_ABOUT, BTN_ENTER_NAME, BTN_ENTER_VALUE, BTN_RUN_PAUSE, BTN_STATE_A, BTN_STATE_B, DLG_ABOUT,
//     DLG_ENTER_NAME, DLG_ENTER_VALUE, DRAGFLOAT_GAUGE, SLIDER_ANOTHER, SLIDER_GAUGE,
// };
use crate::ids::*;
use crate::world::TheWorld;
use crate::world::world_demo::ThingState;

// --------- User customized application specific constants.  ----------------
/// User customized simulation parameters

// `INTERVAL`: Time between simulation steps in seconds
const INTERVAL: f64 = 0.5;

// `BATCH_SIZE`: Number of world advances to perform in a single simulation step
// during fast-forward of the simulation.
const BATCH_SIZE: u32 = 1001;

// `SIM_REPAINT_16MS`: If true, the simulation will request repaint at 16ms intervals.
// This may result in a smoother animation, but may also cause performance issues
// because of extra refresh requests. If false, the simulation will request repaint
// at intervals determined by INTERVAL.
const SIM_REPAINT_16MS: bool = false;

/// Main application structure.
///
/// Represents the root of the application and contains
/// the main canvas with all UI components
/// and a world struct containing program data and logic.
#[derive(Debug)]
pub struct TheApp {
    world: Box<TheWorld>,
    canvas: TheCanvas,
    msgs: Vec<WidgetMsg>,
    sim_timer: SimTimer,
}

// eframe::App trait -------------------------------

/// The eframe::App trait is the bridge between the user's custom application logic
/// and the eframe framework.
///
/// # Parameters
/// - `ctx`: A reference to the [`Context`] object, which provides the necessary environment.
/// - `frame`: A reference to the [`eframe::Frame`] object. Not used in this demo.
impl eframe::App for TheApp {
    /// Called each time the UI needs repainting.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Establish event loop
        self.event_loop(ctx);
        // Handle messages if any exist
        self.handle_emitted_messages();
    }
} // end impl eframe::App

// app_gl::UserApp trait -------------------------------

/// A trait representing a user-defined application.
///
/// The `new()` function must have an empty parameter list. This guarantees that
/// the application `new()` constructor will have the correct signature to be called by the
/// `run_the_app()` function.
impl app_gl::UserApp for TheApp {
    /// # Returns
    /// A new `TheApp` instance
    fn new() -> Self {
        Self {
            world: Box::new(TheWorld::new()),
            canvas: TheCanvas::new(),
            msgs: Vec::new(), // Vec<WidgetMsg>
            sim_timer: SimTimer::new(INTERVAL, BATCH_SIZE),
        }
    }
} // end impl app_gl::UserApp

impl TheApp {
    // -------- User customization below --------

    /// What to do with [`WidgetMsg`] messages from widgets and dialogs.
    /// All method handing methods in this module need application specific customizations.
    fn handle_msg(&mut self, msg: WidgetMsg) {
        match msg {
            WidgetMsg::ButtonClicked(id) => {
                self.handle_button(id);
            }
            WidgetMsg::DragFloatChanged(id, value) => {
                self.handle_drag_float(id, value);
            }
            // WidgetMsg::DialogAcceptedText(id, text) => {
            //     self.handle_text_entry(id, text);
            // }
            WidgetMsg::DialogAcceptedMultiTextEntry(id, values) => {
                self.handle_multi_text_entry(id, values);
            }
            WidgetMsg::DialogAcceptedDragFloat(id, val) => {
                self.handle_drag_float_dlg(id, val);
            }
            _ => {} // Other messages may not be handled in this app1                                                                                                                                other
        }
    }

    /// Handle button messages
    ///
    /// Requires application specific customization.
    fn handle_button(&mut self, id: ButtonId) {
        match id {
            BTN_ABOUT => {
                self.canvas.canvas.set_dialog(Box::new(MessageBoxDlg::new(
                    DLG_ABOUT,
                    "About",
                    "App using the gui_lib library.\n\
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

            // BTN_ENTER_NAME => {
            //     self.canvas.canvas.set_dialog(Box::new(TextEntryDlg::new(
            //         DLG_ENTER_NAME,
            //         "Enter name",
            //         "Name:",
            //         self.world.name.clone(),
            //     )));
            // }
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

            BTN_SLOW_FAST => {
                if self.sim_timer.fast_forward() {
                    //self.sim_timer.set_normal_speed();
                    self.sim_timer.exit_fast_forward();
                } else {
                    self.sim_timer.set_fast_forward();
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
    ///
    /// Requires application specific customization.
    fn handle_drag_float(&mut self, id: DragFloatId, value: f32) {
        match id {
            DRAGFLOAT_GAUGE => {
                self.world.gauge.set_pointer(value.into());
            }

            _ => {}
        }
    }

    // /// Handle text entry messages
    // ///
    // /// Requires application specific customization.
    // fn handle_text_entry(&mut self, id: TextEntryDlgId, text: String) {
    //     match id {
    //         DLG_ENTER_NAME => {
    //             self.world.name = text.clone();
    //         }
    //
    //         _ => {}
    //     }
    // }

    /// Handle handle_multi_text_entry messages
    ///
    /// Requires application specific customization.
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
    ///
    /// Requires application specific customization.
    fn handle_drag_float_dlg(&mut self, id: DragFloatDlgId, val: f32) {
        match id {
            DLG_ENTER_VALUE => {
                self.world.value = val as f64;
            }

            _ => {}
        }
    }
} // end impl TheApp
