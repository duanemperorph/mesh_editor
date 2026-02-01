//
// keyboard_commands -> handle keyboard commands
//
use crate::editor_state::EditorState;
use macroquad::prelude::*;
use mesh_editor::mesh::Mesh as MeshData;

pub fn handle_keyboard_commands(editor_state: &mut EditorState, mesh: &mut MeshData) {
    if is_key_pressed(KeyCode::Tab) {
        editor_state.toggle_viewer_mode();
    }
    if is_key_pressed(KeyCode::Space) {
        editor_state.toggle_input_mode();
    }
    if is_key_pressed(KeyCode::X) {
        let selected_verts = editor_state.selection().selected_vert_indicies();
        let polys = mesh.polys_partially_in_vertex_indicies(selected_verts);
        let verts_from_polys = mesh.vert_indicies_from_poly_indicies(polys);
        editor_state
            .selection_mut()
            .replace_selected_vert_indicies_set(verts_from_polys);
    }
    if is_key_pressed(KeyCode::Escape) {
        editor_state.selection_mut().clear()
    }
}
