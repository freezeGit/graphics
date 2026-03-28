//! ## Module lines
//! Contains struct Lines

// Lines.rs
use crate::egui::{self, Pos2};
use crate::shapes::base::{Shape, ShapeBase};

/// Struct Lines
///
/// A collection of lines with same color, width, and line style.
#[derive(Debug, Default)]
pub struct Lines {
    base: ShapeBase,
    lines: Vec<[Pos2; 2]>,
}

impl Lines {
    pub fn new(location: Pos2, lines: Vec<[Pos2; 2]>) -> Self {
        Self {
            base: ShapeBase {
                location,
                ..Default::default()
            },
            lines,
        }
    }
} // impl Lines

/// Implement trait Shape for Lines.
///
/// Make trait [`Shape`] methods available.
impl Shape for Lines {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        let stroke = egui::Stroke::new(self.base.line_width(), self.base.color());
        let translation = self.base.location().to_vec2() + canvas_offset;

        for line in &self.lines {
            painter.line_segment([line[0] + translation, line[1] + translation], stroke);
        }
    }
}
// impl Shape for Lines
