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
    let fovy = panel.distance();
    let aspect = viewport.w / viewport.h;
    let screen_width_fraction = (viewport.w as f32) / screen_width();
    let screen_height_fraction = (viewport.h as f32) / screen_height();

    println!("calced aspect: {}", aspect);

    let world_dx = -screen_delta.x * fovy * aspect / screen_width_fraction;
    let world_dy = -screen_delta.y * fovy / screen_height_fraction;

    return vec2(world_dx, world_dy);
}
