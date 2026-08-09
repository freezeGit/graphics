use gui_lib::{Lines, Pos2, Rectangle, Shape, ShapeBase, Vec2};

#[derive(Debug, Default)]
pub struct SeqGraph {
    base: ShapeBase,
    vec: Vec<Rectangle>,
}

impl SeqGraph {
    // const SG_SIZE: i32 = 200;
    //pub fn new(location: Pos2, size: i32) -> Self {
    // pub fn new(location: Pos2) -> Self {
    //     const SG_SIZE: i32 = 200;
    //     //const SPACING: f32 = 4.0;
    //     const SPACING: f32 = 20.0;
    //     //const RECT_SIZE: f32 = 2.0;
    //     const RECT_SIZE: f32 = 18.0;
    //     let mut base = ShapeBase::default();
    //
    //     let mut vec = Vec::new();
    //     //let spacing = 4.0;
    //     for i in 0..SG_SIZE {
    //         vec.push(Rectangle::new_from_center(
    //             location + egui::vec2(i as f32 * SPACING, 0.0),
    //             Vec2::splat(RECT_SIZE),
    //         ));
    //     }
    //     Self { base, vec }
    // }

    pub fn new(location: Pos2) -> Self {
        const SG_SIZE: i32 = 180;
        //const SPACING: f32 = 4.0;
        const SPACING: f32 = 6.0;
        //const RECT_SIZE: f32 = 2.0;
        const RECT_SIZE: f32 = 5.0;
        let mut base = ShapeBase::default();

        let mut vec = Vec::new();
        //let spacing = 4.0;
        for i in 0..SG_SIZE {
            let mut rect = Rectangle::new_from_center(
                location + egui::vec2(i as f32 * SPACING, 0.0),
                Vec2::splat(RECT_SIZE),
            );
            rect.set_line_width(1.0);
            rect.set_color(egui::Color32::LIGHT_GRAY);
            rect.set_fill_color(egui::Color32::DARK_BLUE);
            vec.push(rect);
            //))
        }
        Self { base, vec }
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
    }
}



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
