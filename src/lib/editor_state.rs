//
//
//

use crate::mesh::{Coord3D, Line, LineIndex, Poly, PolyIndex, VertIndex};

pub enum Selection {
    Verticies(Vec<VertIndex>),
    Lines(Vec<LineIndex>),
    Polys(Vec<PolyIndex>),
}

pub enum InputMode {
    SingleSelect,
    MultiSelect,
    InsertVert,
}

pub struct InsertPreview {
    vert: Option<Coord3D>,
    line: Option<Line>,
    poly: Option<Poly>,
}

// TOOD:
// * input mode placeholder

pub struct PanelState2D {
    flipped: bool,
    pan: (f32, f32),
    zoom: f32,
}

pub struct PanelStateRotateCam {
    rotation: (f32, f32),
    zoom: f32,
}

pub struct EditorState {
    selection: Selection,
    input_mode: InputMode,
    panel_state_xz: PanelState2D,
    panel_state_yz: PanelState2D,
    panel_state_rotate_cam: PanelStateRotateCam,
    show_grid: bool,
}
