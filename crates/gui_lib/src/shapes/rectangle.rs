//! ## module rectangle
//! Declation for struct Rectangle:
//! A rectangle with a specified size and location.
//!
// rectangle.rs

use crate::LineStyle;
use crate::egui::{self, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Vec2};
use crate::shapes::base::{Shape, ShapeBase};
use std::f32::consts::TAU;

/// For Rectangle, `base.location` is the top-left corner.
#[derive(Debug, Default)]
pub struct Rectangle {
    base: ShapeBase,
    pub size: Vec2,
}
impl Rectangle {
    /// Construct Rectangle.
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

    pub fn new_from_points(top_left: Pos2, bottom_right: Pos2) -> Rectangle {
        Rectangle::new_from_top_left(top_left, bottom_right - top_left)
    }

    /// Rectangle location is top-left corner.
    /// Rectangle size is width and height.
    /// Rectangle::move_to() moves the top-left corner.
    pub fn location(&self) -> Pos2 {
        self.base.location()
    }
    pub fn size(&self) -> Vec2 {
        self.size
    }
    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }
    pub fn width(&self) -> f32 {
        self.size.x
    }
    pub fn set_width(&mut self, w: f32) {
        self.size.x = w;
    }
    pub fn height(&self) -> f32 {
        self.size.y
    }
    pub fn set_height(&mut self, h: f32) {
        self.size.y = h;
    }
    pub fn center(&self) -> Pos2 {
        Pos2::new(
            self.base.location().x + self.size.x / 2.0,
            self.base.location().y + self.size.y / 2.0,
        )
    }

    // private functions
    fn draw_solid_rectangle(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        let rect = Rect::from_min_size(self.base.location() + canvas_offset, self.size);
        painter.rect(
            rect,
            CornerRadius::ZERO,
            self.base.fill_color(),
            Stroke::new(self.base.line_width(), self.base.color()), // border
            StrokeKind::Outside,                                    // Outside / Inside / Middle
        );
    }

    fn draw_broken_rectangle(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        let rect = Rect::from_min_size(self.base.location() + canvas_offset, self.size);
        let stroke = egui::Stroke::new(self.base.line_width(), self.base.color());
        let pts = vec![
            rect.left_top(),
            rect.left_bottom(),
            rect.right_bottom(),
            rect.right_top(),
            rect.left_top(),
        ];
        painter.rect_filled(rect, CornerRadius::ZERO, self.base.fill_color());

        match self.base.line_style() {
            LineStyle::Dashed => {
                let shapes = egui::Shape::dashed_line(
                    &pts,
                    stroke,
                    self.base.dash_length(),
                    self.base.dash_gap(),
                );
                painter.extend(shapes);
            }
            LineStyle::Dotted => {
                let shapes = egui::Shape::dotted_line(
                    &pts,
                    self.base.color(),
                    self.base.dot_spacing(),
                    self.base.dot_radius(),
                );
                painter.extend(shapes);
            }
            LineStyle::Solid => {}
        }
    }
} //impl Rectangle

/// Implement trait Shape for Rectangle.
///
/// Make trait [`Shape`] methods available.
impl Shape for Rectangle {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        if self.base.line_style() == LineStyle::Solid {
            self.draw_solid_rectangle(painter, canvas_offset);
        } else {
            self.draw_broken_rectangle(painter, canvas_offset);
        }
    }
} // impl Shape for Rectangle
