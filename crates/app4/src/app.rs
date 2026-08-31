//! ## Application. struct TheApp is the main structure and entry point of the application.
//! - Contains a `Canvas` for holding a collection of shapes.
//! - Provides methods for creating and updating the UI.
//! - All method handing methods in this module need application specific customizations.
//! - Contains a 'World" which contains all non-gui program data and logic. .

// app1.rs

mod app_internal; // internal functions that do not require application specific customizations

use crate::inits;
use egui::Context;
#[allow(unused_imports)]
use gui_lib::{
    ButtonId, DialogId, DragFloatDlg, DragFloatDlgId, DragFloatId, MessageBoxDlg,
    MultiTextEntryDlg, MultiTextEntryDlgId, NilDlg, RadioBoxesDlg, RadioBoxesDlgId,
    RadioBoxesField, SimTimer, SliderId, TextEntryDlg, TextEntryDlgId, TextEntryField, WidgetMsg,
    World, app_gl,
};
use statrs::statistics::Statistics;
use std::fs;
use rand::Rng;
use crate::canvas::TheCanvas;
use crate::ids::*;
use crate::world::emerge::BitArray;
use crate::world::{Rule, TheWorld};
use crate::world::delta_ones::Deltas;
//use crate::world::delta_ones::DeltaOnes;

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
            // TDJ: Not needed for this app
            // WidgetMsg::DragFloatChanged(id, value) => {
            //     self.handle_drag_float(id, value);
            // }
            WidgetMsg::DialogAcceptedRadioBoxes(id, value) => {
                self.handle_radio_boxes(id, value);
            }
            WidgetMsg::DialogAcceptedMultiTextEntry(id, values) => {
                self.handle_multi_text_entry(id, values);
            }
            WidgetMsg::DialogAcceptedText(id, text) => {
                self.handle_text_entry(id, text);
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
                    Delta ones.",
                )));
            }

            BTN_SIM => {
                let current_choice =
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
                                self.world.start_ones.to_string(),
                            ),
                        ],
                    )));
            }

            BTN_BATCH => {
                println!("Batch size: {}", self.sim_timer.batch_size());
                self.canvas.canvas.set_dialog(Box::new(TextEntryDlg::new(
                    DLG_BATCH,
                    "Enter batch size",
                    "Batch size: ",
                    self.sim_timer.batch_size().to_string(),
                )));
            }

            // BTN_ZOOM => {
            //     let sgr = self.canvas.view_handles.dgr.borrow();
            //
            //     self.canvas
            //         .canvas
            //         .set_dialog(Box::new(MultiTextEntryDlg::new(
            //             DLG_ZOOM,
            //             "Enter Zoom specs",
            //             [
            //                 TextEntryField::new(
            //                     "scale",
            //                     "Scale (> 0)",
            //                     //sgr.zoom_scale().to_string(),
            //                 ),
            //                 TextEntryField::new(
            //                     "focus",
            //                     "Focus (0.0 to 1.0)",
            //                     //sgr.zoom_focus().to_string(),
            //                 ),
            //             ],
            //         )));
            // }

            BTN_SEQ => {
                let mut the_len = self.world.attractor.len();
                if the_len == 0 {
                    the_len = inits::INITIAL_SEQ_LENGTH;
                }

                self.canvas
                    .canvas
                    .set_dialog(Box::new(MultiTextEntryDlg::new(
                        DLG_SEQUENCE,
                        "Enter sequence specs",
                        [
                            TextEntryField::new(
                                "discard",
                                "Discard interactions",
                                self.world.attractor.discard.to_string(),
                                //inits::INITIAL_SEQ_DISCARD.to_string(),
                            ),
                            TextEntryField::new(
                                "seq_length",
                                "Sequence length",
                                //inits::INITIAL_SEQ_LENGTH.to_string(),
                                the_len.to_string(),
                            ),
                        ],
                    )));

                //let delta = DeltaOnes::new(Rule::new(5), 2900, 100, &mut self.world.rng);
                //println!("{}", delta.delta_stats_str());
                println!("Bits = {}", self.world.bits.len());
                let d = Deltas::new(Rule::new(5), 100, &mut self.world.rng);
                println!("Poss ones = {}", d.len());

            }

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
            DLG_SIM_STATE => match value {
                CHOICE_RUN => {
                    self.sim_timer.set_to_run_normal_speed();
                }
                CHOICE_PAUSE => {
                    self.sim_timer.pause();
                }
                CHOICE_FAST => {
                    self.sim_timer.set_to_run_fast_forward();
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_multi_text_entry(&mut self, id: MultiTextEntryDlgId, values: Vec<(String, String)>) {
        match id {
            DLG_ENTER_SPECS => {
                self.sim_timer.pause();

                let mut bad_val = false;
                let mut rule = self.world.rule.number();
                let mut bits = self.world.bits.len();
                let mut ones = self.world.start_ones;

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
                    self.world.bits =
                        BitArray::new_with_random_ones(bits, ones, &mut self.world.rng);
                    self.world.start_ones = ones;
                    self.world.frame_number = 0;
                }
            }

            // DLG_ZOOM => {
            //     let mut bad_val = false;
            //
            //     let mut sgr = self.canvas.view_handles.dgr.borrow_mut();
            //     for item in values {
            //         let (item_id, text) = item;
            //         match item_id.as_str() {
            //             "scale" => match text.trim().parse::<f32>() {
            //                 Ok(number) if number >= 0.0 => {
            //                     sgr.set_zoom_scale(number);
            //                 //     self.canvas
            //                 //         .view_handles
            //                 //         .stxt_scale
            //                 //         .borrow_mut()
            //                 //         .set_text(format!("Scale: {}", number));
            //                 }
            //                 Ok(number) => {
            //                     bad_val = true;
            //                     eprintln!(
            //                         "Invalid scale value: {number}. Scale must be greater than 0."
            //                     );
            //                 }
            //                 Err(err) => {
            //                     bad_val = true;
            //                     eprintln!("Could not parse scale value {:?}: {err}", text);
            //                 }
            //             },
            //             "focus" => match text.trim().parse::<f32>() {
            //                 Ok(number) if number >= 0.0 && number <= 1.0 => {
            //                     sgr.set_zoom_focus(number);
            //                     //self.canvas
            //                         // .view_handles
            //                         // .stxt_focus
            //                         // .borrow_mut()
            //                         // .set_text(format!("Focus: {}", number));
            //                 }
            //                 Ok(number) => {
            //                     bad_val = true;
            //                     eprintln!(
            //                         "Invalid focus value: {number}. Must be between 0 and 1."
            //                     );
            //                 }
            //                 Err(err) => {
            //                     bad_val = true;
            //                     eprintln!("Could not parse focus value {:?}: {err}", text);
            //                 }
            //             },
            //             _ => {}
            //         }
            //     }
            //
            //     if bad_val {
            //         self.canvas.canvas.set_dialog(Box::new(MessageBoxDlg::new(
            //             DLG_BAD_VALS,
            //             "Error Message",
            //             "Bad value(s) entered.",
            //         )));
            //     }
            // }

            DLG_SEQUENCE => {
                let mut discard: usize = 0;
                let mut seq_len: usize = 0;
                let mut bad_val = false;

                for item in values {
                    let (item_id, text) = item;
                    match item_id.as_str() {
                        "discard" => match text.trim().parse::<usize>() {
                            Ok(number) => {
                                discard = number;
                            }
                            Err(err) => {
                                bad_val = true;
                                eprintln!("Could not parse discard number {:?}: {err}", text);
                            }
                        },
                        "seq_length" => match text.trim().parse::<usize>() {
                            Ok(number) => {
                                seq_len = number;
                            }
                            Err(err) => {
                                bad_val = true;
                                eprintln!(
                                    "Could not parse sequence length number {:?}: {err}",
                                    text
                                );
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
                    // Discard the first 'discard' interactions.
                    for _ in 0..discard {
                        self.world.advance();
                    }
                    self.world.attractor.discard = discard;

                    // Push 'seq_len' ones counts to the empty attractor sequence.
                    self.world.attractor.seq.clear();
                    self.world.attractor.seq.reserve(seq_len);
                    for _ in 0..seq_len {
                        self.world
                            .attractor
                            .seq
                            .push(self.world.bits.ones_count().try_into().unwrap());
                        self.world.advance();
                    }

                    // println!("Discard length: {}", self.world.attractor.discard);
                    // //println!("Sequence length: {}", self.world.attractor.seq.len());
                    // println!("Sequence length: {}", self.world.attractor.len());
                    // for val in self.world.attractor.seq.iter().take(10) {
                    //     println!("{}", val);
                    // }

                    let numbers = vec![2, 4, 6, 8, 10];
                    let mut mean_test = 0.0;
                    if numbers.len() > 0 {
                        mean_test = numbers.iter().map(|&value| value as f64).sum::<f64>()
                            / numbers.len() as f64;
                    }
                    println!("Mean_test: {}", mean_test);

                    let mut mean = 0.0;
                    if self.world.attractor.seq.len() > 0 {
                        mean = self
                            .world
                            .attractor
                            .seq
                            .iter()
                            .map(|&value| value as f64)
                            .sum::<f64>()
                            / self.world.attractor.seq.len() as f64;
                    }
                    println!("Mean: {}", mean);

                    let values_f64: Vec<f64> =
                        self.world.attractor.seq.iter().map(|&x| x as f64).collect();

                    let data = values_f64.as_slice();

                    println!("Count: {}", data.len());
                    println!("Mean: {}", data.mean());
                    println!("Minimum: {}", data.min());
                    println!("Maximum: {}", data.max());
                    println!("Variance: {}", data.variance());
                    println!("Standard deviation: {}", data.std_dev());

                    // let values_f64: Vec<f64> =
                    //     self.world.attractor.seq.iter().map(|&x| x as f64).collect();
                    //
                    // let data = values_f64.as_slice();
                    //
                    // println!("Count: {}", data.len());
                    // println!("Mean: {}", data.mean());
                    // println!("Minimum: {}", data.min());
                    // println!("Maximum: {}", data.max());
                    // println!("Variance: {}", data.variance());
                    // println!("Standard deviation: {}", data.std_dev());

                    fs::write(
                        "sequence.txt",
                        self.world
                            .attractor
                            .seq
                            .iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join("\n"),
                    )
                    .expect("Unable to write to file");

                    self.canvas.update(&self.world);
                }
            }

            _ => {}
        }
    }

    // for git

    fn handle_text_entry(&mut self, id: TextEntryDlgId, text: String) {
        match id {
            DLG_BATCH => {
                match text.trim().parse::<u32>() {
                    Ok(number) => {
                        self.sim_timer.set_batch_size(number);
                        // self.canvas
                        //     .view_handles
                        //     .stxt_batch
                        //     .borrow_mut()
                        //     .set_text(format!("Batch: {}", number));
                    }
                    Err(err) => {
                        self.canvas.canvas.set_dialog(Box::new(MessageBoxDlg::new(
                            DLG_BAD_BATCH,
                            "Error Message",
                            err.to_string(),
                            //"Bad value(s) entered.",
                        )));
                    }
                }
            }

            _ => {}
        }
    }
} // end impl TheApp
