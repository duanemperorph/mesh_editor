//
//
//

use crate::mesh::{Coord3D, Line, LineIndex, Poly, PolyIndex, VertIndex};

pub enum Selection {
    None,
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
    insert_preview: InsertPreview,
}

impl InsertPreview {
    pub fn new() -> InsertPreview {
        InsertPreview {
            vert: None,
            line: None,
            poly: None,
        }
    }

    pub fn vert(&self) -> Option<&Coord3D> {
        self.vert.as_ref()
    }

    pub fn line(&self) -> Option<&Line> {
        self.line.as_ref()
    }

    pub fn poly(&self) -> Option<&Poly> {
        self.poly.as_ref()
    }

    pub fn set_vert(&mut self, vert: Coord3D) {
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

impl PanelState2D {
    pub fn new() -> PanelState2D {
        PanelState2D {
            flipped: false,
            pan: (0.0, 0.0),
            zoom: 1.0,
        }
    }

    pub fn flipped(&self) -> bool {
        self.flipped
    }

    pub fn pan(&self) -> (f32, f32) {
        self.pan
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        self.flipped = flipped;
    }

    pub fn set_pan(&mut self, pan: (f32, f32)) {
        self.pan = pan;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }
}

impl PanelStateRotateCam {
    pub fn new() -> PanelStateRotateCam {
        PanelStateRotateCam {
            rotation: (0.0, 0.0),
            zoom: 1.0,
        }
    }

    pub fn rotation(&self) -> (f32, f32) {
        self.rotation
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn set_rotation(&mut self, rotation: (f32, f32)) {
        self.rotation = rotation;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }
}

impl EditorState {
    pub fn new() -> EditorState {
        EditorState {
            selection: Selection::None,
            input_mode: InputMode::SingleSelect,
            panel_state_xz: PanelState2D::new(),
            panel_state_yz: PanelState2D::new(),
            panel_state_rotate_cam: PanelStateRotateCam::new(),
            insert_preview: InsertPreview::new(),
        }
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut Selection {
        &mut self.selection
    }

    pub fn input_mode(&self) -> &InputMode {
        &self.input_mode
    }

    pub fn input_mode_mut(&mut self) -> &mut InputMode {
        &mut self.input_mode
    }

    pub fn panel_state_xz(&self) -> &PanelState2D {
        &self.panel_state_xz
    }

    pub fn panel_state_xz_mut(&mut self) -> &mut PanelState2D {
        &mut self.panel_state_xz
    }

    pub fn panel_state_yz(&self) -> &PanelState2D {
        &self.panel_state_yz
    }

    pub fn panel_state_yz_mut(&mut self) -> &mut PanelState2D {
        &mut self.panel_state_yz
    }

    pub fn panel_state_rotate_cam(&self) -> &PanelStateRotateCam {
        &self.panel_state_rotate_cam
    }

    pub fn panel_state_rotate_cam_mut(&mut self) -> &mut PanelStateRotateCam {
        &mut self.panel_state_rotate_cam
    }

    pub fn insert_preview(&self) -> &InsertPreview {
        &self.insert_preview
    }

    pub fn insert_preview_mut(&mut self) -> &mut InsertPreview {
        &mut self.insert_preview
    }
}
