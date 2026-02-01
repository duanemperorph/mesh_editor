//
// mod screen_to_world -> screen coord to world coord mapping functions
//

use crate::editor_panel_state::*;
use macroquad::prelude::*;

//
// Used by pan function
//
pub fn mouse_delta_to_world_scale_vec2(
    mouse_delta: Vec2,
    panel: &PanelState2D,
    viewport: Rect,
) -> Vec2 {
    let fovy = panel.distance();
    let aspect = viewport.w / viewport.h;
    let screen_width_fraction = (viewport.w as f32) / screen_width();
    let screen_height_fraction = (viewport.h as f32) / screen_height();

    let world_dx = -mouse_delta.x * fovy * aspect / screen_width_fraction;
    let world_dy = -mouse_delta.y * fovy / screen_height_fraction;

    return vec2(world_dx, world_dy);
}

//
//
//
pub fn mouse_coord_to_world_coord_vec2(
    mouse_coords: Vec2,
    panel: &PanelState2D,
    viewport: Rect,
) -> Vec2 {
    let fovy = panel.distance() * 2.0;
    let aspect = viewport.w / viewport.h;

    // Normalize mouse position relative to viewport center (-0.5 to 0.5)
    let norm_x = (mouse_coords.x - viewport.x) / viewport.w - 0.5;
    let norm_y = (mouse_coords.y - viewport.y) / viewport.h - 0.5;

    let world_x = -norm_x * fovy * aspect + panel.pan().x;
    let world_y = -norm_y * fovy + panel.pan().y; // Negative because screen Y is inverted

    vec2(world_x, world_y)
}
