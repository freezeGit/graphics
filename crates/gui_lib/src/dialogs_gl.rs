//! ## Module dialogs contains the Dialog trait and the various dialog types.
//!
// dialogs_gl

use crate::egui;
use crate::ids_gl::{
    DragFloatDlgId, MessageBoxDlgId, MultiTextEntryDlgId, TextEntryDlgId, RadioBoxesDlgId,
};
use crate::messages_gl::WidgetMsg;

// -----------------------------
/// Trait for all dialogs.
pub trait Dialog: std::fmt::Debug {
    /// Displays the dialog and pushes its [`WidgetMsg>`] into `out`.
    ///
    /// Returns true when the dialog is closed.
    fn invoke_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool;
}

// ---------- NilDlg -----------------
/// Does nothing.
#[derive(Debug)]
pub struct NilDlg;

impl Dialog for NilDlg {
    fn invoke_modal(&mut self, _ctx: &egui::Context, _out: &mut Vec<WidgetMsg>) -> bool {
        true // Nothing to open so already closed
    }
}

// ----------- MessageBoxDlg ------------------
/// Displays a message box with a title and text.
/// Does not emit a message.
#[derive(Debug)]
pub struct MessageBoxDlg {
    egui_id: egui::Id,
    title: String,
    title_size: f32,
    text: String,
    font_size: f32,
}

impl MessageBoxDlg {
    const DEFAULT_TITLE_SIZE: f32 = 22.0; // 20.0
    const DEFAULT_FONT_SIZE: f32 = 18.0; // 16.0

    pub fn new(id: MessageBoxDlgId, title: impl Into<String>, text: impl Into<String>) -> Self {
        Self::new_sized(
            id,
            title,
            Self::DEFAULT_TITLE_SIZE,
            text,
            Self::DEFAULT_FONT_SIZE,
        )
    }

    pub fn new_sized(
        id: MessageBoxDlgId,
        title: impl Into<String>,
        title_size: f32,
        text: impl Into<String>,
        font_size: f32,
    ) -> Self {
        Self {
            egui_id: egui::Id::new(("message_box_dialog", id)),
            title: title.into(),
            title_size,
            text: text.into(),
            font_size,
        }
    }
} // end of MessageBoxDlg

impl Dialog for MessageBoxDlg {
    fn invoke_modal(&mut self, ctx: &egui::Context, _out: &mut Vec<WidgetMsg>) -> bool {
        const DEFAULT_MN_WIDTH: f32 = 320.0; // Default minimum width for message box

        let mut close = false;

        egui::Modal::new(self.egui_id).show(ctx, |ui| {
            ui.set_min_width(DEFAULT_MN_WIDTH);

            ui.heading(egui::RichText::new(&self.title).size(self.title_size));
            ui.separator();

            ui.label(egui::RichText::new(&self.text).size(self.font_size));
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    close = true;
                }
            });
        });

        close
    }
} // end of impl Dialog for MessageBoxDlg

// ---------------- TextEntryDlg ----------------------
/// Displays a dialog with a title, prompt, and text entry field.
/// Outputs the text entered by the user as a String associated with the emitted message.
/// Emits WidgetMsg::DialogAcceptedText(TextEntryDlgId, String).
#[derive(Debug)]
pub struct TextEntryDlg {
    egui_id: egui::Id,
    id: TextEntryDlgId,
    title: String,
    prompt: String,
    text: String,
}

impl TextEntryDlg {
    pub fn new(
        id: TextEntryDlgId,
        title: impl Into<String>,
        prompt: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            egui_id: egui::Id::new(("text_entry_dialog", id)),
            id,
            title: title.into(),
            prompt: prompt.into(),
            text: text.into(),
        }
    }
} // end of TextEntryDlg

impl Dialog for TextEntryDlg {
    fn invoke_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool {
        const DEFAULT_TED_SIZE: f32 = 20.0; // Default font size for text entry dialog
        const DEFAULT_TE_WIDTH: f32 = 320.0; // Default minimum width for text entry dialog

        let mut close = false;

        egui::Modal::new(self.egui_id).show(ctx, |ui| {
            ui.set_min_width(DEFAULT_TE_WIDTH);

            ui.heading(egui::RichText::new(&self.title).size(DEFAULT_TED_SIZE));
            ui.separator();

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("{}:", self.prompt)).size(DEFAULT_TED_SIZE));
                ui.add(egui::TextEdit::singleline(&mut self.text).font(egui::TextStyle::Heading));
            });

            ui.add_space(15.0);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    out.push(WidgetMsg::DialogAcceptedText(self.id, self.text.clone()));
                    close = true;
                }
                if ui.button("Cancel").clicked() {
                    close = true;
                }
            });
        });

        close
    }
} // end of impl Dialog for TextEntryDlg

// ------------ DragFloatDlg ------------------------------
/// Displays a dialog with a title and floating point value entry field.
/// Outputs the value entered by the user.
/// Emits WidgetMsg::DialogAcceptedDragFloat(DragFloatId, f32).
#[derive(Debug)]
pub struct DragFloatDlg {
    egui_id: egui::Id,
    id: DragFloatDlgId,
    title: String,
    //prompt: String,
    value: f32,
    decimal: usize,
    speed: f64,
}

impl DragFloatDlg {
    pub fn new(id: DragFloatDlgId, title: impl Into<String>, value: f32) -> Self {
        Self {
            egui_id: egui::Id::new(("text_entry_dialog", id)),
            id,
            title: title.into(),
            value,
            decimal: 0,
            speed: 1.0,
        }
    }

    pub fn set_decimal(&mut self, decimal: usize) {
        self.decimal = decimal;
    }
    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }
} // end of DragFloatDlg

impl Dialog for DragFloatDlg {
    fn invoke_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool {
        const DEFAULT_DF_SIZE: f32 = 20.0; // Default font size for drag float dialog
        const DEFAULT_DF_WIDTH: f32 = 320.0; // Default minimum width for drag float dialog

        let mut close = false;

        egui::Modal::new(self.egui_id).show(ctx, |ui| {
            ui.set_min_width(DEFAULT_DF_WIDTH);

            ui.heading(egui::RichText::new(&self.title).size(DEFAULT_DF_SIZE));
            ui.separator();
            ui.add_space(10.0);
            ui.add(
                egui::DragValue::new(&mut self.value)
                    .fixed_decimals(self.decimal)
                    .speed(self.speed),
            );

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    out.push(WidgetMsg::DialogAcceptedDragFloat(self.id, self.value));
                    close = true;
                }
                if ui.button("Cancel").clicked() {
                    close = true;
                }
            });
        });

        close
    }
} // end of impl Dialog for DragFloatDlg

// ------------ MultiTextEntryDlg ------------------------------
/// An array of TextEntryField's is used to construct a MultiTextEntryDlg.
#[derive(Debug, Clone)]
pub struct TextEntryField {
    pub id: String,
    pub prompt: String,
    pub text: String,
}

impl TextEntryField {
    pub fn new(id: impl Into<String>, prompt: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            prompt: prompt.into(),
            text: text.into(),
        }
    }
}

/// Displays a dialog with a title,
/// and a number of prompt, text entry fields.
/// Outputs the texts entered by the user as a Vec<(String, String)
/// (associated with the emitted message).
/// The first value in the tuple is the id of the field, and the second is the text entered.
/// Emits WidgetMsg::DialogAcceptedText(MultiTextEntryDlgId, Vec<(String, String)>).
#[derive(Debug)]
pub struct MultiTextEntryDlg {
    pub id: MultiTextEntryDlgId,
    pub egui_id: egui::Id,
    pub title: String,
    pub fields: Vec<TextEntryField>,
}

impl MultiTextEntryDlg {
    pub fn new<I>(id: MultiTextEntryDlgId, title: impl Into<String>, fields: I) -> Self
    where
        I: IntoIterator<Item = TextEntryField>,
    {
        Self {
            id,
            //egui_id: egui::Id::new(("person_dlg", id)),
            egui_id: egui::Id::new(("multi_text_entry_dlg", id)),
            title: title.into(),
            fields: fields.into_iter().collect(),
        }
    }
} // end of MultiTextEntryDlg

impl Dialog for MultiTextEntryDlg {
    fn invoke_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool {
        let mut close = false;
        const DEFAULT_SIZE: f32 = 20.0;

        egui::Modal::new(self.egui_id).show(ctx, |ui| {
            ui.set_min_width(350.0);

            ui.heading(egui::RichText::new(&self.title).size(DEFAULT_SIZE));
            ui.separator();

            for f in &mut self.fields {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("{}:", f.prompt)).size(DEFAULT_SIZE));
                    ui.add(egui::TextEdit::singleline(&mut f.text).font(egui::TextStyle::Heading));
                });
            }

            ui.add_space(15.0);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    let values = self
                        .fields
                        .iter()
                        .map(|f| (f.id.clone(), f.text.clone()))
                        .collect();

                    out.push(WidgetMsg::DialogAcceptedMultiTextEntry(self.id, values));
                    close = true;
                }
                if ui.button("Cancel").clicked() {
                    close = true;
                }
            });
        });

        close
    }
} // end of impl Dialog for MultiTextEntryDlg

// ---------------------------------------------------

// ------------ RadioBoxesDlg ------------------------------
/// An array of TextEntryField's is used to construct a MultiTextEntryDlg.
#[derive(Debug, Clone)]
pub struct RadioBoxesField {
    //pub id: String,
    pub selected: String,
    pub alternative: String,
    pub label: String,
}

impl RadioBoxesField {
    pub fn new(selected: impl Into<String>, alternative: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            selected: selected.into(),
            alternative: alternative.into(),
            label: label.into(),
        }
    }
}

// /// Displays a dialog with a title,
// /// and a number of prompt, text entry fields.
// /// Outputs the texts entered by the user as a Vec<(String, String)
// /// (associated with the emitted message).
// /// The first value in the tuple is the id of the field, and the second is the text entered.
// /// Emits WidgetMsg::DialogAcceptedText(MultiTextEntryDlgId, Vec<(String, String)>).
#[derive(Debug)]
pub struct RadioBoxesDlg {
    pub id: RadioBoxesDlgId,
    pub egui_id: egui::Id,
    pub title: String,
    pub fields: Vec<RadioBoxesField>,
}

impl RadioBoxesDlg {
    pub fn new<I>(id: RadioBoxesDlgId, title: impl Into<String>, fields: I) -> Self
    where
        I: IntoIterator<Item = RadioBoxesField>,
    {
        Self {
            id,
            egui_id: egui::Id::new(("radio_boxes_dlg", id)),
            title: title.into(),
            fields: fields.into_iter().collect(),
        }
    }
} // end of RadioBoxesDlg

impl Dialog for RadioBoxesDlg {
    fn invoke_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool {
        let mut close = false;
        const DEFAULT_SIZE: f32 = 20.0;

        egui::Modal::new(self.egui_id).show(ctx, |ui| {
            ui.set_min_width(350.0);

            ui.heading(egui::RichText::new(&self.title).size(DEFAULT_SIZE));
            ui.separator();

            // for f in &mut self.fields {
            //     ui.horizontal(|ui| {
            //         ui.label(egui::RichText::new(format!("{}:", f.prompt)).size(DEFAULT_SIZE));
            //         ui.add(egui::TextEdit::singleline(&mut f.text).font(egui::TextStyle::Heading));
            //     });
            // }

            ui.add_space(15.0);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    // let values = self
                    //     .fields
                    //     .iter()
                    //     .map(|f| (f.id.clone(), f.text.clone()))
                    //     .collect();
                    //
                    // out.push(WidgetMsg::DialogAcceptedMultiTextEntry(self.id, values));
                    out.push(WidgetMsg::DialogAcceptedRadioBoxes(self.id, "Output".to_string()));
                    close = true;
                }
                if ui.button("Cancel").clicked() {
                    close = true;
                }
            });
        });

        close
    }
} // end of impl Dialog for MultiTextEntryDlg

// impl Dialog for MultiTextEntryDlg {
//     fn invoke_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool {
//         let mut close = false;
//         const DEFAULT_SIZE: f32 = 20.0;
//
//         egui::Modal::new(self.egui_id).show(ctx, |ui| {
//             ui.set_min_width(350.0);
//
//             ui.heading(egui::RichText::new(&self.title).size(DEFAULT_SIZE));
//             ui.separator();
//
//             for f in &mut self.fields {
//                 ui.horizontal(|ui| {
//                     ui.label(egui::RichText::new(format!("{}:", f.prompt)).size(DEFAULT_SIZE));
//                     ui.add(egui::TextEdit::singleline(&mut f.text).font(egui::TextStyle::Heading));
//                 });
//             }
//
//             ui.add_space(15.0);
//             ui.horizontal(|ui| {
//                 if ui.button("OK").clicked() {
//                     let values = self
//                         .fields
//                         .iter()
//                         .map(|f| (f.id.clone(), f.text.clone()))
//                         .collect();
//
//                     out.push(WidgetMsg::DialogAcceptedMultiTextEntry(self.id, values));
//                     close = true;
//                 }
//                 if ui.button("Cancel").clicked() {
//                     close = true;
//                 }
//             });
//         });
//
//         close
//     }
// } // end of impl Dialog for MultiTextEntryDlg


