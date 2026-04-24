//! ## Application. struct TheApp is the main structure and entry point of the application.
//! - Contains a `Canvas` for holding a collection of shapes.
//! - Provides methods for creating and updating the UI.
//! - Contains a 'World" which contains all  non-gui program data and logic.
//!
//! This demo app is intended to demonstrate usage of gui_lib, and for use as a template.

// app.rs

mod app_internal; // internal functions that do not rquire application specific customizations

use ::gui_lib as gl;
use egui::Context;
use gui_lib::{
    ButtonId, Dialog, DragFloatDlg, DragFloatDlgId, DragFloatId, MessageBoxDlg, MultiTextEntryDlg,
    MultiTextEntryDlgId, NilDlg, SliderId, TextEntryDlg, TextEntryDlgId, TextEntryField, SimTimer,
    WidgetMsg, app_gl,
};
use std::time::Duration;

use crate::canvas::TheCanvas;
// use crate::ids::{
//     BTN_ABOUT, BTN_ENTER_NAME, BTN_ENTER_VALUE, BTN_RUN_PAUSE, BTN_STATE_A, BTN_STATE_B, DLG_ABOUT,
//     DLG_ENTER_NAME, DLG_ENTER_VALUE, DRAGFLOAT_GAUGE, SLIDER_ANOTHER, SLIDER_GAUGE,
// };
use crate::ids::*;
use crate::world::{TheWorld};
use crate::world::world_demo::ThingState;

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
            // TDJ: use constant for simulation speed?
            // sim_timer: Timer::new(Duration::from_millis(500)),
            //sim_timer: Timer::new(Duration::from_millis(200)),
            sim_timer: SimTimer::new(0.5), //TDJ:
        }
    }
} // end impl run::UserApp

impl TheApp {
    // -------- User customization below --------
    //! All method handing methods in this module need application specific customizations.

    /// Handle button messages
    ///
    /// Requires application specific customization.
    fn handle_button(&mut self, id: ButtonId) {
        match id {
            BTN_ABOUT => {
                self.canvas.canvas.set_dialog(Box::new(MessageBoxDlg::new(
                    DLG_ABOUT,
                    "About",
                    "Demonstration app using the gui_lib library.\n\
                    Intended to be used as a template to get started.\n\
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

    /// Handle text entry messages
    ///
    /// Requires application specific customization.
    // fn handle_text_entry(&mut self, id: TextEntryDlgId, text: String) {
    //     match id {
    //         DLG_ENTER_NAME => {
    //             self.world.name = text.clone();
    //         }
    //
    //         _ => {}
    //     }
    // }

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
