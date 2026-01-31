//
// insert_preview_state -> state type for insert preview
//
use macroquad::prelude::*;
use mesh_editor::mesh::{Line, LineIndex, Poly, PolyIndex, VertIndex};

pub struct InsertPreview {
    vert: Option<Vec3>,
    line: Option<Line>,
    poly: Option<Poly>,
}

impl InsertPreview {
    pub fn new() -> InsertPreview {
        InsertPreview {
            vert: None,
            line: None,
            poly: None,
        }
    }

    pub fn vert(&self) -> Option<&Vec3> {
        self.vert.as_ref()
    }

    pub fn line(&self) -> Option<&Line> {
        self.line.as_ref()
    }

    pub fn poly(&self) -> Option<&Poly> {
        self.poly.as_ref()
    }

    pub fn set_vert(&mut self, vert: Vec3) {
        self.vert = Some(vert);
    }

    pub fn set_line(&mut self, line: Line) {
        self.line = Some(line)
    }

    pub fn set_poly(&mut self, poly: Poly) {
        self.poly = Some(poly);
    }

    pub fn clear_vert(&mut self) {
        self.vert = None
    }

    pub fn clear_line(&mut self) {
        self.line = None
    }

    pub fn clear_poly(&mut self) {
        self.poly = None
    }

    pub fn clear(&mut self) {
        self.vert = None;
        self.line = None;
        self.poly = None;
    }
}
