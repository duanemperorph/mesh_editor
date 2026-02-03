//
// render_pane -> pane renderer functions
//

use crate::editor_panel_state::*;
use crate::insert_operation::*;
use crate::panes::Viewport;
use crate::selection::Selection;
use macroquad::prelude::{Mesh as MacroMesh, *};
use mesh_editor::mesh::{Mesh as MeshData, *};
use std::collections::HashSet;

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
    selection: &Selection,
    viewport: Viewport,
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

    let selected_verts = selection.selected_vert_indicies_set();
    let selected_lines = mesh.lines_in_vertex_indicies(&selected_verts);
    let selected_polys = mesh.polys_in_vertex_indicies(&selected_verts);

    render_mesh(mesh);

    if (selected_polys.len() > 0) {
        render_mesh_selected_polys(mesh, &selected_polys);
    }

    render_lines(mesh, &selected_lines);
    render_points(mesh, &selected_verts);
    pop_model_matrix()
}

fn render_points(mesh: &MeshData, selected_points: &HashSet<VertIndex>) {
    let unselected_color = WHITE;
    let selected_color = RED;
    let sphere_radius = 0.05;

    for (i, v) in mesh.verts().iter().enumerate() {
        let color = if selected_points.contains(&i) {
            selected_color
        } else {
            unselected_color
        };
        draw_sphere(*v, sphere_radius, None, color);
    }
}

fn render_lines(mesh: &MeshData, selected_lines: &HashSet<LineIndex>) {
    let unselected_color = Color::new(0.85, 0.85, 0.85, 1.0);
    let selected_color = Color::new(0.4, 0.1, 0.1, 1.0);

    for (i, (v1, v2)) in mesh.lines_to_vert_pairs().iter().enumerate() {
        let color = if selected_lines.contains(&i) {
            selected_color
        } else {
            unselected_color
        };
        draw_line_3d(*v1, *v2, color);
    }
}

//
// Render mesh in one go zoom zoom
//
fn render_mesh(mesh: &MeshData) {
    let unselected_color = GRAY;
    let mesh = mesh_data_to_macro_mesh(mesh, unselected_color);
    draw_mesh(&mesh);
}

//
// Render mesh in one go zoom zoom
//
fn render_mesh_selected_polys(mesh: &MeshData, selected_polys: &HashSet<PolyIndex>) {
    let selected_color = Color::new(0.5, 0.35, 0.35, 1.0);
    let mesh = selected_polys_to_macro_mesh(mesh, selected_polys, selected_color);
    draw_mesh(&mesh);
}

fn render_insert_operation(operation: InsertOperation, mesh: &MeshData) {
    if let InsertOperation::Vert(insert_vert_op) = operation {
        let preview_color = Color::new(0.0, 0.5, 0.5, 1.0);
        let sphere_radius = 0.05;

        draw_sphere(insert_vert_op.new_vert, sphere_radius, None, preview_color);

        if let Some(origin_vert_index) = insert_vert_op.origin_vert_index
            && let Some(&origin_vert) = mesh.verts().get(origin_vert_index)
        {
            draw_line_3d(origin_vert, insert_vert_op.new_vert, preview_color);
        }
    } else if let InsertOperation::Line(insert_line_op) = operation {
        let preview_color = Color::new(0.0, 0.5, 0.5, 1.0);

        if let Some(&v1) = mesh.verts().get(insert_line_op.new_line.0)
            && let Some(&v2) = mesh.verts().get(insert_line_op.new_line.1)
        {
            draw_line_3d(v1, v2, preview_color);

            if let Some(poly) = insert_line_op.get_constructed_poly(mesh) {
                let poly_color = Color::new(0.0, 0.5, 0.5, 0.5);
                let poly_mesh = poly_to_macro_mesh(mesh, &poly, poly_color);
                draw_mesh(&poly_mesh);
            }
        }
    }
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

    MacroMesh {
        vertices,
        indices,
        texture: None,
    }
}

fn selected_polys_to_macro_mesh(
    mesh_data: &MeshData,
    selected_polys: &HashSet<PolyIndex>,
    color: Color,
) -> MacroMesh {
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
        .selected_polys_to_triangle_indicies(selected_polys)
        .iter()
        .map(|index| *index as u16)
        .collect();

    MacroMesh {
        vertices,
        indices,
        texture: None,
    }
}

fn poly_to_macro_mesh(mesh_data: &MeshData, poly: &Poly, color: Color) -> MacroMesh {
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

    let indices = MeshData::poly_indicies_to_triangle_indicies(poly)
        .iter()
        .map(|index| *index as u16)
        .collect();

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

fn push_model_matrix(matrix: Mat4) {
    unsafe {
        get_internal_gl().quad_gl.push_model_matrix(matrix);
    }
}

fn pop_model_matrix() {
    unsafe {
        get_internal_gl().quad_gl.pop_model_matrix();
    }
}
