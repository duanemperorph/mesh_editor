//
// insert_preview_state -> state type for insert preview
//
use macroquad::prelude::*;
use mesh_editor::mesh::{Line, LineIndex, Mesh as MeshData, Poly, PolyIndex, VertIndex};

pub struct InsertVertOperation {
    pub selected_vert: Option<VertIndex>,
    pub new_vert: Vec3,
}

pub struct InsertLineOperation {
    pub new_line: Line,
    pub completes_poly: bool,
}

pub enum InsertOperation {
    Vert(InsertVertOperation),
    Line(InsertLineOperation),
    None,
}

pub struct InsertOperationResult {
    new_vert: Option<Vec3>,
    new_line: Option<Line>,
    new_poly: Option<Poly>,
}

impl InsertVertOperation {
    pub fn new(new_vert: Vec3, selected_vert: Option<VertIndex>) -> InsertVertOperation {
        InsertVertOperation {
            new_vert,
            selected_vert,
        }
    }
}

impl InsertLineOperation {
    fn new(new_line: Line, completes_poly: bool) -> InsertLineOperation {
        InsertLineOperation {
            new_line,
            completes_poly,
        }
    }
}

impl InsertOperation {
    fn insert_into(&self, mesh: &MeshData) -> InsertOperationResult {
        match self {
            Self::Vert(vert_op) => {
                let new_vert = Some(vert_op.new_vert);
                let new_line = if let Some(selected_index) = vert_op.selected_vert {
                    let new_index = mesh.verts().len();
                    Some((selected_index, new_index))
                } else {
                    None
                };
                InsertOperationResult {
                    new_vert,
                    new_line,
                    new_poly: None,
                }
            }
            Self::Line(line_op) => {
                let new_line = line_op.new_line;
                let new_poly = if line_op.completes_poly {
                    let points_between = mesh.find_verts_between(new_line.0, new_line.1);
                    if points_between.len() >= 3 {
                        Some(points_between)
                    } else {
                        None
                    }
                } else {
                    None
                };
                InsertOperationResult {
                    new_vert: None,
                    new_line: Some(new_line),
                    new_poly,
                }
            }
            Self::None => InsertOperationResult::empty(),
        }
    }
}

impl InsertOperationResult {
    fn new(
        new_vert: Option<Vec3>,
        new_line: Option<Line>,
        new_poly: Option<Poly>,
    ) -> InsertOperationResult {
        return InsertOperationResult {
            new_vert,
            new_line,
            new_poly,
        };
    }

    fn empty() -> InsertOperationResult {
        InsertOperationResult {
            new_vert: None,
            new_line: None,
            new_poly: None,
        }
    }
}
