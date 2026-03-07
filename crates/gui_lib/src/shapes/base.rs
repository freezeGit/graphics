// base.rs

use crate::egui::{self, Color32, Pos2, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineStyle {
    Solid,
    Dashed,
    Dotted,
    //Dashed { dash: f32, gap: f32 },
    //Dotted { spacing: f32, radius: f32 },
}

/// Base struct for all shapes.
#[derive(Debug)]
pub struct ShapeBase {
    pub(crate) location: Pos2,
    pub(crate) points: Vec<Pos2>,
    pub(crate) color: Color32,
    pub(crate) fill_color: Color32,
    pub(crate) line_width: f32,
    pub(crate) line_style: LineStyle,
}

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
}

impl Default for ShapeBase {
    fn default() -> Self {
        Self {
            location: Pos2::default(),
            points: Vec::new(),
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

    pub(crate) fn points_translated(&self, offset: Vec2) -> Vec<Pos2> {
        self.points.iter().map(|p| *p + offset).collect()
    }

    pub(crate) fn dash_length(&self) -> f32 {
        4.0 * self.line_width
    }
    pub(crate) fn dash_gap(&self) -> f32 {
        1.0 + (2.0 * self.line_width)
    }
    pub(crate) fn dot_radius(&self) -> f32 {
        self.line_width / 2.0
    }
    pub(crate) fn dot_spacing(&self) -> f32 {
        1.0 + (2.0 * self.line_width)
    }
}
