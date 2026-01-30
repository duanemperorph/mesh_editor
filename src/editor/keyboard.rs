//
// keyboard -> handle the keypresses and update the state
//

use crate::editor_state::*;
use macroquad::prelude::*;

pub fn handle_key_presses(editor_state: &mut EditorState) {
    if is_key_pressed(KeyCode::Tab) {
        editor_state.toggle_viewer_mode();
    }
    if is_key_pressed(KeyCode::F1) {
        editor_state.set_input_mode(InputMode::SelectVerts);
    }
    if is_key_pressed(KeyCode::F2) {
        editor_state.set_input_mode(InputMode::SelectLines);
    }
    if is_key_pressed(KeyCode::F3) {
        editor_state.set_input_mode(InputMode::SelectPolys);
    }
    if is_key_pressed(KeyCode::F4) {
        editor_state.set_input_mode(InputMode::InsertVerts);
    }
}
