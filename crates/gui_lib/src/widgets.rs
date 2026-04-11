//! ## Module widgets contains the [`Widget`] trait and various widgets.
//!
// src/gui_lib/widgets.rs

use crate::Color32;
use crate::egui::RichText;
//use crate::ids::{ButtonId, DragFloatId, SliderId, WidgetMsg};
use crate::ids_gl::{ButtonId, DragFloatId, SliderId};
use crate::WidgetMsg;

/// Trait for invoking any widget in the UI.
/// Outputs a vector of [`WidgetMsg`]s.
pub trait Widget: std::fmt::Debug {
    fn invoke(&mut self, ui: &mut egui::Ui, out: &mut Vec<WidgetMsg>);
}

// ============================================================
// Simple widgets (no messages)
// ============================================================

/// Creates space between widgets
#[derive(Debug, Default)]
pub struct Space {
    pub size: f32,
}

impl Space {
    pub fn new(size: f32) -> Self {
        Self { size }
    }
}

impl Widget for Space {
    fn invoke(&mut self, ui: &mut egui::Ui, _out: &mut Vec<WidgetMsg>) {
        ui.add_space(self.size);
    }
}

/// Creatws a line separating widgets
#[derive(Debug, Default)]
pub struct Separator;

impl Separator {
    pub fn new() -> Self {
        Self
    }
}
impl Widget for Separator {
    fn invoke(&mut self, ui: &mut egui::Ui, _out: &mut Vec<WidgetMsg>) {
        ui.separator();
    }
}

/// Text output widget
#[derive(Debug, Default)]
pub struct Label {
    pub text: String,
    pub color: Color32,
    pub size: f32,
}

impl Label {
    pub fn new(text: impl Into<String>, color: Color32, size: f32) -> Self {
        Self {
            text: text.into(),
            color,
            size,
        }
    }
}

impl Widget for Label {
    fn invoke(&mut self, ui: &mut egui::Ui, _out: &mut Vec<WidgetMsg>) {
        ui.label(RichText::new(&self.text).color(self.color).size(self.size));
    }
}

// ============================================================
// Button
// ============================================================

/// A customizable button widget.
/// Emits WidgetMsg::ButtonClicked(self.id)
#[derive(Debug, Default)]
pub struct Button {
    pub id: ButtonId,
    pub label: String,
    pub width: f32,
    pub height: f32,
}

impl Button {
    pub fn new(id: ButtonId, label: impl Into<String>, width: f32, height: f32) -> Self {
        Self {
            id,
            label: label.into(),
            width,
            height,
        }
    }
}

impl Widget for Button {
    fn invoke(&mut self, ui: &mut egui::Ui, out: &mut Vec<WidgetMsg>) {
        let resp = ui.add_sized(
            egui::vec2(self.width, self.height),
            egui::Button::new(RichText::new(&self.label).size(14.0).strong()),
        );

        if resp.clicked() {
            out.push(WidgetMsg::ButtonClicked(self.id));
        }
    }
}

// ============================================================
// Slider
// ============================================================
/// A customizable DragFloat widget.
/// Depreciated
#[derive(Debug)]
pub struct Slider {
    id: SliderId,
    label: String,
    value: f32,
    range: std::ops::RangeInclusive<f32>,
}
impl Slider {
    pub fn new(
        id: SliderId,
        label: impl Into<String>,
        value: f32,
        range: std::ops::RangeInclusive<f32>,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            value,
            range,
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

impl Widget for Slider {
    fn invoke(&mut self, ui: &mut egui::Ui, out: &mut Vec<WidgetMsg>) {
        let resp = ui.add(egui::Slider::new(&mut self.value, self.range.clone()).text(&self.label));

        if resp.changed() {
            out.push(WidgetMsg::SliderChanged(self.id, self.value));
        }
    }
}

// ============================================================
// DragFloat
// ============================================================

/// A customizable DragFloat widget.
/// Allows user input of floating point value.
/// Emits WidgetMsg::DragFloatChanged(self.id, self.value)
#[derive(Debug)]
pub struct DragFloat {
    id: DragFloatId,
    label: String,
    value: f32,
    range: std::ops::RangeInclusive<f32>,
    decimal: usize,
    //speed: f64,
    speed: f32,
}
impl DragFloat {
    pub fn new(
        id: DragFloatId,
        label: impl Into<String>,
        value: f32,
        range: std::ops::RangeInclusive<f32>,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            value,
            range,
            decimal: 0,
            speed: 1.0,
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
    pub fn set_decimal(&mut self, decimal: usize) {
        self.decimal = decimal;
    }
    //pub fn set_speed(&mut self, speed: f64) {
    //     self.speed = speed;
    // }
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }
}

impl Widget for DragFloat {
    fn invoke(&mut self, ui: &mut egui::Ui, out: &mut Vec<WidgetMsg>) {
        let resp = ui.add(
            egui::DragValue::new(&mut self.value)
                .range(self.range.clone())
                .prefix(&self.label)
                .fixed_decimals(self.decimal)
                .speed(self.speed),
        );

        if resp.changed() {
            out.push(WidgetMsg::DragFloatChanged(self.id, self.value));
        }
    }
}
