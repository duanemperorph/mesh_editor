//
// mouse -> handle mouse input
//
use crate::editor_panel_state::*;
use crate::editor_state::*;
use crate::panes::*;
use macroquad::prelude::*;

enum SelectedPanel<'a> {
    Panel2DView(&'a mut PanelState2D),
    PanelFreeCam(&'a mut PanelStateFreeCam),
    BottomBar,
}

pub fn handle_mouse_input<'a>(editor_state: &'a mut EditorState, panes: &Panes) {
    // handle mouse events and stuff
    let panel = get_panel_under_coords_mut(vec2(1.0, 1.0), editor_state, panes) else {
        println!("Failed to get the panel under mouse ");
        return;
    };
}

fn get_panel_under_coords_mut<'a>(
    coords: Vec2,
    editor_state: &'a mut EditorState,
    panes: &Panes,
) -> Option<SelectedPanel<'a>> {
    let is_in_full_content_mode = *editor_state.viewer_mode() == ViewerMode::FreeCam;
    let Some(pane) = panes.get_pane_under_coords(coords, is_in_full_content_mode) else {
        return None;
    };

    let panel = match pane {
        PaneId::FullContent => {
            SelectedPanel::PanelFreeCam(editor_state.panel_state_rotate_cam_mut())
        }
        PaneId::Left => SelectedPanel::Panel2DView(editor_state.panel_state_xz_mut()),
        PaneId::TopRight => SelectedPanel::Panel2DView(editor_state.panel_state_yz_mut()),
        PaneId::BottomRight => SelectedPanel::Panel2DView(editor_state.panel_state_xy_mut()),
        PaneId::BottomBar => SelectedPanel::BottomBar,
    };

    return Some(panel);
}
