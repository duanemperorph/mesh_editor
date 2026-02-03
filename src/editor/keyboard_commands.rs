//
// keyboard_commands -> handle keyboard commands
//
use crate::editor_state::*;
use crate::insert_operation::*;
use macroquad::prelude::*;
use mesh_editor::mesh::Mesh as MeshData;

pub fn handle_keyboard_commands(editor_state: &mut EditorState, mesh: &mut MeshData) {
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
}
