// polyline.rs
// use crate::gui_lib::egui::{self, Color32, Pos2, Vec2};
// use crate::gui_lib::shapes::base::{LineStyle, Shape, ShapeBase};
use crate::egui::{self, Color32, Pos2, Vec2};
use crate::shapes::base::{LineStyle, Shape, ShapeBase};

/// A customizable Polyline component.
///
/// # Fields
/// * `position` - position of the circle center (: eframe::egui::Pos2)
/// * `radius` - The radius of the button
#[derive(Debug, Default)]
pub struct Polyline {
    base: ShapeBase,
}

impl Polyline {
    pub fn new(location: Pos2, points: impl IntoIterator<Item = Pos2>) -> Self {
        Self {
            base: ShapeBase {
                location,
                points: points.into_iter().collect(),
                ..Default::default()
            },
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
        //let translation = self.base.location.to_vec2() + canvas_offset;
        let translation = self.base.location().to_vec2() + canvas_offset;

        let points = self.base.points_translated(translation);
        //let stroke = egui::Stroke::new(self.base.line_width, self.base.color);
        //let stroke = egui::Stroke::new(self.base.line_width(), self.base.color);
        let stroke = egui::Stroke::new(self.base.line_width(), self.base.color());

        match self.base.line_style() {
            LineStyle::Solid => {
                painter.add(egui::epaint::PathShape::line(points, stroke));
            }
            LineStyle::Dashed => {
                let shapes = egui::Shape::dashed_line(
                    &points,
                    stroke,
                    self.base.dash_length(),
                    self.base.dash_gap(),
                );
                painter.extend(shapes);
            }
            LineStyle::Dotted => {
                let shapes = egui::Shape::dotted_line(
                    &points,
                    //self.base.color,
                    self.base.color(),
                    self.base.dot_spacing(),
                    self.base.dot_radius(),
                );
                painter.extend(shapes);
            }
        }
    }
}
