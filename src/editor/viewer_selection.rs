//
// viewer_selection - Handle selection in any of the "2D" editor panes
//
use crate::editor_panel_state::*;
use crate::editor_state::*;
use crate::panes::*;
use macroquad::prelude::*;
use mesh_editor::mesh::{Mesh as MeshData, *};

pub fn select_point_under_mouse(
    mouse_coord: Vec2,
    editor_state: &mut EditorState,
    mesh: &mut MeshData,
    panel: PanelState2D,
    viewport: Viewport,
) {
}
