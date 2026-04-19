//! ## Module base contains the [`ShapeBase`] struct and the [`Shape`] trait.
// base.rs

use crate::egui::{self, Color32, Pos2, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineStyle {
    Solid,
    Dashed,
    Dotted,
}

/// Base struct for all shapes.
///
/// Implementations of the `Shape` trait use this struct to store common properties.
/// ShapeBase methods are available for any Shape.
#[derive(Debug)]
pub struct ShapeBase {
    pub(crate) location: Pos2,
    pub(crate) color: Color32,
    pub(crate) fill_color: Color32,
    pub(crate) line_width: f32,
    pub(crate) line_style: LineStyle,
}

impl ShapeBase {
    // /// Create a new, empty ShapeBase with default values.
    // pub fn new() -> Self {
    //     Self::default()
    // }
    pub fn location(&self) -> Pos2 {
        self.location
    }
    pub fn move_to(&mut self, location: Pos2) {
        self.location = location;
    }
    pub fn color(&self) -> Color32 {
        self.color
    }
    pub fn set_color(&mut self, col: Color32) {
        self.color = col;
    }

    pub fn fill_color(&self) -> Color32 {
        self.fill_color
    }
    pub fn set_fill_color(&mut self, col: Color32) {
        self.fill_color = col;
    }

    pub fn line_width(&self) -> f32 {
        self.line_width
    }
    pub fn set_line_width(&mut self, lw: f32) {
        self.line_width = lw;
    }

    pub fn line_style(&self) -> LineStyle {
        self.line_style
    }
    pub fn set_line_style(&mut self, ls: LineStyle) {
        self.line_style = ls;
    }
    pub(crate) fn dash_length(&self) -> f32 {
        4.0 * self.line_width
    }
    pub(crate) fn dash_gap(&self) -> f32 {
        1.0 + (2.0 * self.line_width)
    }
    pub(crate) fn dot_radius(&self) -> f32 {
        // Diameter of 1.33 times line width looks better
        self.line_width / 1.5
    }
    pub(crate) fn dot_spacing(&self) -> f32 {
        1.0 + (2.5 * self.line_width)
    }
} // end of impl ShapeBase

impl Default for ShapeBase {
    fn default() -> Self {
        Self {
            location: Pos2::default(),
            color: Color32::BLACK,
            fill_color: Color32::TRANSPARENT,
            line_width: 2.0,
            line_style: LineStyle::Solid,
            //line_style: LineStyle::Dashed { dash: 8.0, gap: 4.0 },
            //line_style: LineStyle::Dashed,
            //line_style: LineStyle::Dotted { spacing: 8.0, radius: 2.0 },
            //line_style: LineStyle::Dotted,
        }
    }
} // end of impl Default for ShapeBase

/// trait Shape is implemented by all shapes.
///
/// Importantly, all shapes can call the draw_at() function, which draws the shape.
///
/// This trait represents a generic geometric shape with properties like
/// location, color, line width, and additional rendering behaviors.
///
/// Types implementing this trait must define methods to access and
/// modify a `ShapeBase` and provide the ability to render themselves on
/// a canvas using `egui::Painter`.
///
/// # Requirements
/// The trait requires implementors to manage shape properties via the `ShapeBase`:
/// - Location (`Pos2`)
/// - Stroke color (`Color32`)
/// - Fill color (`Color32`)
/// - Line width (`f32`)
/// - Line style (`LineStyle`)
///
/// # Methods
/// - **Drawing**: Render the shape in either canvas-local or default coordinates.
/// - **State Management**: Get or set the shape's base properties.
/// pub
pub trait Shape: std::fmt::Debug {
    fn base(&self) -> &ShapeBase;
    fn base_mut(&mut self) -> &mut ShapeBase;

    /// Draw in *canvas-local* coordinates, translated by `canvas_offset`
    /// where `canvas_offset` is the screen-space top-left of the canvas.
    fn draw_at(&self, painter: &egui::Painter, canvas_offset: egui::Vec2);

    /// Convenience: draw with canvas at (0,0)
    fn draw(&self, painter: &egui::Painter) {
        self.draw_at(painter, egui::Vec2::ZERO);
    }

    fn location(&self) -> Pos2 {
        self.base().location()
    }

    fn move_to(&mut self, location: Pos2) {
        self.base_mut().move_to(location)
    }

    fn color(&self) -> Color32 {
        self.base().color()
    }
    fn set_color(&mut self, col: Color32) {
        self.base_mut().set_color(col)
    }

    fn fill_color(&self) -> Color32 {
        self.base().fill_color()
    }
    fn set_fill_color(&mut self, col: Color32) {
        self.base_mut().set_fill_color(col)
    }

    fn line_width(&self) -> f32 {
        self.base().line_width()
    }
    fn set_line_width(&mut self, lw: f32) {
        self.base_mut().set_line_width(lw)
    }
    fn line_style(&self) -> LineStyle {
        self.base().line_style
    }
    fn set_line_style(&mut self, ls: LineStyle) {
        self.base_mut().set_line_style(ls)
    }
} // end of trait Shape
