// line.rs

use crate::Circle;
use crate::egui::{self, Pos2, Vec2};
use crate::shapes::base::{Shape, ShapeBase};

#[derive(Debug, Default)]
pub struct Line {
    base: ShapeBase,
    pub vctr: Vec2,
}

impl Line {
    pub fn new(start: Pos2, vctr: Vec2) -> Self {
        Self::new_from_vector(start, vctr)
    }

    pub fn new_from_vector(start: Pos2, vctr: Vec2) -> Self {
        Self {
            base: ShapeBase {
                location: start,
                ..Default::default()
            },
            vctr,
        }
    }

    pub fn new_from_points(start: Pos2, end: Pos2) -> Self {
        Self::new_from_vector(start, end - start)
    }

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
        self.vctr = self.vctr.normalized() * length;
    }

    pub fn angle(&self) -> f32 {
        self.vctr.angle()
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.vctr = Vec2::angled(angle) * self.vctr.length();
    }
}

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

        painter.line_segment(
            [start, end],
            egui::Stroke::new(self.base.line_width(), self.base.color()),
        );
    }
}

// In
// egui, angles for rotation (e.g., in images) are typically represented in radians, with positive values indicating clockwise rotation. You can calculate the angle of a 2D vector using the .angle() method on a Vec2. For rotating images, use ImageOptions to specify the angle (in radians) and the rotation origin.
// Here are the key details for handling angles in egui:
//
// Radians: All rotation functions use radians, not degrees.
// Direction: A positive angle rotates clockwise.
// Vector Angle: egui::Vec2 has an .angle() method that returns the angle of the vector.
// Image Rotation: Use egui::Image::new(...).rotate(angle, origin) to rotate images. origin is a Vec2 in normalized UV space (e.g., Vec2::splat(0.5) for the center).
// egui_probe: In egui_probe, you can use #[egui_probe(as angle)] to render a float as an angle.
//
// Example usage (Image Rotation):
// rust
//
// ui.add(
// egui::Image::new(texture_id)
// .rotate(std::f32::consts::TAU / 4.0, egui::Vec2::splat(0.5)) // 90 deg clockwise
// );
