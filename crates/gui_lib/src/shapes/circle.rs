// circle.rs

use crate::egui::{self, Pos2};
use crate::shapes::base::{Shape, ShapeBase};

/// A customizable Circle component.

#[derive(Debug, Default)]
pub struct Circle {
    base: ShapeBase,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Pos2, radius: f32) -> Self {
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
}

impl Shape for Circle {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        let center = self.base.location() + canvas_offset;

        painter.circle(
            center,
            self.radius,
            self.base.fill_color(),
            egui::Stroke::new(self.base.line_width(), self.base.color()),
        );
    }
}
