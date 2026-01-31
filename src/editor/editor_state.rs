//
// editor_state -> managed dyanmic editor state (separate from mesh being edited)
//

use crate::editor_panel_state::*;
use crate::insert_preview_state::*;
use crate::panes::Panes;
use macroquad::prelude::*;
use mesh_editor::mesh::{Line, LineIndex, Poly, PolyIndex, VertIndex};
use strum::Display;

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

//
// Init + Acessors
//

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
