//! ## module closed_olyline
//! Contains a customizable Polyline component.
//!
// polyline.rs

use crate::egui::epaint::PathShape;
use crate::egui::{self, Color32, Pos2, Stroke, Vec2};
use crate::shapes::base::{LineStyle, Shape, ShapeBase};

/// A customizable ClosedPolyline component.
///
/// # Fields
/// * base: ShapeBase - The base properties of the shape.
/// * points: Vec<Pos2> - The points to be joined to form the polyline.
/// All points will be plotted relative to 'location'.
/// Any point Pos2::ZERO will be plotted at 'location'.
#[derive(Debug, Default)]
pub struct ClosedPolyline {
    base: ShapeBase,
    points: Vec<Pos2>,
}

impl ClosedPolyline {
    pub fn new(location: Pos2, points: impl IntoIterator<Item = Pos2>) -> Self {
        Self {
            base: ShapeBase {
                location,
                ..Default::default()
            },
            points: points.into_iter().collect(),
        }
    }

    // --------- Private functions ---------
    fn close_last_point(pts: &mut Vec<Pos2>) {
        pts.push(pts[0]);
    }

    fn draw_fill(&self, painter: &egui::Painter, pts: &Vec<Pos2>) {
        let closed_path = PathShape {
            points: pts.clone(),
            closed: true,
            fill: self.base.fill_color(),
            stroke: Stroke::NONE.into(),
        };
        painter.add(egui::Shape::Path(closed_path));
    }
} // end of impl ClosedPolyline

/// Implement trait Shape for Polyline.
///
/// Make trait [`Shape`] methods available.
impl Shape for ClosedPolyline {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        if self.points.len() < 2 {
            return;
        }

        let translation = self.base.location().to_vec2() + canvas_offset;
        let mut points_trans: Vec<egui::Pos2> =
            self.points.iter().map(|p| *p + translation).collect();
        let stroke = egui::Stroke::new(self.base.line_width(), self.base.color());

        match self.base.line_style() {
            LineStyle::Solid => {
                let closed_path = PathShape {
                    points: points_trans,
                    closed: true,
                    fill: self.base.fill_color(),
                    stroke: stroke.into(),
                };
                painter.add(egui::Shape::Path(closed_path));
            }

            LineStyle::Dashed => {
                self.draw_fill(painter, &points_trans);
                ClosedPolyline::close_last_point(&mut points_trans);

                let shapes = egui::Shape::dashed_line(
                    &points_trans,
                    stroke,
                    self.base.dash_length(),
                    self.base.dash_gap(),
                );
                painter.extend(shapes);
            }

            LineStyle::Dotted => {
                self.draw_fill(painter, &points_trans);
                ClosedPolyline::close_last_point(&mut points_trans);

                let shapes = egui::Shape::dotted_line(
                    &points_trans,
                    self.base.color(),
                    self.base.dot_spacing(),
                    self.base.dot_radius(),
                );
                painter.extend(shapes);
            }
        }
    }
} // end of impl Shape for Polyline
