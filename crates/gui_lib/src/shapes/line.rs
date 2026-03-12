// line.rs

use crate::Circle;
use crate::egui::{self, Pos2};
use crate::shapes::base::{Shape, ShapeBase};

pub struct Line {
    base: ShapeBase,
    pub end: Pos2,
}

impl Line {
    pub fn new(start: Pos2, end: Pos2) -> Self {
        let ps = end - start;
        Self {
            base: ShapeBase {
                location: start,
                ..Default::default()
            },
            end,
        }
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        let start = self.base.location() + canvas_offset;
        let end = self.end + canvas_offset;
        //painter.add(egui::Shape::line_segment([start, end], egui::Stroke::new(self.base.line_width(), self.base.color())));

        painter.line_segment(
            [start, end],
            egui::Stroke::new(self.base.line_width(), self.base.color()),
        );
    }
}

// impl BaseShape {
//     pub fn stroke(&self) -> egui::Stroke {
//         egui::Stroke::new(self.line_width(), self.color())
//     }
// }

// impl Circle {
//
//     pub fn new(center: Pos2, radius: f32) -> Self {
//         Self::new_from_center(center, radius)
//     }
//     pub fn new_from_center(center: Pos2, radius: f32) -> Self {
//         Self {
//             base: ShapeBase {
//                 location: center,
//                 ..Default::default()
//             },
//             radius,
//         }
//     }
//
//     pub fn new_from_top_left(tl: Pos2, radius: f32) -> Self {
//         let center = Pos2::new(tl.x + radius, tl.y + radius);
//         Self::new(center, radius)
//     }
// }
//
// impl Shape for Circle {
//     fn base(&self) -> &ShapeBase {
//         &self.base
//     }
//     fn base_mut(&mut self) -> &mut ShapeBase {
//         &mut self.base
//     }
//
//     fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
//         let center = self.base.location() + canvas_offset;
//
//         painter.circle(
//             center,
//             self.radius,
//             self.base.fill_color(),
//             egui::Stroke::new(self.base.line_width(), self.base.color()),
//         );
//     }
// }
