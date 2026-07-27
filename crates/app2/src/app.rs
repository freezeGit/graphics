//! ## Application. struct TheApp is the main structure and entry point of the application.
//! - Contains a `Canvas` for holding a collection of shapes.
//! - Provides methods for creating and updating the UI.
//! - All method handing methods in this module need application specific customizations.
//! - Contains a 'World" which contains all non-gui program data and logic. .

// app1.rs

mod app_internal; // internal functions that do not require application specific customizations

//use ::gui_lib as gl;
use egui::Context;
// use gui_lib::{
//     ButtonId, Dialog, DialogId, DragFloatDlg, DragFloatDlgId, DragFloatId, MessageBoxDlg,
//     MultiTextEntryDlg, MultiTextEntryDlgId, NilDlg, RadioBoxesDlg, RadioBoxesDlgId,
//     RadioBoxesField, SimTimer, SliderId, TextEntryDlg, TextEntryDlgId, TextEntryField, WidgetMsg,
//     app_gl,
// };
use crate::inits;
#[allow(unused_imports)]
use gui_lib::{
    ButtonId, DialogId, DragFloatDlg, DragFloatDlgId, DragFloatId, MessageBoxDlg,
    MultiTextEntryDlg, MultiTextEntryDlgId, NilDlg, RadioBoxesDlg, RadioBoxesDlgId,
    RadioBoxesField, SimTimer, SliderId, TextEntryDlg, TextEntryDlgId, TextEntryField, WidgetMsg,
    app_gl,
};

use crate::canvas::TheCanvas;
use crate::ids::*;
//use crate::world::TheWorld;
use crate::world::emerge::BitArray;
use crate::world::{Rule, TheWorld};
//use crate::world::world_demo::ThingState;

/// Constants for simulation state choice. 1 = Run, 2 = Pause, 3 = Fast-forward.
const CHOICE_RUN: i32 = 1;
const CHOICE_PAUSE: i32 = 2;
const CHOICE_FAST: i32 = 3;
//const CHOICE_RESET: i32 = 4;
const CHOICE_OTHER: i32 = 100;
// ---------------------------

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
            // TDJ: Not needed for this app
            // WidgetMsg::DragFloatChanged(id, value) => {
            //     self.handle_drag_float(id, value);
            // }
            // WidgetMsg::DialogAcceptedText(id, text) => {
            //     self.handle_text_entry(id, text);
            // }
            WidgetMsg::DialogAcceptedRadioBoxes(id, value) => {
                self.handle_radio_boxes(id, value);
            }
            WidgetMsg::DialogAcceptedMultiTextEntry(id, values) => {
                self.handle_multi_text_entry(id, values);
            }
            // TDJ: Not needed for this app
            // WidgetMsg::DialogAcceptedDragFloat(id, val) => {
            //     self.handle_drag_float_dlg(id, val);
            // }
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
                    First working app",
                )));
            }

            BTN_SIM => {
                let current_choice =
                    //if self.sim_timer.is_running() && !self.sim_timer.fast_forward() {
                    if self.sim_timer.is_running() && self.sim_timer.normal_speed() {
                        CHOICE_RUN
                    } else if self.sim_timer.is_running() && self.sim_timer.fast_forward() {
                        CHOICE_FAST
                    } else if !self.sim_timer.is_running() {
                        CHOICE_PAUSE
                    } else {
                        CHOICE_OTHER
                    };

                self.canvas.canvas.set_dialog(Box::new(RadioBoxesDlg::new(
                    DLG_SIM_STATE,
                    "Sim state",
                    current_choice,
                    [
                        RadioBoxesField::new(CHOICE_RUN, "Run"),
                        RadioBoxesField::new(CHOICE_PAUSE, "Pause"),
                        RadioBoxesField::new(CHOICE_FAST, "Fast-forward"),
                    ],
                )));
            }

            BTN_NEW_SIM => {
                self.canvas
                    .canvas
                    .set_dialog(Box::new(MultiTextEntryDlg::new(
                        DLG_ENTER_SPECS,
                        "Enter simulation specs",
                        [
                            TextEntryField::new(
                                "rule",
                                "Rule (0 to 15)",
                                self.world.rule.number().to_string(),
                            ),
                            TextEntryField::new(
                                "bitsnum",
                                "Bits number",
                                self.world.bits.len().to_string(),
                            ),
                            //TextEntryField::new("onesnum", "Ones number", "500"),
                            TextEntryField::new(
                                "onesnum",
                                "Ones number",
                                //self.world.init_ones.to_string(),
                                //self.world.init_ones.to_string(),
                                //self.world.bits.ones_count().to_string(),
                                inits::INITIAL_ONES.to_string(),
                            ),
                        ],
                    )));
            }

            //{
            _ => {}
        }
    }

    // TDJ: Not needed for this app
    // fn handle_drag_float(&mut self, id: DragFloatId, value: f32) {
    //     match id {
    //         // DRAGFLOAT_GAUGE => {
    //         //     self.world.gauge.set_pointer(value.into());
    //         // }
    //         _ => {}
    //     }
    // }

    fn handle_radio_boxes(&mut self, id: RadioBoxesDlgId, value: i32) {
        match id {
            DLG_SIM_STATE => {
                match value {
                    // CHOICE_RUN => {
                    //     self.sim_timer.set_normal_speed();
                    //     self.sim_timer.run();
                    // }
                    CHOICE_RUN => {
                        self.sim_timer.set_to_run_normal_speed();
                    }
                    CHOICE_PAUSE => {
                        self.sim_timer.pause();
                    }
                    // CHOICE_FAST => {
                    //     self.sim_timer.set_fast_forward();
                    //     self.sim_timer.run();
                    // }
                    CHOICE_FAST => {
                        self.sim_timer.set_to_run_fast_forward();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn handle_multi_text_entry(&mut self, id: MultiTextEntryDlgId, values: Vec<(String, String)>) {
        // let mut rule: u8 = self.world.rule.number();
        // let mut bits: usize = self.world.bits.len();
        // let mut ones: usize = self.world.bits.ones_count();
        //
        // self.sim_timer.pause();

        match id {
            DLG_ENTER_SPECS => {
                self.sim_timer.pause();
                let mut bad_val = false;
                let mut rule = self.world.rule.number();
                let mut bits = self.world.bits.len();
                //println!("Orig bits = {bits}");
                let mut ones = self.world.bits.ones_count();

                for item in values {
                    let (item_id, text) = item;
                    // match item_id.as_str() {
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
                        "bitsnum" => match text.trim().parse::<usize>() {
                            Ok(number) if number >= 2 => {
                                bits = number;
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
                        "onesnum" => match text.trim().parse::<usize>() {
                            Ok(number) if number <= bits => {
                                ones = number;
                            }
                            Ok(number) => {
                                bad_val = true;
                                eprintln!(
                                    "Invalid ones number: {number}. \
                                    Ones number must be smaller than bits number.\
                                    Wiil be set to number of bits."
                                );
                                //ones = bits;
                            }
                            Err(err) => {
                                bad_val = true;
                                eprintln!("Could not parse ones number {:?}: {err}", text);
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
                    println!("Rule = {rule} = {}", self.world.rule.number());
                    self.world.bits =
                        BitArray::new_with_random_ones(bits, ones, &mut self.world.rng);
                    println!("Bits = {bits} = {}", self.world.bits.len());
                    println!("Ones = {ones} = {}", self.world.bits.ones_count());
                    self.world.frame_number = 0;
                }

                // self.world.rule = Rule::new(rule);
                // println!("Rule = {rule} = {}", self.world.rule.number());
                //
                // if ones > bits {
                //     ones = bits;
                // }
                // println!("Ones = {ones}");
                //
                // self.world.bits = BitArray::new_with_random_ones(bits, ones, &mut self.world.rng);
                // println!("Bits = {bits} = {}", self.world.bits.len());
            }
            _ => {}
        }

        // self.world.rule = Rule::new(rule);
        // println!("Rule = {rule} = {}", self.world.rule.number());
        //
        // if ones > bits {
        //     ones = bits;
        // }
        // println!("Ones = {ones}");

        // fn handle_multi_text_entry(&mut self, id: MultiTextEntryDlgId, values: Vec<(String, String)>) {
        //     // let mut rule: u8 = 0;
        //     // let mut bits: usize = 50000;
        //     // let mut ones: usize = 500;
        //
        //     let mut rule: u8 = self.world.rule.number();
        //     let mut bits: usize = self.world.bits.len();
        //     //let mut ones: usize = self.world.init_ones;
        //     let mut ones: usize = self.world.bits.ones_count();
        //
        //     self.sim_timer.pause();
        //
        //     match id {
        //         DLG_ENTER_SPECS => {
        //             for item in values {
        //                 let (item_id, text) = item;
        //                 // match item_id.as_str() {
        //                 match item_id.as_str() {
        //                     "rule" => match text.trim().parse::<u8>() {
        //                         Ok(number) if number < 16 => {
        //                             rule = number;
        //                         }
        //                         Ok(number) => {
        //                             eprintln!(
        //                                 "Invalid rule number: {number}. Rule must be between 0 and 15."
        //                             );
        //                         }
        //                         Err(err) => {
        //                             eprintln!("Could not parse rule number {:?}: {err}", text);
        //                         }
        //                     },
        //                     "bitsnum" => match text.trim().parse::<usize>() {
        //                         Ok(number) if number >= 2 => {
        //                             bits = number;
        //                         }
        //                         Ok(number) => {
        //                             eprintln!("Invalid bits number: {number}. Bits number too small.");
        //                         }
        //                         Err(err) => {
        //                             eprintln!("Could not parse bits number {:?}: {err}", text);
        //                         }
        //                     },
        //                     "onesnum" => match text.trim().parse::<usize>() {
        //                         Ok(number) if number <= bits => {
        //                             ones = number;
        //                         }
        //                          Ok(number) => {
        //                             eprintln!(
        //                                 "Invalid ones number: {number}. \
        //                                 Ones number must be smaller than bits number.\
        //                                 Wiil be set to number of bits."
        //                             );
        //                             ones = bits;
        //                         }
        //                         Err(err) => {
        //                             eprintln!("Could not parse ones number {:?}: {err}", text);
        //                         }
        //                     },
        //
        //                     _ => {}
        //                 }
        //             }
        //         }
        //         _ => {}
        //     }
        //
        //     self.world.rule = Rule::new(rule);
        //     println!("Rule = {rule} = {}", self.world.rule.number());
        //
        //     if ones > bits {
        //         ones = bits;
        //     }
        //     println!("Ones = {ones}");

        //self.world.init_ones = ones;
        //self.world.init_ones = ones; TDJ: ones count is now in the BitArray struct
        //self.world.bits = BitArray::new_with_initial_ones(bits, ones);
        // self.world.bits = BitArray::new_with_random_ones(bits, ones, &mut self.world.rng);
        // println!("Bits = {bits} = {}", self.world.bits.len());
    }

    // TJD: Not needed for this app
    //Handle drag float dialog messages
    //Requires application specific customization.
    // fn handle_drag_float_dlg(&mut self, id: DragFloatDlgId, val: f32) {
    //     match id {
    //         // DLG_ENTER_VALUE => {
    //         //     self.world.value = val as f64;
    //         //}
    //         _ => {}
    //     }
    // }
} // end impl TheApp
