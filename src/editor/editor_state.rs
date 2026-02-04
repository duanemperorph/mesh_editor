//
// editor_state -> managed dyanmic editor state (separate from mesh being edited)
//

use crate::editor_panel_state::*;
use crate::insert_operation::*;
use crate::selection::*;
use mesh_editor::mesh::Axis;
use strum::Display;

#[derive(Display)]
pub enum InputMode {
    Select,
    Insert,
    Connect,
    Groups,
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
    panel_state_free_cam: PanelStateFreeCam,
    pending_insert_operation: Option<InsertOperation>,
    viewer_mode: ViewerMode,
    selected_axis: Axis,
}

//
// Init + Acessors
//

impl EditorState {
    pub fn new() -> EditorState {
        EditorState {
            selection: Selection::new(),
            input_mode: InputMode::Select,
            panel_state_xz: PanelState2D::new(PanelViewingPlane::XZ),
            panel_state_yz: PanelState2D::new(PanelViewingPlane::YZ),
            panel_state_xy: PanelState2D::new(PanelViewingPlane::XY),
            panel_state_free_cam: PanelStateFreeCam::new(),
            viewer_mode: ViewerMode::EditorPanels,
            pending_insert_operation: None,
            selected_axis: Axis::X,
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

    pub fn set_input_mode(&mut self, new_mode: InputMode) {
        self.pending_insert_operation = None;
        self.input_mode = new_mode;
    }

    pub fn panel_state_xz(&self) -> &PanelState2D {
        &self.panel_state_xz
    }

    pub fn panel_state_yz(&self) -> &PanelState2D {
        &self.panel_state_yz
    }

    pub fn panel_state_free_cam(&self) -> &PanelStateFreeCam {
        &self.panel_state_free_cam
    }

    pub fn panel_state_xy(&self) -> &PanelState2D {
        &self.panel_state_xy
    }

    pub fn panel_state_2d_from_plane_mut(
        &mut self,
        viewing_plane: PanelViewingPlane,
    ) -> &mut PanelState2D {
        match (viewing_plane) {
            PanelViewingPlane::XZ => &mut self.panel_state_xz,
            PanelViewingPlane::XY => &mut self.panel_state_xy,
            PanelViewingPlane::YZ => &mut self.panel_state_yz,
        }
    }

    pub fn panel_state_2d_from_plane(&self, viewing_plane: PanelViewingPlane) -> &PanelState2D {
        match (viewing_plane) {
            PanelViewingPlane::XZ => &self.panel_state_xz,
            PanelViewingPlane::XY => &self.panel_state_xy,
            PanelViewingPlane::YZ => &self.panel_state_yz,
        }
    }

    pub fn panel_state_xz_mut(&mut self) -> &mut PanelState2D {
        &mut self.panel_state_xz
    }

    pub fn panel_state_yz_mut(&mut self) -> &mut PanelState2D {
        &mut self.panel_state_yz
    }

    pub fn panel_state_xy_mut(&mut self) -> &mut PanelState2D {
        &mut self.panel_state_xy
    }

    pub fn panel_state_free_cam_mut(&mut self) -> &mut PanelStateFreeCam {
        &mut self.panel_state_free_cam
    }

    pub fn pending_insert_operation(&self) -> Option<InsertOperation> {
        self.pending_insert_operation
    }

    pub fn pending_insert_operation_mut(&mut self) -> Option<&mut InsertOperation> {
        self.pending_insert_operation.as_mut()
    }

    pub fn set_pending_insert_operation(&mut self, new_op: InsertOperation) {
        self.pending_insert_operation = Some(new_op);
    }

    pub fn clear_pending_insert_operation(&mut self) {
        self.pending_insert_operation = None
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

    pub fn selected_axis(&self) -> Axis {
        self.selected_axis
    }

    pub fn set_selected_axis(&mut self, new_axis: Axis) {
        self.selected_axis = new_axis;
    }
}
