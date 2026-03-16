//! ## module Text
//! Declation for struct Text:
//! A text with a specified location, size, and font.
//!
// text.rs

use crate::egui::{self, Color32, Pos2, TextStyle};
use crate::shapes::base::{Shape, ShapeBase};
use std::f32::consts::FRAC_PI_2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFont {
    Proportional,
    Monospace,
}

/// A customizable Text component.
/// For Text, `base.location` is the top-left anchor used with Align2::LEFT_TOP.
#[derive(Debug)]
pub struct Text {
    base: ShapeBase,
    text: String,
    size: f32,
    font: TextFont,
    place: egui::Align2,
    angle: f32, // Rotation angle in radians
}
impl Text {
    /// Construct Text
    pub fn new(top_left: Pos2, text: impl Into<String>) -> Self {
        Self::new_from_top_left(top_left, text)
    }

    pub fn new_from_top_left(top_left: Pos2, text: impl Into<String>) -> Self {
        Self::new_from_place(egui::Align2::LEFT_TOP, top_left, text)
    }

    pub fn new_from_center(center: Pos2, text: impl Into<String>) -> Self {
        Self::new_from_place(egui::Align2::CENTER_CENTER, center, text)
    }

    // Private so as to limit number of placement choices
    fn new_from_place(place: egui::Align2, location: Pos2, text: impl Into<String>) -> Self {
        Self {
            base: ShapeBase {
                location,
                ..Default::default()
            },
            text: text.into(),
            size: 24.0,
            font: TextFont::Proportional,
            place,
            angle: 0.0,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn color(&self) -> Color32 {
        self.base.color()
    }
    pub fn set_color(&mut self, color: Color32) {
        self.base_mut().set_color(color);
    }

    pub fn size(&self) -> f32 {
        self.size
    }
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }

    pub fn font(&self) -> TextFont {
        self.font
    }
    pub fn set_font(&mut self, font: TextFont) {
        self.font = font;
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }
    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }
    pub fn set_vertical(&mut self) {
        self.angle = -FRAC_PI_2;
    }
    pub fn set_horizontal(&mut self) {
        self.angle = 0.0;
    }
}

impl Shape for Text {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        let tl = self.base.location() + canvas_offset;
        let font_id = match self.font {
            TextFont::Proportional => egui::FontId::proportional(self.size),
            TextFont::Monospace => egui::FontId::monospace(self.size),
        };

        if self.angle != 0.0 {
            // rotate
            let galley = painter.layout_no_wrap(
                self.text.clone(),
                font_id,
                self.base.color(),
            );
            let mut shape = egui::Shape::galley(tl, galley, self.base.color);
            if let egui::Shape::Text(ref mut text_shape) = shape {
                text_shape.angle = self.angle;
            }
            painter.add(shape);
        } else {
            // do not rotate
            painter.text(
                tl,
                self.place,
                self.text.as_str(),
                font_id,
                self.base.color,
            );
        }
    }
}
