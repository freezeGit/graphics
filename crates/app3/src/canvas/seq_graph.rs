//use crate::inits;
use gui_lib::{Lines, Pos2, Rectangle, Shape, ShapeBase, Vec2};

// TDJ: zoom

const SG_SIZE: i32 = 180;
const SG_SPACING: f32 = 6.0;
const SG_MARK_SIZE: f32 = 5.0;
const SG_HEIGHT: f32 = 500.0;
const SG_WIDTH: f32 = SG_SIZE as f32 * SG_SPACING;

#[derive(Debug, Clone, Copy)]
struct Zoom {
    scale: f32,
    focus: f32,
}

impl Default for Zoom {
    fn default() -> Self {
        Self {
            scale: 1.0,
            focus: 0.5,
        }
    }
}

#[derive(Debug, Default)]
pub struct SeqGraph {
    base: ShapeBase,
    vec: Vec<Rectangle>,
    lines: Lines,
    zoom: Zoom,
}

impl SeqGraph {
    pub fn new(location: Pos2) -> Self {
        let mut base = ShapeBase::default();
        base.move_to(location);

        let mut vec = Vec::new();

        assert!(SG_SIZE > 0);
        for i in 0..SG_SIZE {
            let mut rect = Rectangle::new_from_center(
                location + egui::vec2(i as f32 * SG_SPACING, 0.0),
                Vec2::splat(SG_MARK_SIZE),
            );
            rect.set_line_width(1.0);
            rect.set_color(egui::Color32::LIGHT_GRAY);
            rect.set_fill_color(egui::Color32::DARK_BLUE);
            vec.push(rect);
            //))
        }

        let mut lines: Lines = Lines::new(
            //Pos2::new(250.0, 705.0),
            location,
            vec![
                [Pos2::new(-20.0, 0.0), Pos2::new(SG_WIDTH + 20.0, 0.0)],
                [
                    Pos2::new(-20.0, -(SG_HEIGHT / 2.0)),
                    Pos2::new(SG_WIDTH + 20.0, -(SG_HEIGHT / 2.0)),
                ],
                [
                    Pos2::new(-20.0, -SG_HEIGHT),
                    Pos2::new(SG_WIDTH + 20.0, -SG_HEIGHT),
                ],
            ],
        );
        lines.set_line_width(1.0);

        Self {
            base,
            vec,
            lines,
            zoom: Zoom::default(),
        }
    }

    pub fn location(&self) -> Pos2 {
        self.base.location()
    }

    pub fn add_val(&mut self, ones_fraction: f32) {
        if self.vec.is_empty() {
            return;
        }

        for i in 0..self.vec.len() - 1 {
            let current_x = self.vec[i].location().x;
            let next_y = self.vec[i + 1].location().y;
            let new_location = egui::Pos2::new(current_x, next_y);
            self.vec[i].move_to(new_location);
        }

        let vx = self.vec.last_mut().unwrap().location().x;

        let clamped_fraction = ones_fraction.clamp(0.0, 1.0);
        let scaled_height =
            (0.5 + (clamped_fraction - self.zoom.focus) * self.zoom.scale) * SG_HEIGHT;
        let mark_offset = SG_MARK_SIZE / 2.0;
        let vy = self.location().y - (mark_offset + scaled_height);

        let loc = egui::Pos2::new(vx, vy);
        self.vec.last_mut().unwrap().move_to(loc);

        if self.zoom.scale == 1.0 {
            self.lines.set_color(egui::Color32::TRANSPARENT);
        }
    }
} // impl SeqGraph

impl Shape for SeqGraph {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        for s in &self.vec {
            s.draw_at(painter, canvas_offset);
        }
        self.lines.draw_at(painter, canvas_offset);
    }
} // impl Shape for SeqGraph
