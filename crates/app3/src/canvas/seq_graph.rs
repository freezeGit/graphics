use gui_lib::{Lines, Pos2, Rectangle, Shape, ShapeBase, Vec2};
use std::cell::RefCell;
use std::rc::Rc;

const SG_SIZE: i32 = 180;
const SG_SPACING: f32 = 6.0;
const SG_MARK_SIZE: f32 = 5.0;
const SG_HEIGHT: f32 = 500.0;
const SG_WIDTH: f32 = SG_SIZE as f32 * SG_SPACING;

#[derive(Debug, Default)]
pub struct SeqGraph {
    base: ShapeBase,
    vec: Vec<Rectangle>,
    lines: Lines,
}

impl SeqGraph {
    pub fn new(location: Pos2) -> Self {
        let mut base = ShapeBase::default();

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
                [Pos2::new(-20.0, -250.0), Pos2::new(SG_WIDTH + 20.0, -250.0)],
                [Pos2::new(-20.0, -500.0), Pos2::new(SG_WIDTH + 20.0, -500.0)],
            ],
        );
        lines.set_line_width(1.0);

        Self { base, vec, lines }
    }

    pub fn add_val(&mut self, ones_fraction: f32) {
        if self.vec.is_empty() {
            return;
        }
        let clamped_fraction = ones_fraction.clamp(0.0, 1.0);

        for i in 0..self.vec.len() - 1 {
            let current_x = self.vec[i].location().x;
            let next_y = self.vec[i + 1].location().y;
            let new_location = egui::Pos2::new(current_x, next_y);
            self.vec[i].move_to(new_location);
        }

        let current_x = self.vec.last_mut().unwrap().location().x;
        let next_y = clamped_fraction * SG_HEIGHT; // Fix: Used clamped variable
        let new_location = egui::Pos2::new(current_x, next_y);
        self.vec.last_mut().unwrap().move_to(new_location);
    }

    //let length = 950.0 * (world.bits.ones_fraction() as f32);
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

// impl Container {
//     // Updates all positions dynamically at runtime
//     pub fn update_all_positions(&mut self, spacing: u32, base_x: u32, base_y: u32) {
//         // .enumerate() provides the index (i), .iter_mut() lets us modify the elements
//         for (i, rect) in self.data.iter_mut().enumerate() {
//             let index = i as u32;
//
//             // The formula calculates the unique position for each item based on its index
//             rect.x = base_x + (index * (rect.width + spacing));
//             rect.y = base_y;
//         }
//     }
// }
