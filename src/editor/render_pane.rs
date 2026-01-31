//
// render_pane -> pane renderer functions
//

use crate::editor_panel_state::*;
use crate::panes::Viewport;
use macroquad::prelude::{Mesh as MacroMesh, *};
use mesh_editor::mesh::Mesh as MeshData;

pub fn render_editor_pane_viewport(
    panel_state: &impl PanelCameraVectors,
    mesh: &MeshData,
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
    render_mesh(mesh);
    render_lines(mesh);
    render_points(mesh);
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

//
// Render mesh in one go zoom zoom
//
fn render_mesh(mesh: &MeshData) {
    let color = GRAY;
    let mesh = mesh_data_to_macro_mesh(mesh, GRAY);
    draw_mesh(&mesh);
}

fn mesh_data_to_macro_mesh(mesh_data: &MeshData, color: Color) -> MacroMesh {
    let vertices = mesh_data
        .verts()
        .iter()
        .map(|v| Vertex {
            position: *v,
            uv: Vec2::ZERO,
            color: color.into(),
            normal: Vec4::ZERO,
        })
        .collect();

    let indices = mesh_data
        .polys_to_triangle_indicies()
        .iter()
        .map(|index| *index as u16)
        .collect();

    pub struct Mesh {
        pub vertices: Vec<Vertex>,
        pub indices: Vec<u16>,
        pub texture: Option<Texture2D>,
    }

    MacroMesh {
        vertices,
        indices,
        texture: None,
    }
}
