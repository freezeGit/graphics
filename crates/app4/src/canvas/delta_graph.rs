//use crate::inits;
use crate::inits;
use gui_lib::{Line, Lines, Pos2, Rectangle, Shape, ShapeBase, Text, Vec2};
// TDJ: zoom

const SG_SIZE: i32 = 250;
const SG_SPACING: f32 = 4.0;
const SG_MARK_SIZE: f32 = 3.0;
const SG_HEIGHT: f32 = 600.0;
const SG_WIDTH: f32 = SG_SIZE as f32 * SG_SPACING;

#[derive(Debug, Default)]
pub struct DeltaGraph {
    base: ShapeBase,
    means: Vec<Rectangle>,
    horiz_lines: Lines,
    vertical_lines: Lines,
    nvec: Vec<Text>,
}

impl DeltaGraph {
    pub fn new(location: Pos2) -> Self {
        let mut base = ShapeBase::default();
        base.move_to(location);

        let mut means = Vec::new();

        assert!(SG_SIZE > 0);
        for i in 0..=SG_SIZE {
            //TDJ: Is this right?
            let mut rect = Rectangle::new_from_center(
                // Initial position is off-screen
                location + egui::vec2(i as f32 * SG_SPACING, -10000.0),
                Vec2::splat(SG_MARK_SIZE),
            );
            rect.set_line_width(1.0);
            //rect.set_color(egui::Color32::LIGHT_GRAY);
            rect.set_color(egui::Color32::RED);
            rect.set_fill_color(egui::Color32::RED);
            means.push(rect);
        }

        let mut horiz_lines = Lines::new(
            location,
            vec![
                [Pos2::new(0.0, 0.0), Pos2::new(SG_WIDTH, 0.0)],
                [
                    Pos2::new(0.0, -SG_HEIGHT * 0.75),
                    Pos2::new(SG_WIDTH, -SG_HEIGHT * 0.75),
                ],
                [
                    Pos2::new(0.0, -SG_HEIGHT * 0.5),
                    Pos2::new(SG_WIDTH, -SG_HEIGHT * 0.5),
                ],
                [
                    Pos2::new(0.0, -SG_HEIGHT * 0.25),
                    Pos2::new(SG_WIDTH, -SG_HEIGHT * 0.25),
                ],
                [Pos2::new(0.0, -SG_HEIGHT), Pos2::new(SG_WIDTH, -SG_HEIGHT)],
            ],
        );
        horiz_lines.set_line_width(1.0);

        let mut vertical_lines = Lines::new(
            location,
            vec![
                [Pos2::new(0.0, 0.0), Pos2::new(0.0, -SG_HEIGHT)],
                [
                    Pos2::new(SG_WIDTH / 2.0, 0.0),
                    Pos2::new(SG_WIDTH / 2.0, -SG_HEIGHT),
                ],
                [Pos2::new(SG_WIDTH, 0.0), Pos2::new(SG_WIDTH, -SG_HEIGHT)],
            ],
        );
        vertical_lines.set_line_width(1.0);

        let mut st_p2 = Text::new_from_center(location + egui::vec2(-30.0, -SG_HEIGHT), "+2");
        let mut st_p1 =
            Text::new_from_center(location + egui::vec2(-30.0, -SG_HEIGHT * 0.75), "+1");
        let mut st_0 = Text::new_from_center(location + egui::vec2(-30.0, -SG_HEIGHT * 0.5), "0");
        let mut st_m1 =
            Text::new_from_center(location + egui::vec2(-30.0, -SG_HEIGHT * 0.25), "\u{2212}1");
        let mut st_m2 = Text::new_from_center(location + egui::vec2(-30.0, 0.0), "\u{2212}2");
        let nvec = vec![st_m2, st_m1, st_0, st_p1, st_p2];

        Self {
            base,
            means,
            horiz_lines,
            vertical_lines,
            nvec,
        }
    }

    pub fn location(&self) -> Pos2 {
        self.base.location()
    }

    pub fn set_mean_y(&mut self, index: usize, y_val: f32) {
        let x = self.means[index].center().x;
        let y = self.location().y - 0.5 * SG_HEIGHT - 0.25 * y_val * SG_HEIGHT;
        let new_pos = Pos2::new(x, y);
        self.means[index].move_to(new_pos);
    }
} // impl DeltaGraph

impl Shape for DeltaGraph {
    fn base(&self) -> &ShapeBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ShapeBase {
        &mut self.base
    }

    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2) {
        self.horiz_lines.draw_at(painter, canvas_offset);
        self.vertical_lines.draw_at(painter, canvas_offset);
        for s in &self.nvec {
            s.draw_at(painter, canvas_offset);
        }
        for s in &self.means {
            s.draw_at(painter, canvas_offset);
        }
    }
} // impl Shape for DeltaGraph
