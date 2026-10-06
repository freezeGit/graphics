//use crate::inits;
use crate::inits;
use gui_lib::{Line, Lines, Pos2, Rectangle, Shape, ShapeBase, Vec2};
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
            scale: inits::SEQ_GRAPH_SCALE,
            focus: inits::SEQ_GRAPH_FOCUS,
        }
    }
}

#[derive(Debug, Default)]
pub struct SeqGraph {
    base: ShapeBase,
    ones_vec: Vec<Rectangle>,
    cxns_vec: Vec<Rectangle>,
    mid: Line,
    lines: Lines,
    zoom: Zoom,
}

enum SeqGraphVals {
    Ones,
    Cxns,
}

impl SeqGraph {
    pub fn new(location: Pos2) -> Self {
        assert!(SG_SIZE > 0);

        let mut base = ShapeBase::default();
        base.move_to(location);

        let mut cxns_vec = Vec::new();
        for i in 0..SG_SIZE {
            let mut rect = Rectangle::new_from_center(
                // Initial position is off-screen
                location + egui::vec2(i as f32 * SG_SPACING, -10000.0),
                Vec2::splat(SG_MARK_SIZE),
            );
            rect.set_line_width(1.0);
            rect.set_color(egui::Color32::LIGHT_GRAY);
            rect.set_fill_color(egui::Color32::BLUE);
            cxns_vec.push(rect);
        }

        let mut ones_vec = Vec::new();
        for i in 0..SG_SIZE {
            let mut rect = Rectangle::new_from_center(
                // Initial position is off-screen
                location + egui::vec2(i as f32 * SG_SPACING, -10000.0),
                Vec2::splat(SG_MARK_SIZE),
            );
            rect.set_line_width(1.0);
            rect.set_color(egui::Color32::LIGHT_GRAY);
            //rect.set_fill_color(egui::Color32::DARK_BLUE);
            rect.set_fill_color(egui::Color32::RED);
            ones_vec.push(rect);
        }

        let mut mid: Line = Line::new_from_points(
            location + egui::vec2(-20.0, -(SG_HEIGHT / 2.0)),
            location + egui::vec2(SG_WIDTH + 20.0, -(SG_HEIGHT / 2.0)),
        );
        mid.set_line_width(1.0);

        let mut lines: Lines = Lines::new(
            //Pos2::new(250.0, 705.0),
            location,
            vec![
                [Pos2::new(-20.0, 0.0), Pos2::new(SG_WIDTH + 20.0, 0.0)],
                [
                    Pos2::new(-20.0, -SG_HEIGHT),
                    Pos2::new(SG_WIDTH + 20.0, -SG_HEIGHT),
                ],
            ],
        );
        lines.set_line_width(1.0);

        Self {
            base,
            ones_vec,
            cxns_vec,
            mid,
            lines,
            zoom: Zoom::default(),
        }
    }

    pub fn location(&self) -> Pos2 {
        self.base.location()
    }

    pub fn zoom_scale(&self) -> f32 {
        self.zoom.scale
    }

    pub fn zoom_focus(&self) -> f32 {
        self.zoom.focus
    }

    pub fn set_zoom_scale(&mut self, scale: f32) {
        if scale.is_finite() {
            self.zoom.scale = scale.max(0.0);
        }
    }

    pub fn set_zoom_focus(&mut self, focus: f32) {
        if focus.is_finite() {
            self.zoom.focus = focus.clamp(0.0, 1.0);
        }
    }

    pub fn add_ones_val(&mut self, ones_fraction: f32) {
        self.add_val(SeqGraphVals::Ones, ones_fraction);
    }

    pub fn add_cxns_val(&mut self, cxns_fraction: f32) {
        self.add_val(SeqGraphVals::Cxns, cxns_fraction);
    }

    fn add_val(&mut self, sg_vals: SeqGraphVals, fraction: f32) {
        let self_y = self.location().y;
        let zoom_focus = self.zoom.focus;
        let zoom_scale = self.zoom.scale;

        let vec_val = match sg_vals {
            SeqGraphVals::Ones => &mut self.ones_vec,
            SeqGraphVals::Cxns => &mut self.cxns_vec,
        };

        if vec_val.is_empty() {
            return;
        }

        for i in 0..vec_val.len() - 1 {
            let current_x = vec_val[i].location().x;
            let next_y = vec_val[i + 1].location().y;
            let new_location = egui::Pos2::new(current_x, next_y);
            vec_val[i].move_to(new_location);
        }

        let vx = vec_val.last_mut().unwrap().location().x;

        let clamped_fraction = fraction.clamp(0.0, 1.0);
        let scaled_height = (0.5 + (clamped_fraction - zoom_focus) * zoom_scale) * SG_HEIGHT;
        let mark_offset = SG_MARK_SIZE / 2.0;
        let vy = self_y - (mark_offset + scaled_height);

        let loc = egui::Pos2::new(vx, vy);
        vec_val.last_mut().unwrap().move_to(loc);
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
        for s in &self.ones_vec {
            s.draw_at(painter, canvas_offset);
        }
        for s in &self.cxns_vec {
            s.draw_at(painter, canvas_offset);
        }
        self.mid.draw_at(painter, canvas_offset);
        if self.zoom.scale == 1.0 && self.zoom.focus == 0.5 {
            self.lines.draw_at(painter, canvas_offset);
        }
    }
} // impl Shape for SeqGraph
