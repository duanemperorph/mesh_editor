use crate::insert_operation::*;
use crate::selection::Selection;
use macroquad::prelude::{Mesh as MacroMesh, *};
use mesh_editor::mesh::{Mesh as MeshData, *};
use std::collections::HashSet;

pub(super) fn render_mesh(mesh: &MeshData, selection: &Selection) {
    let selected_verts: HashSet<VertIndex> = selection.selected_vert_indicies_set();
    let selected_lines: HashSet<LineIndex> = mesh.lines_in_vertex_indicies(&selected_verts);
    let selected_polys: HashSet<PolyIndex> = mesh.polys_in_vertex_indicies(&selected_verts);

    render_triangles(mesh);

    if (selected_polys.len() > 0) {
        render_triangles_selected_polys(mesh, &selected_polys);
    }

    render_lines(mesh, &selected_lines);
    render_points(mesh, &selected_verts);
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

fn render_triangles(mesh: &MeshData) {
    let unselected_color = GRAY;
    let mesh = mesh_data_to_macro_mesh(mesh, unselected_color);
    draw_mesh(&mesh);
}

fn render_triangles_selected_polys(mesh: &MeshData, selected_polys: &HashSet<PolyIndex>) {
    let selected_color = Color::new(0.5, 0.35, 0.35, 1.0);
    let mesh = selected_polys_to_macro_mesh(mesh, selected_polys, selected_color);
    draw_mesh(&mesh);
}

pub(super) fn render_insert_operation(operation: InsertOperation, mesh: &MeshData) {
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
