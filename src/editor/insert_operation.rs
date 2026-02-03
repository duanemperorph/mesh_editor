//
// insert_preview_state -> state type for insert preview
//
use macroquad::prelude::*;
use mesh_editor::mesh::{Line, LineIndex, Mesh as MeshData, Poly, PolyIndex, VertIndex};

#[derive(Copy, Clone)]
pub struct InsertVertOperation {
    pub new_vert: Vec3,
    pub origin_vert_index: Option<VertIndex>,
}

#[derive(Copy, Clone)]
pub struct InsertLineOperation {
    pub new_line: Line,
    pub completes_poly: bool,
}

#[derive(Copy, Clone)]
pub enum InsertOperation {
    Vert(InsertVertOperation),
    Line(InsertLineOperation),
}

pub struct InsertOperationResult {
    new_vert: Option<Vec3>,
    new_line: Option<(Vec3, Vec3)>,
    new_poly: Option<Poly>,
}

impl InsertVertOperation {
    pub fn new(new_vert: Vec3, origin_vert_index: Option<VertIndex>) -> InsertVertOperation {
        InsertVertOperation {
            new_vert,
            origin_vert_index,
        }
    }
}

impl InsertLineOperation {
    pub fn new(new_line: Line, completes_poly: bool) -> InsertLineOperation {
        InsertLineOperation {
            new_line,
            completes_poly,
        }
    }
}

impl InsertOperation {
    //todo: this
}

impl InsertOperationResult {
    //todo: this
}
