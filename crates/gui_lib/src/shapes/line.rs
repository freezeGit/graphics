//! ## Module line
//! Contains struct Line

// line.rs

use crate::egui::{self, Pos2, Vec2,};
use crate::shapes::base::{Shape, ShapeBase,};
use crate::{LineStyle};

/// Struct Line
///
/// A line segment with a start point and a vector. 
#[derive(Debug, Default)]
pub struct Line {
    base: ShapeBase,
    vctr: Vec2,
}

impl Line {
    /// Create a new Line.
    ///
    /// # Arguments
    /// * `start` - The start point
    /// * `vctr` - The vector
    pub fn new(start: Pos2, vctr: Vec2) -> Self {
        Self::new_from_vector(start, vctr)
    }

    /// Create a new Line from start point and vector.
    pub fn new_from_vector(start: Pos2, vctr: Vec2) -> Self {
        Self {
            base: ShapeBase {
                location: start,
                ..Default::default()
            },
            vctr,
        }
    }

    /// Create a new Line from start point and end point.
    pub fn new_from_points(start: Pos2, end: Pos2) -> Self {
        Self::new_from_vector(start, end - start)
    }

    /// Create a new Line from start point and angle and length.
    pub fn new_from_angle(start: Pos2, angle: f32, length: f32) -> Self {
        Self::new_from_vector(start, Vec2::angled(angle) * length)
    }

    pub fn vector(&self) -> Vec2 {
        self.vctr
    }
    pub fn set_vector(&mut self, vector: Vec2) {
        self.vctr = vector;
    }
    pub fn length(&self) -> f32 {
        self.vctr.length()
    }
    pub fn set_length(&mut self, length: f32) {
        if self.vctr.length_sq() > 0.0 {
            self.vctr = self.vctr.normalized() * length;
        }
    }
    pub fn angle(&self) -> f32 {
        self.vctr.angle()
    }
    pub fn set_angle(&mut self, angle: f32) {
        self.vctr = Vec2::angled(angle) * self.vctr.length();
    }
}

/// Implement trait Shape for Line.
///
/// Make trait [`Shape`] methods available.
impl Shape for Line {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        let start = self.base.location() + canvas_offset;
        let end = start + self.vctr;
        let stroke = egui::Stroke::new(self.base.line_width(), self.base.color());

        match self.base.line_style() {
            LineStyle::Solid => {
                painter.line_segment([start, end], stroke);
            },
            LineStyle::Dashed => {
                let shapes = egui::Shape::dashed_line(
                    &[start, end],
                    stroke,
                    self.base.dash_length(),
                    self.base.dash_gap(),
                );
                painter.extend(shapes);
            },
            LineStyle::Dotted => {
                let shapes = egui::Shape::dotted_line(
                    &[start, end],
                    self.base.color(),
                    self.base.dot_spacing(),
                    self.base.dot_radius(),
                );
                painter.extend(shapes);
            },
        }
    }
} // impl Shape for Line
