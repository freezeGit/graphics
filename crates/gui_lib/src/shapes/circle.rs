//! ## module circle
//! Contains a  customizable Circle component.

// circle.rs

use crate::egui::{self, Pos2};
use crate::shapes::base::{Shape, ShapeBase};
use crate::{Color32, LineStyle};
use std::f32::consts::TAU;

/// A customizable Circle component.
#[derive(Debug, Default)]
pub struct Circle {
    base: ShapeBase,
    radius: f32,
}

impl Circle {
    pub fn new(center: Pos2, radius: f32) -> Self {
        Self::new_from_center(center, radius)
    }
    pub fn new_from_center(center: Pos2, radius: f32) -> Self {
        Self {
            base: ShapeBase {
                location: center,
                ..Default::default()
            },
            radius,
        }
    }

    pub fn new_from_top_left(tl: Pos2, radius: f32) -> Self {
        let center = Pos2::new(tl.x + radius, tl.y + radius);
        Self::new(center, radius)
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
    pub fn set_radius(&mut self, r: f32) {
        self.radius = r;
    }
    // --------- Private functions ---------
    fn draw_solid_circle(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        let center = self.base.location() + canvas_offset;

        painter.circle(
            center,
            self.radius,
            self.base.fill_color(),
            egui::Stroke::new(self.base.line_width(), self.base.color()),
        );
    }

    fn draw_broken_circle(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        let center = self.base.location() + canvas_offset;
        let translation = self.base.location().to_vec2() + canvas_offset;
        let stroke = egui::Stroke::new(self.base.line_width(), self.base.color());
        let segments = ((self.radius * 0.75) as usize).clamp(12, 128);
        let mut pts = Vec::with_capacity(segments + 1);

        painter.circle_filled(center, self.radius, self.base.fill_color());

        for i in 0..=segments {
            let a = i as f32 / segments as f32 * TAU;
            pts.push(Pos2::new(self.radius * a.cos(), self.radius * a.sin()));
        }
        let pts_trans: Vec<Pos2> = pts.iter().map(|p| *p + translation).collect();

        match self.base.line_style() {
            LineStyle::Dashed => {
                let shapes = egui::Shape::dashed_line(
                    &pts_trans,
                    stroke,
                    self.base.dash_length(),
                    self.base.dash_gap(),
                );
                painter.extend(shapes);
            },
            LineStyle::Dotted => {
                let shapes = egui::Shape::dotted_line(
                    &pts_trans,
                    self.base.color(),
                    self.base.dot_spacing(),
                    self.base.dot_radius(),
                );
                painter.extend(shapes);
            },
            LineStyle::Solid => {},
        }
    }
} // impl Circle

/// Implement trait Shape for Circle.
///
/// Make trait [`Shape`] methods available.
impl Shape for Circle {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        if self.base.line_style() == LineStyle::Solid {
            self.draw_solid_circle(painter, canvas_offset);
        } else {
            self.draw_broken_circle(painter, canvas_offset);
        }
    }
} // impl Shape
