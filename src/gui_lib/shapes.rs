// src/gui_lib/shapes.rs

pub mod base;
pub mod polyline;
pub mod circle;
pub mod rectangle;

pub use base::{LineStyle, Shape, ShapeBase};
pub use polyline::Polyline;
pub use circle::Circle;
pub use rectangle::Rectangle;

// Planned Shapes:
// - Polyline
// - Closed_polyline
// - Polygon
// - Marked_polyline
// - Marks
// - Mark
// - Marker (Shapes optimized for drawing markers on graphs or charts
// - Lines
// - Rectangle
// - Circle
// - Ellipse
// - Text
// - Line
// - Axis
// - Function
// - Image
// - Shape_rect (A group of Shapes enclosed in and clipped by a Rectangle)

// Discussion in ChatGPT
// - Rust question/Modular structure refactor
//  "allow dashed and dotted lines"

