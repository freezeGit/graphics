// text.rs

//use crate::gui_lib::{Color32, Pos2, Shape, ShapeBase, TextFont}; //TDJ

use crate::gui_lib::egui::{self, Color32, Pos2};
use crate::gui_lib::shapes::base::{Shape, ShapeBase};
//use crate::gui_lib::TextFont;

#[derive(Debug)]
pub enum TextFont {
    Proportional,
    Monospace,
}

/// A customizable Text component.
#[derive(Debug)]
pub struct Text {
    base: ShapeBase,
    text: String,
    color: Color32,
    size: f32,
    font: TextFont,
}

impl Text {
    pub fn new(top_left: Pos2, text: impl Into<String>) -> Self {
        Self {
            base: {
                ShapeBase {
                    location: top_left,
                    ..Default::default()
                }
            },
            text: text.into(),
            color: Color32::BLACK,
            size: 24.0,
            font: TextFont::Proportional,
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn set_color(&mut self, color: Color32) {
        self.color = color;
    }

    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }

    pub fn set_font(&mut self, font: TextFont) {
        self.font = font;
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

        painter.text(
            tl,
            egui::Align2::LEFT_TOP,
            self.text.as_str(),
            font_id,
            self.color,
        );
    }
}
