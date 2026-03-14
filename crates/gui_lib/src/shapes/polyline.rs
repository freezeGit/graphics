// polyline.rs

use crate::egui::{self, Color32, Pos2, Vec2};
use crate::shapes::base::{LineStyle, Shape, ShapeBase};

/// A customizable Polyline component.
///
/// # Fields
/// * location: Pos2 - All points will be plotted relative to 'location'.
///                  - Any point Pos2::ZERO will be plotted at 'location'.
/// * points: Vec<Pos2> - The points to be joined to form the polyline.
#[derive(Debug, Default)]
pub struct Polyline {
    base: ShapeBase,
    points: Vec<Pos2>,
}

impl Polyline {
    pub fn new(location: Pos2, points: impl IntoIterator<Item = Pos2>) -> Self {
        Self {
            base: ShapeBase {
                location,
                ..Default::default()
            },
            points: points.into_iter().collect(),
        }
    }
}

impl Shape for Polyline {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        let translation = self.base.location().to_vec2() + canvas_offset;
        let points_trans: Vec<egui::Pos2> =
            self.points.iter().map(|p| *p + translation).collect();
        let stroke = egui::Stroke::new(self.base.line_width(), self.base.color());

        match self.base.line_style() {
            LineStyle::Solid => {
                painter.line(points_trans, stroke);
            },
            LineStyle::Dashed => {
                let shapes = egui::Shape::dashed_line(
                    &points_trans,
                    stroke,
                    self.base.dash_length(),
                    self.base.dash_gap(),
                );
                painter.extend(shapes);
            },
            LineStyle::Dotted => {
                let shapes = egui::Shape::dotted_line(
                    &points_trans,
                    self.base.color(),
                    self.base.dot_spacing(),
                    self.base.dot_radius(),
                );
                painter.extend(shapes);
            },
        }
    }
}



