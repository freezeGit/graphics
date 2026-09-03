//! ## Application. struct TheApp is the main structure and entry point of the application.
//! - Contains a `Canvas` for holding a collection of shapes.
//! - Provides methods for creating and updating the UI.
//! - All method handing methods in this module need application specific customizations.
//! - Contains a 'World" which contains all non-gui program data and logic. .

// app1.rs

mod app_internal; // internal functions that do not require application specific customizations

use crate::canvas::TheCanvas;
use crate::ids::*;
use crate::inits;
//use crate::world::delta_ones::Deltas;
//use crate::world::emerge::BitArray;
use crate::world::{Rule, TheWorld};
use egui::Context;
#[allow(unused_imports)]
use gui_lib::{
    ButtonId, DialogId, DragFloatDlg, DragFloatDlgId, DragFloatId, MessageBoxDlg,
    MultiTextEntryDlg, MultiTextEntryDlgId, NilDlg, RadioBoxesDlg, RadioBoxesDlgId,
    RadioBoxesField, SimTimer, SliderId, TextEntryDlg, TextEntryDlgId, TextEntryField, WidgetMsg,
    World, app_gl,
};

/// Main application structure.
///
/// Represents the root of the application and contains
/// the main canvas with all UI components
/// and a world struct containing program data and logic.
//#[derive(Debug)] // TDJ: Debug is not needed for this app
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
        //self.canvas.canvas.render(ctx, &mut self.msgs);
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
            sim_timer: SimTimer::new(inits::INTERVAL, inits::SMOOTH_ANIMATION, inits::BATCH_SIZE),
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

            WidgetMsg::DialogAcceptedRadioBoxes(id, value) => {
                self.handle_radio_boxes(id, value);
            }
            WidgetMsg::DialogAcceptedMultiTextEntry(id, values) => {
                self.handle_multi_text_entry(id, values);
            }
            WidgetMsg::DialogAcceptedText(id, text) => {
                self.handle_text_entry(id, text);
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
                    "Emergence. \n\
                    Delta ones graph.",
                )));
            }

            BTN_DELTAS => {
                self.canvas
                    .canvas
                    .set_dialog(Box::new(MultiTextEntryDlg::new(
                        DLG_ENTER_SPECS,
                        "Enter rule and sample size:",
                        [
                            TextEntryField::new(
                                "rule",
                                "Rule (0 to 15)",
                                self.world.rule.number().to_string(),
                            ),
                            TextEntryField::new(
                                "sample_size",
                                "Sample size",
                                self.world.sample_size.to_string(),
                            ),
                        ],
                    )));
            }

            _ => {}
        }
    }

    fn handle_radio_boxes(&mut self, id: RadioBoxesDlgId, value: i32) {
        match id {
            _ => {}
        }
    }

    fn handle_multi_text_entry(&mut self, id: MultiTextEntryDlgId, values: Vec<(String, String)>) {
        match id {
            DLG_ENTER_SPECS => {
                let mut rule = 0;
                let mut sample_size = 0;

                //self.sim_timer.pause();

                let mut bad_val = false;

                for item in values {
                    let (item_id, text) = item;
                    match item_id.as_str() {
                        "rule" => match text.trim().parse::<u8>() {
                            Ok(number) if number < 16 => {
                                rule = number;
                            }
                            Ok(number) => {
                                bad_val = true;
                                eprintln!(
                                    "Invalid rule number: {number}. Rule must be between 0 and 15."
                                );
                            }
                            Err(err) => {
                                bad_val = true;
                                eprintln!("Could not parse rule number {:?}: {err}", text);
                            }
                        },
                        "sample_size" => match text.trim().parse::<u32>() {
                            // Ok(number)  => {
                            //     sample_size = number;
                            //}
                            Ok(number) if number >= 1 => {
                                sample_size = number;
                            }
                            Ok(number) => {
                                bad_val = true;
                                eprintln!("Invalid bits number: {number}. Bits number too small.");
                            }
                            Err(err) => {
                                bad_val = true;
                                eprintln!("Could not parse bits number {:?}: {err}", text);
                            }
                        },

                        _ => {}
                    }
                }

                if bad_val {
                    self.canvas.canvas.set_dialog(Box::new(MessageBoxDlg::new(
                        DLG_BAD_VALS,
                        "Error Message",
                        "Bad value(s) entered.",
                    )));
                } else {
                    self.world.rule = Rule::new(rule);
                    self.world.sample_size = sample_size;

                    // Update the deltas graph
                    let deltas = self.world.recalc_deltas();
                    for i in 0..deltas.len() {
                        self.canvas
                            .view_handles
                            .dgr
                            .borrow_mut()
                            .set_mean_y(i, deltas.get_deltas(i).mean() as f32);
                    }
                }
            }

            _ => {}
        }
    }

    fn handle_text_entry(&mut self, id: TextEntryDlgId, text: String) {
        match id {
            _ => {}
        }
    }
} // end impl TheApp
