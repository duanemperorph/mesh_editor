//
// keyboard -> handle the keypresses and update the state
//

use macroquad::prelude::*;

pub struct ModifierKeys {
    pub shift_key: bool,
    pub meta_key: bool, // both command or ctrl
    pub alt_key: bool,
}

pub fn check_modifier_keys() -> ModifierKeys {
    let shift_key = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
    let alt_key = is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt);

    // Use Ctrl on Windows, Command (Super) on macOS/Linux
    let ctrl_key = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
    let super_key = is_key_down(KeyCode::LeftSuper) || is_key_down(KeyCode::RightSuper);

    ModifierKeys {
        shift_key,
        meta_key: ctrl_key || super_key,
        alt_key,
    }
}
