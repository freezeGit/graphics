// dialogs.rs

use crate::gui_lib::egui;
use crate::gui_lib::ids::{
    MessageBoxDlgId, TextEntryDlgId, DragFloatDlgId, WidgetMsg
};

pub trait Dialog: std::fmt::Debug {
    /// Returns true if it closed this frame.
    fn do_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool;
}

// -----------------------------
#[derive(Debug)]
pub struct MessageBoxDlg {
    egui_id: egui::Id,
    title: String,
    text: String,
}

impl MessageBoxDlg {
    pub fn new(
        id: MessageBoxDlgId, // TDJ id is used to create unique egui::Id. Is this necessary?
        title: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            egui_id: egui::Id::new(("message_box_dialog", id)),
            title: title.into(),
            text: text.into(),
        }
    }
}

impl Dialog for MessageBoxDlg {
    fn do_modal(&mut self, ctx: &egui::Context, _out: &mut Vec<WidgetMsg>) -> bool {
        let mut close = false;

        egui::Modal::new(self.egui_id).show(ctx, |ui| {
            ui.heading(&self.title);
            ui.separator();

            ui.label(&self.text);

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    close = true;
                }
            });
        });

        close
    }
}

// --------------------------------------
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
}

impl Dialog for TextEntryDlg {
    fn do_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool {
        let mut close = false;

        egui::Modal::new(self.egui_id).show(ctx, |ui| {
            ui.heading(&self.title);
            ui.separator();

            ui.label(&self.prompt);
            ui.text_edit_singleline(&mut self.text);

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    //out.push(WidgetMsg::DialogAcceptedText(self.id, self.text.clone()));
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
}

// ------------------------------------------
#[derive(Debug)]
pub struct DragFloatDlg {
    egui_id: egui::Id,
    id: DragFloatDlgId,
    title: String,
    prompt: String,
    value: f32,
    decimal: usize,
    speed: f64,
}

impl DragFloatDlg {
    pub fn new(
        id: DragFloatDlgId,
        title: impl Into<String>,
        prompt: impl Into<String>,
        value: f32,
    ) -> Self {
        Self {
            egui_id: egui::Id::new(("text_entry_dialog", id)),
            id,
            title: title.into(),
            prompt: prompt.into(),
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
}

impl Dialog for DragFloatDlg {
    fn do_modal(&mut self, ctx: &egui::Context, out: &mut Vec<WidgetMsg>) -> bool {
        let mut close = false;

        egui::Modal::new(self.egui_id).show(ctx, |ui| {
            ui.heading(&self.title);
            ui.separator();
            ui.label(&self.prompt);
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
}

// ------------------------------------------------

