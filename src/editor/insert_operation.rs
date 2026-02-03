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

impl InsertVertOperation {
    pub fn new(new_vert: Vec3, origin_vert_index: Option<VertIndex>) -> InsertVertOperation {
        InsertVertOperation {
            new_vert,
            origin_vert_index,
        }
    }

    pub fn apply(&self, mesh: &mut MeshData) {
        mesh.add_vert(self.new_vert);

        if let Some(origin_vert_index) = self.origin_vert_index {
            let new_vert_index = mesh.verts().len() - 1;
            mesh.add_line((origin_vert_index, new_vert_index));
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

    pub fn get_constructed_poly(&self, mesh: &MeshData) -> Option<Poly> {
        let verts_between = mesh.find_verts_between(self.new_line.0, self.new_line.1);

        if verts_between.len() >= 3 {
            return Some(verts_between);
        }
        None
    }

    pub fn apply(&self, mesh: &mut MeshData) {
        mesh.add_line(self.new_line);

        if self.completes_poly
            && let Some(new_poly) = self.get_constructed_poly(mesh)
        {
            mesh.add_poly(new_poly);
        }
    }
}
