//
// mouse -> handle mouse input
//
use crate::editor_panel_state::*;
use crate::editor_state::*;
use crate::panes::*;
use crate::screen_to_world::*;
use macroquad::prelude::*;

enum SelectedPanel<'a> {
    Panel2DView(&'a mut PanelState2D),
    PanelFreeCam(&'a mut PanelStateFreeCam),
    BottomBar,
}

type SelectedPanelInfo<'a> = (SelectedPanel<'a>, Rect);

pub fn handle_mouse_input<'a>(editor_state: &'a mut EditorState, panes: &Panes) {
    let current_mouse_coords = mouse_position().into();
    // println!("mouse at: {}", current_mouse_coords);
    let Some((mut panel, viewport)) =
        get_panel_under_coords_mut(current_mouse_coords, editor_state, panes)
    else {
        return;
    };

    // handle mouse events and stuff
    if is_mouse_button_down(MouseButton::Right) {
        handle_mouse_pan(&mut panel, viewport);
    }
}

fn handle_mouse_pan(panel: &mut SelectedPanel, viewport: Rect) {
    let SelectedPanel::Panel2DView(panel) = panel else {
        return;
    };
    let mouse_delta = mouse_delta_position();
    let pan_delta = screen_fraction_to_world_scale_vec2(mouse_delta, panel, viewport);
    *panel.pan_mut() += pan_delta;
    let new_pan = panel.pan();
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

    // println!("Got pane: {}", pane.pane_id);

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
