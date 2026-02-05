use crate::selection::Selection;
use macroquad::prelude::*;
use mesh_editor::mesh::{Mesh as MeshData, *};
use std::f32::consts::PI;

use super::mesh::render_mesh;
use super::{pop_model_matrix, push_model_matrix};

pub(super) fn render_mesh_with_mirror_mode(
    mode: &MirrorMode,
    mesh: &MeshData,
    selection: &Selection,
) {
    match mode {
        MirrorMode::None => {
            render_mesh(mesh, selection);
        }
        MirrorMode::Bilateral => {
            render_bilateral_symmetry(mesh, selection);
        }
        MirrorMode::RadialX(count) => {
            render_radial_symmetry(Axis::X, *count, mesh, selection);
        }
        MirrorMode::RadialY(count) => {
            render_radial_symmetry(Axis::Y, *count, mesh, selection);
        }
        MirrorMode::RadialZ(count) => {
            render_radial_symmetry(Axis::Z, *count, mesh, selection);
        }
    }
}

fn render_bilateral_symmetry(mesh: &MeshData, selection: &Selection) {
    render_mesh(mesh, selection);
    push_model_matrix(Mat4::from_scale(vec3(-1.0, 1.0, 1.0)));
    render_mesh(mesh, selection);
    pop_model_matrix();
}

fn render_radial_symmetry(axis: Axis, sides: u8, mesh: &MeshData, selection: &Selection) {
    let rotation_delta = 2.0 * PI / (sides as f32);

    for i in 0..sides {
        let rotation = rotation_delta * (i as f32);
        render_mesh_rotated(axis, rotation, mesh, selection)
    }
}

fn render_mesh_rotated(axis: Axis, rotation: f32, mesh: &MeshData, selection: &Selection) {
    let rotation_mat = match axis {
        Axis::X => Mat4::from_euler(EulerRot::XYZ, rotation, 0.0, 0.0),
        Axis::Y => Mat4::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
        Axis::Z => Mat4::from_euler(EulerRot::XYZ, 0.0, 0.0, rotation),
    };
    push_model_matrix(rotation_mat);
    render_mesh(mesh, selection);
    pop_model_matrix();
}
