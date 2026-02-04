//
// keyboard_commands -> handle keyboard commands
//
use crate::editor_state::*;
use crate::insert_operation::*;
use crate::keyboard::*;
use macroquad::prelude::*;
use mesh_editor::mesh::{Mesh as MeshData, *};
use std::collections::*;

pub fn handle_keyboard_commands(editor_state: &mut EditorState, mesh: &mut MeshData) {
    handle_global_commands(editor_state);

    match editor_state.input_mode() {
        InputMode::Select => handle_selection_commands(editor_state, mesh),
        InputMode::Insert | InputMode::Connect => handle_insert_commands(editor_state, mesh),
        InputMode::Groups => {} //TODO: this
    }
}

//
// Global commands
//
fn handle_global_commands(editor_state: &mut EditorState) {
    if is_key_pressed(KeyCode::Tab) {
        editor_state.toggle_viewer_mode();
    }
    if is_key_pressed(KeyCode::F1) {
        editor_state.set_input_mode(InputMode::Select);
    }
    if is_key_pressed(KeyCode::F2) {
        editor_state.set_input_mode(InputMode::Insert);
    }
    if is_key_pressed(KeyCode::F3) {
        editor_state.set_input_mode(InputMode::Connect);
    }
    if is_key_pressed(KeyCode::F4) {
        editor_state.set_input_mode(InputMode::Groups);
    }

    // Clear/New mesh (Cmd+Shift+N)
    if is_key_down(KeyCode::LeftSuper)
        && is_key_down(KeyCode::LeftShift)
        && is_key_pressed(KeyCode::N)
    {
        // TODO: clear/new mesh
    }

    // Undo (Cmd+Z)
    if is_key_down(KeyCode::LeftSuper)
        && !is_key_down(KeyCode::LeftShift)
        && is_key_pressed(KeyCode::Z)
    {
        // TODO: undo
    }

    // Redo (Cmd+Shift+Z)
    if is_key_down(KeyCode::LeftSuper)
        && is_key_down(KeyCode::LeftShift)
        && is_key_pressed(KeyCode::Z)
    {
        // TODO: redo
    }
}

//
// Insert commands
//
fn handle_insert_commands(editor_state: &mut EditorState, mesh: &mut MeshData) {
    let modifier_keys = check_modifier_keys();

    if is_key_pressed(KeyCode::Space) {
        if let Some(insert_op) = editor_state.pending_insert_operation() {
            if let InsertOperation::Vert(insert_vert_op) = insert_op {
                insert_vert_op.apply(mesh);
                let new_vert_index = mesh.verts().len() - 1;
                // add new point to the selection
                editor_state
                    .selection_mut()
                    .add_selected_vert_indicies(&[new_vert_index]);
            } else if let InsertOperation::Line(insert_line_op) = insert_op {
                insert_line_op.apply(mesh);
            }
        }
        editor_state.clear_pending_insert_operation();
    }

    if let Some(InsertOperation::Vert(insert_op)) = editor_state.pending_insert_operation_mut() {
        let delta = if modifier_keys.meta_key {
            0.01
        } else if modifier_keys.shift_key {
            0.1
        } else {
            0.01
        };

        // Translate pending insert vertex (W/S=Z, A/D=X, R/F=Y)
        if is_key_pressed(KeyCode::W) {
            insert_op.new_vert.z += delta;
        }
        if is_key_pressed(KeyCode::S) {
            insert_op.new_vert.z -= delta;
        }
        if is_key_pressed(KeyCode::A) {
            insert_op.new_vert.x -= delta;
        }
        if is_key_pressed(KeyCode::D) {
            insert_op.new_vert.x += delta;
        }
        if is_key_pressed(KeyCode::R) {
            insert_op.new_vert.y += delta;
        }
        if is_key_pressed(KeyCode::F) {
            insert_op.new_vert.y -= delta;
        }
    }
}

//
// Selection commands
//
fn handle_selection_commands(editor_state: &mut EditorState, mesh: &mut MeshData) {
    let modifier_keys = check_modifier_keys();

    if is_key_pressed(KeyCode::X) {
        let selected_verts = editor_state.selection().selected_vert_indicies_set();
        let polys = mesh.polys_partially_in_vertex_indicies(&selected_verts);
        let verts_from_polys = mesh.vert_indicies_from_poly_indicies(polys);
        editor_state
            .selection_mut()
            .replace_selected_vert_indicies(&verts_from_polys);
    }
    if is_key_pressed(KeyCode::Escape) {
        editor_state.selection_mut().clear()
    }

    let selected_verts: Vec<usize> = editor_state.selection().selected_vert_indicies().clone();

    let delta = if modifier_keys.meta_key {
        0.01
    } else if modifier_keys.shift_key {
        0.1
    } else {
        0.01
    };

    // Translation (W/S=Z, A/D=X, R/F=Y)
    if is_key_pressed(KeyCode::W) {
        mesh.translate_verts(&selected_verts, vec3(0.0, 0.0, delta));
    }
    if is_key_pressed(KeyCode::S) {
        mesh.translate_verts(&selected_verts, vec3(0.0, 0.0, -delta));
    }
    if is_key_pressed(KeyCode::A) {
        mesh.translate_verts(&selected_verts, vec3(-delta, 0.0, 0.0));
    }
    if is_key_pressed(KeyCode::D) {
        mesh.translate_verts(&selected_verts, vec3(delta, 0.0, 0.0));
    }
    if is_key_pressed(KeyCode::R) {
        mesh.translate_verts(&selected_verts, vec3(0.0, delta, 0.0));
    }
    if is_key_pressed(KeyCode::F) {
        mesh.translate_verts(&selected_verts, vec3(0.0, -delta, 0.0));
    }

    // Select axis (F5-F7)
    if is_key_pressed(KeyCode::F5) {
        editor_state.set_selected_axis(Axis::X);
    }
    if is_key_pressed(KeyCode::F6) {
        editor_state.set_selected_axis(Axis::Y);
    }
    if is_key_pressed(KeyCode::F7) {
        editor_state.set_selected_axis(Axis::Z);
    }

    let axis = editor_state.selected_axis();
    let rotation_delta = if modifier_keys.shift_key {
        std::f32::consts::PI / 12.0 // 15 degrees
    } else {
        std::f32::consts::PI / 180.0 // 1 degree
    };
    let scale_factor = if modifier_keys.shift_key { 1.1 } else { 1.01 };

    // Rotation (Q/E)
    if is_key_pressed(KeyCode::Q) {
        mesh.rotate_verts(&selected_verts, -rotation_delta, axis);
    }
    if is_key_pressed(KeyCode::E) {
        mesh.rotate_verts(&selected_verts, rotation_delta, axis);
    }

    // Scale (=/-)
    if is_key_pressed(KeyCode::Equal) {
        mesh.scale_verts_around_axis(&selected_verts, scale_factor, axis);
    }
    if is_key_pressed(KeyCode::Minus) {
        mesh.scale_verts_around_axis(&selected_verts, 1.0 / scale_factor, axis);
    }

    // Delete (Cmd+D)
    if modifier_keys.meta_key && !modifier_keys.shift_key && is_key_pressed(KeyCode::D) {
        mesh.delete_verts(&selected_verts);
        editor_state.selection_mut().clear();
    }

    // Split (Option+S)
    if modifier_keys.alt_key && is_key_pressed(KeyCode::S) {
        let selected_verts = editor_state.selection().selected_vert_indicies_set();
        let line_indicies: HashSet<usize> = mesh.lines_in_vertex_indicies(&selected_verts);
        mesh.split_lines(&line_indicies);
    }

    // Duplicate (Cmd+Shift+D)
    if modifier_keys.meta_key && modifier_keys.shift_key && is_key_pressed(KeyCode::D) {
        mesh.duplicate_verts(&selected_verts, false);
    }

    // Extrude (Cmd+X)
    if modifier_keys.meta_key && is_key_pressed(KeyCode::X) {
        mesh.duplicate_verts(&selected_verts, true);
    }
}
