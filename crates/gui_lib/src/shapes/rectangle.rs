// rectangle.rs

use crate::egui::{self, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Vec2};
use crate::shapes::base::{Shape, ShapeBase};

/// For Rectangle, `base.location` is the top-left corner.
#[derive(Debug, Default)]
pub struct Rectangle {
    base: ShapeBase,
    pub size: Vec2,
}
impl Rectangle {
    pub fn new(top_left: Pos2, size: Vec2) -> Self {
        Self::new_from_top_left(top_left, size)
    }
    pub fn new_from_top_left(top_left: Pos2, size: Vec2) -> Self {
        Self {
            base: ShapeBase {
                location: top_left,
                ..Default::default()
            },
            size,
        }
    }

    pub fn new_from_center(center: Pos2, size: Vec2) -> Self {
        let top_left = Pos2::new(center.x - size.x / 2.0, center.y - size.y / 2.0);
        Self::new_from_top_left(top_left, size)
    }
}

impl Shape for Rectangle {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        let rect = Rect::from_min_size(self.base.location() + canvas_offset, self.size);
        painter.rect(
            rect,
            CornerRadius::ZERO,
            self.base.fill_color(),
            Stroke::new(self.base.line_width(), self.base.color()), // border
            StrokeKind::Outside,                                    // Outside / Inside / Middle
        );
    }
}
