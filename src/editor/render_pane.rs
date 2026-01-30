//
// render_pane -> pane renderer functions
//

use crate::editor_state::*;
use crate::panes::Viewport;
use macroquad::prelude::{Mesh as MacroMesh, *};
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

fn render_points(mesh: &MeshData) {
    let sphere_color = RED;
    let sphere_radius = 0.05;

    for v in mesh.verts() {
        draw_sphere(*v, sphere_radius, None, sphere_color);
    }
}

fn render_lines(mesh: &MeshData) {
    let line_color = GREEN;

    for (v1, v2) in mesh.lines_to_vert_pairs() {
        draw_line_3d(v1, v2, line_color);
    }
}

// fn render_polys(mesh: &MeshData) {
//     let poly_color = GRAY;

//     for (v1, v2, v3) in mesh.polys_to_triangle_verts() {
//         draw_triangle3d(v1, v2, v3, poly_color);
//     }
// }

// fn mesh_data_to_macro_mesh(mesh_data: &MeshData) -> MacroMesh {
//     // convert custom editor mesh to macro mesh
// }
