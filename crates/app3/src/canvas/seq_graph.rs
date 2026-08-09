use gui_lib::{Lines, Pos2, ShapeBase};

pub struct SeqGraph {
    vec: Vec<i32>,
}

impl SeqGraph {
    pub fn new(location: Pos2, size: i32) -> Self {
        let mut vec = Vec::new();
        for i in 1..size{
            vec.push(i);

        }
        Self {
            // base: ShapeBase {
            //     location,
            //     ..Default::default()
            // },
            vec,
        }
   }

} // impl Lines



// #[derive(Debug, Clone, Copy)]
// pub struct Rectangle {
//     pub x: u32,
//     pub y: u32,
//     pub width: u32,
//     pub height: u32,
// }
//
// pub struct Container {
//     data: Box<[Rectangle]>,
// }
//
// impl Container {
//     // 1. IMMUTABLE GETTER: Returns an optional reference to read data
//     // Best for: Rendering/drawing loops where you just need coordinates
//     pub fn get_rect(&self, index: usize) -> Option<&Rectangle> {
//         self.data.get(index)
//     }
//
//     // 2. MUTABLE UPDATE: Directly modifies the fields of a specific rectangle
//     // Best for: Drag-and-drop actions, resizing, or moving elements
//     pub fn update_rect_position(&mut self, index: usize, new_x: u32, new_y: u32) -> bool {
//         if let Some(rect) = self.data.get_mut(index) {
//             rect.x = new_x;
//             rect.y = new_y;
//             true // Successfully updated
//         } else {
//             false // Index was out of bounds
//         }
//     }
//
//     // 3. WHOLE REPLACEMENT: Replaces the entire rectangle structure at once
//     // Best for: Swapping out a shape completely (enabled since we derived Copy/Clone!)
//     pub fn replace_rect(&mut self, index: usize, new_rect: Rectangle) -> bool {
//         if let Some(rect) = self.data.get_mut(index) {
//             *rect = new_rect; // Overwrites the existing rectangle instantly
//             true
//         } else {
//             false
//         }
//     }
// }
//
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
