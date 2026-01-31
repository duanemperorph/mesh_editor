//
// render_pane -> pane renderer functions
//

use crate::editor_panel_state::*;
use crate::panes::Viewport;
use macroquad::prelude::{Mesh as MacroMesh, *};
use mesh_editor::mesh::Mesh as MeshData;

pub trait PanelCameraVectors {
    fn to_target_vec(&self) -> Vec3;
    fn to_camera_pos_vec(&self) -> Vec3;
    fn to_up_vec(&self) -> Vec3;
    fn to_model_rotation(&self) -> Vec3;
    fn distance(&self) -> f32;
}

pub fn render_editor_pane_viewport(
    panel_state: &impl PanelCameraVectors,
    mesh: &MeshData,
    viewport: Viewport,
) {
    let aspect = (viewport.2 as f32) / (viewport.3 as f32);
    let fovy = panel_state.distance() * 2.0;

    // let fovy = if aspect < 1.0 { aspect * fovy } else { fovy };

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

//
// Per-panel camera setup
//

impl PanelCameraVectors for PanelState2D {
    fn to_target_vec(&self) -> Vec3 {
        match self.viewing_plane() {
            PanelViewingPlane::XY => vec3(-self.pan().x, self.pan().y, 0.0),
            PanelViewingPlane::XZ => vec3(self.pan().x, 0.0, self.pan().y),
            PanelViewingPlane::YZ => vec3(0.0, self.pan().y, self.pan().x),
        }
    }

    fn to_camera_pos_vec(&self) -> Vec3 {
        let offset = self.distance() * if self.is_flipped() { -1.0 } else { 1.0 };
        match self.viewing_plane() {
            PanelViewingPlane::XY => vec3(-self.pan().x, self.pan().y, offset),
            PanelViewingPlane::XZ => vec3(self.pan().x, offset, self.pan().y),
            PanelViewingPlane::YZ => vec3(offset, self.pan().y, self.pan().x),
        }
    }

    fn to_up_vec(&self) -> Vec3 {
        match self.viewing_plane() {
            PanelViewingPlane::XY => vec3(0.0, 1.0, 0.0),
            PanelViewingPlane::XZ => vec3(0.0, 0.0, 1.0),
            PanelViewingPlane::YZ => vec3(0.0, 1.0, 0.0),
        }
    }

    fn to_model_rotation(&self) -> Vec3 {
        vec3(0.0, 0.0, 0.0)
    }

    fn distance(&self) -> f32 {
        PanelState2D::distance(self)
    }
}

impl PanelCameraVectors for PanelStateFreeCam {
    fn to_target_vec(&self) -> Vec3 {
        vec3(0.0, 0.0, 0.0)
    }

    fn to_camera_pos_vec(&self) -> Vec3 {
        // Fixed camera position along Z axis at distance
        vec3(0.0, 0.0, self.distance())
    }

    fn to_up_vec(&self) -> Vec3 {
        vec3(0.0, 1.0, 0.0)
    }

    fn to_model_rotation(&self) -> Vec3 {
        // rotation.x = yaw (Y axis), rotation.y = pitch (X axis)
        vec3(self.rotation().y, self.rotation().x, 0.0)
    }

    fn distance(&self) -> f32 {
        PanelStateFreeCam::distance(self)
    }
}
