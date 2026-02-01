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
    if is_key_pressed(KeyCode::X) {}
}
