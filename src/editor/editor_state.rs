//
// editor_state -> managed dyanmic editor state (separate from mesh being edited)
//

use macroquad::prelude::*;
use mesh_editor::mesh::{Line, LineIndex, Poly, PolyIndex, VertIndex};
use strum::Display;

pub trait PanelCameraVectors {
    fn to_target_vec(&self) -> Vec3;
    fn to_camera_pos_vec(&self) -> Vec3;
    fn to_up_vec(&self) -> Vec3;
    fn to_model_rotation(&self) -> Vec3;
}

pub enum Selection {
    None,
    Verticies(Vec<VertIndex>),
    Lines(Vec<LineIndex>),
    Polys(Vec<PolyIndex>),
}

#[derive(Display)]
pub enum InputMode {
    SelectVerts,
    SelectLines,
    SelectPolys,
    InsertVerts,
}

#[derive(PartialEq)]
pub enum ViewerMode {
    EditorPanels,
    FreeCam,
}

pub struct InsertPreview {
    vert: Option<Vec3>,
    line: Option<Line>,
    poly: Option<Poly>,
}

#[derive(Copy, Clone)]
pub enum PanelViewingPlane {
    XZ,
    YZ,
    XY,
}

#[derive(Copy, Clone)]
pub struct PanelState2D {
    viewing_plane: PanelViewingPlane,
    is_flipped: bool,
    pan: Vec2,
    distance: f32,
}

#[derive(Copy, Clone)]
pub struct PanelStateFreeCam {
    rotation: Vec2,
    distance: f32,
}

pub struct EditorState {
    selection: Selection,
    input_mode: InputMode,
    panel_state_xz: PanelState2D,
    panel_state_yz: PanelState2D,
    panel_state_xy: PanelState2D,
    panel_state_rotate_cam: PanelStateFreeCam,
    insert_preview: InsertPreview,
    viewer_mode: ViewerMode,
}

impl PanelCameraVectors for PanelState2D {
    fn to_target_vec(&self) -> Vec3 {
        match self.viewing_plane {
            PanelViewingPlane::XY => vec3(self.pan.x, self.pan.y, 0.0),
            PanelViewingPlane::XZ => vec3(self.pan.x, 0.0, self.pan.y),
            PanelViewingPlane::YZ => vec3(0.0, self.pan.x, self.pan.y),
        }
    }

    fn to_camera_pos_vec(&self) -> Vec3 {
        let offset = self.distance * if self.is_flipped { -1.0 } else { 1.0 };
        match self.viewing_plane {
            PanelViewingPlane::XY => vec3(0.0, 0.0, offset),
            PanelViewingPlane::XZ => vec3(0.0, offset, 0.0),
            PanelViewingPlane::YZ => vec3(offset, 0.0, 0.0),
        }
    }

    fn to_up_vec(&self) -> Vec3 {
        match self.viewing_plane {
            PanelViewingPlane::XY => vec3(0.0, 1.0, 0.0),
            PanelViewingPlane::XZ => vec3(0.0, 0.0, 1.0),
            PanelViewingPlane::YZ => vec3(0.0, 1.0, 0.0),
        }
    }

    fn to_model_rotation(&self) -> Vec3 {
        return vec3(0.0, 0.0, 0.0);
    }
}

impl PanelCameraVectors for PanelStateFreeCam {
    fn to_target_vec(&self) -> Vec3 {
        vec3(0.0, 0.0, 0.0)
    }

    fn to_camera_pos_vec(&self) -> Vec3 {
        // Fixed camera position along Z axis at distance
        vec3(0.0, 0.0, self.distance)
    }

    fn to_up_vec(&self) -> Vec3 {
        vec3(0.0, 1.0, 0.0)
    }

    fn to_model_rotation(&self) -> Vec3 {
        // rotation.x = yaw (Y axis), rotation.y = pitch (X axis)
        vec3(self.rotation.y, self.rotation.x, 0.0)
    }
}

//
// Init + Acessors
//

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

impl PanelState2D {
    pub fn new(viewing_plane: PanelViewingPlane) -> PanelState2D {
        PanelState2D {
            is_flipped: false,
            pan: vec2(0.0, 0.0),
            distance: 5.0,
            viewing_plane,
        }
    }

    pub fn is_flipped(&self) -> bool {
        self.is_flipped
    }

    pub fn pan(&self) -> Vec2 {
        self.pan
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        self.is_flipped = flipped;
    }

    pub fn set_pan(&mut self, pan: Vec2) {
        self.pan = pan;
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
    }
}

impl PanelStateFreeCam {
    pub fn new() -> PanelStateFreeCam {
        PanelStateFreeCam {
            rotation: vec2(0.0, 0.0),
            distance: 10.0,
        }
    }

    pub fn rotation(&self) -> Vec2 {
        self.rotation
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn set_rotation(&mut self, rotation: Vec2) {
        self.rotation = rotation;
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
    }
}

impl EditorState {
    pub fn new() -> EditorState {
        EditorState {
            selection: Selection::None,
            input_mode: InputMode::SelectVerts,
            panel_state_xz: PanelState2D::new(PanelViewingPlane::XZ),
            panel_state_yz: PanelState2D::new(PanelViewingPlane::YZ),
            panel_state_xy: PanelState2D::new(PanelViewingPlane::XY),
            panel_state_rotate_cam: PanelStateFreeCam::new(),
            viewer_mode: ViewerMode::EditorPanels,
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

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
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

    pub fn panel_state_rotate_cam(&self) -> &PanelStateFreeCam {
        &self.panel_state_rotate_cam
    }

    pub fn panel_state_rotate_cam_mut(&mut self) -> &mut PanelStateFreeCam {
        &mut self.panel_state_rotate_cam
    }

    pub fn panel_state_xy(&self) -> &PanelState2D {
        &self.panel_state_xy
    }

    pub fn panel_state_xy_mut(&mut self) -> &mut PanelState2D {
        &mut self.panel_state_xy
    }

    pub fn insert_preview(&self) -> &InsertPreview {
        &self.insert_preview
    }

    pub fn insert_preview_mut(&mut self) -> &mut InsertPreview {
        &mut self.insert_preview
    }

    pub fn viewer_mode(&self) -> &ViewerMode {
        &self.viewer_mode
    }

    pub fn toggle_viewer_mode(&mut self) {
        match self.viewer_mode {
            ViewerMode::EditorPanels => self.viewer_mode = ViewerMode::FreeCam,
            ViewerMode::FreeCam => self.viewer_mode = ViewerMode::EditorPanels,
        }
    }
}
