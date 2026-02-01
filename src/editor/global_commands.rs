//
// keyboard -> handle the keypresses and update the state
//

use crate::editor_state::*;
use macroquad::prelude::*;

pub struct ModifierKeys {
    shift_key: bool,
    meta_key: bool, // both command or ctrl
    alt_key: bool,
}

pub fn handle_global_keyboard_commands(editor_state: &mut EditorState) {
    if is_key_pressed(KeyCode::Tab) {
        editor_state.toggle_viewer_mode();
    }
    if is_key_pressed(KeyCode::Space) {
        editor_state.toggle_input_mode();
    }
}

pub fn check_modifier_keys() -> ModifierKeys {
    let shift_key = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
    let alt_key = is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt);

    // Use Ctrl on Windows, Command (Super) on macOS/Linux
    let meta_key = if cfg!(target_os = "windows") {
        is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)
    } else {
        is_key_down(KeyCode::LeftSuper) || is_key_down(KeyCode::RightSuper)
    };

    ModifierKeys {
        shift_key,
        meta_key,
        alt_key,
    }
}
