//
//
//

use crate::mesh::{LineIndex, PolyIndex, VertIndex};

pub enum Selection {
    Verticies(Vec<VertIndex>),
    Lines((Vec<LineIndex>)),
    Polys((Vec<PolyIndex>)),
}

pub struct EditorState {}
