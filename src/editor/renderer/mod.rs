//
// renderer -> pane renderer functions
//

mod camera;
mod mesh;
mod mirror;

pub use camera::PanelCameraVectors;

use crate::panes::Viewport;
use crate::selection::Selection;
use macroquad::prelude::*;
use mesh_editor::mesh::Mesh as MeshData;

use mirror::render_mesh_with_mirror_mode;

pub fn render_editor_pane_viewport(
    panel_state: &impl PanelCameraVectors,
    mesh: &MeshData,
    selection: &Selection,
    viewport: Viewport,
    is_displaying_grid: bool,
) {
    let aspect = (viewport.2 as f32) / (viewport.3 as f32);
    let fovy = panel_state.distance() * 2.0;

    let camera = Camera3D {
        position: panel_state.to_camera_pos_vec(),
        target: panel_state.to_target_vec(),
        up: panel_state.to_up_vec(),
        fovy: fovy,
        projection: Projection::Orthographics,
        viewport: Some(viewport),
        render_target: None,
        aspect: Some(aspect),
        z_near: 0.01,
        z_far: 10000.0,
    };
    set_camera(&camera);

    // apply panel model rotation matrix if needed;
    let rotation = panel_state.to_model_rotation();
    let rotation_matrix = Mat4::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
    push_model_matrix(rotation_matrix);

    // before rendering the mesh draw backgroud objects like grid and axis lines
    if is_displaying_grid {
        draw_grid(100, 0.1, BLUE, Color::new(0.3, 0.3, 0.3, 1.0));
    }

    render_mesh_with_mirror_mode(mesh.mirror_mode(), mesh, selection);

    pop_model_matrix();
}

pub(crate) fn push_model_matrix(matrix: Mat4) {
    unsafe {
        get_internal_gl().quad_gl.push_model_matrix(matrix);
    }
}

pub(crate) fn pop_model_matrix() {
    unsafe {
        get_internal_gl().quad_gl.pop_model_matrix();
    }
}
