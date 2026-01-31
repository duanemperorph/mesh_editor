//
// mouse -> handle mouse input
//
use crate::editor_panel_state::*;
use crate::editor_state::*;
use crate::panes::*;
use macroquad::prelude::*;
use std::f32::consts::PI;

enum SelectedPanel<'a> {
    Panel2DView(&'a mut PanelState2D),
    PanelFreeCam(&'a mut PanelStateFreeCam),
    BottomBar,
}

type SelectedPanelInfo<'a> = (SelectedPanel<'a>, Rect);

pub fn handle_mouse_input<'a>(editor_state: &'a mut EditorState, panes: &Panes) {
    let current_mouse_coords = mouse_position().into();

    let Some((panel, viewport)) =
        get_panel_under_coords_mut(current_mouse_coords, editor_state, panes)
    else {
        return;
    };

    if let SelectedPanel::Panel2DView(panel) = panel {
        if is_mouse_button_down(MouseButton::Right) {
            handle_mouse_pan(panel, viewport);
        } else if is_mouse_button_pressed(MouseButton::Middle) {
            handle_reset_pan(panel);
        }
        handle_mouse_wheel_2d(panel);
    } else if let SelectedPanel::PanelFreeCam(panel) = panel {
        if is_mouse_button_down(MouseButton::Right) {
            handle_mouse_rotation(panel, viewport);
        } else if is_mouse_button_pressed(MouseButton::Middle) {
            handle_reset_free_cam(panel);
        }
        handle_mouse_wheel_free_cam(panel);
    }
}

fn handle_mouse_pan(panel: &mut PanelState2D, viewport: Rect) {
    let mouse_delta = mouse_delta_position();
    let pan_delta = screen_fraction_to_world_scale_vec2(mouse_delta, panel, viewport);
    *panel.pan_mut() += pan_delta;
}

fn handle_reset_pan(panel: &mut PanelState2D) {
    *panel.pan_mut() = vec2(0.0, 0.0);
    *panel.distance_mut() = 10.0;
}

fn handle_mouse_wheel_2d(panel: &mut PanelState2D) {
    let (_, wheel_y) = mouse_wheel();
    if wheel_y != 0.0 {
        *panel.distance_mut() -= wheel_y / 500.0;
    }
}

fn handle_mouse_rotation(panel: &mut PanelStateFreeCam, viewport: Rect) {
    let mouse_delta = mouse_delta_position();
    let rotation_delta = rotation_from_mouse_delta(mouse_delta, viewport);
    *panel.rotation_mut() += rotation_delta;
}

fn handle_mouse_wheel_free_cam(panel: &mut PanelStateFreeCam) {
    let (_, wheel_y) = mouse_wheel();
    if wheel_y != 0.0 {
        *panel.distance_mut() -= wheel_y / 500.0;
    }
}

fn handle_reset_free_cam(panel: &mut PanelStateFreeCam) {
    *panel.rotation_mut() = vec2(0.0, 0.0);
    *panel.distance_mut() = 10.0;
}

fn get_panel_under_coords_mut<'a>(
    coords: Vec2,
    editor_state: &'a mut EditorState,
    panes: &Panes,
) -> Option<SelectedPanelInfo<'a>> {
    let is_in_full_content_mode = *editor_state.viewer_mode() == ViewerMode::FreeCam;
    let Some(pane) = panes.get_pane_under_coords(coords, is_in_full_content_mode) else {
        return None;
    };

    let panel = match pane.pane_id {
        PaneId::FullContent => {
            SelectedPanel::PanelFreeCam(editor_state.panel_state_rotate_cam_mut())
        }
        PaneId::Left => SelectedPanel::Panel2DView(editor_state.panel_state_xz_mut()),
        PaneId::TopRight => SelectedPanel::Panel2DView(editor_state.panel_state_yz_mut()),
        PaneId::BottomRight => SelectedPanel::Panel2DView(editor_state.panel_state_xy_mut()),
        PaneId::BottomBar => SelectedPanel::BottomBar,
    };

    return Some((panel, pane.viewport_rect));
}

//
// Used by pan function
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

    let world_dx = -screen_delta.x * fovy * aspect / screen_width_fraction;
    let world_dy = -screen_delta.y * fovy / screen_height_fraction;

    return vec2(world_dx, world_dy);
}

pub fn rotation_from_mouse_delta(mouse_delta: Vec2, viewport: Rect) -> Vec2 {
    let screen_width_fraction = (viewport.w as f32) / screen_width();
    let screen_height_fraction = (viewport.h as f32) / screen_height();
    let rot_x = -mouse_delta.x / screen_width_fraction * PI;
    let rot_y = -mouse_delta.y / screen_height_fraction * PI / 2.0;
    return vec2(rot_x, rot_y);
}
