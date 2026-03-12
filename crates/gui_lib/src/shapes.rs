// src/gui_lib/shapes.rs

pub mod base;
pub mod line;
pub mod circle;
pub mod polyline;
pub mod rectangle;
pub mod text;

pub use base::{LineStyle, Shape, ShapeBase};
pub use line::Line;
pub use circle::Circle;
pub use polyline::Polyline;
pub use rectangle::Rectangle;
pub use text::{Text, TextFont};

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
