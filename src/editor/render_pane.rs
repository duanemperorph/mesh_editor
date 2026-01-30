//
// render_pane -> pane renderer functions
//

use crate::editor_state::*;
use crate::panes::Viewport;
use macroquad::prelude::*;
use mesh_editor::mesh::Mesh as MeshData;

pub fn render_editor_pane_viewport(
    panel_state: &impl PanelCameraVectors,
    mesh: MeshData,
    viewport: Viewport,
) {
    let camera = Camera3D {
        position: panel_state.to_camera_pos_vec(),
        target: panel_state.to_target_vec(),
        up: panel_state.to_up_vec(),
        fovy: 45.0,
        projection: Projection::Orthographics,
        viewport: Some(viewport),
        render_target: None,
        aspect: None,
        z_near: 0.01,
        z_far: 10000.0,
    };
    set_camera(&camera);
}
