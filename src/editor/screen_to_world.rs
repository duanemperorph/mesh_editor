//
// screen_to_world -> screen to world coord transforms
//
//

use crate::editor_panel_state::*;
use macroquad::prelude::*;
//
// Scales a screen delta -> world delta. Use for mouse pan.
//
pub fn screen_fraction_to_world_scale_vec2(
    screen_delta: Vec2,
    panel: &PanelState2D,
    viewport: Rect,
) -> Vec2 {
    let fovy = panel.distance() * 2.0;
    let aspect = viewport.w / viewport.h;

    let world_dx = -screen_delta.x * fovy * aspect;
    let world_dy = -screen_delta.y * fovy;

    return vec2(world_dx, world_dy);
}
