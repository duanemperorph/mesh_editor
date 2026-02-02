//
// panel-ui-controls -> ui controls over each panel
//
use crate::editor_panel_state::*;
use crate::editor_state::*;
use crate::panes::Panes;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets::Button};

pub fn add_panel_ui_controls(editor_state: &mut EditorState, panes: &Panes) {
    set_default_camera(); // Required for UI to render in screen space

    // Left pane (XZ) - bottom-right corner
    draw_pane_buttons(
        editor_state.panel_state_xz_mut(),
        panes.left_rect(),
        hash!("xz"),
    );

    // Top-right pane (YZ)
    draw_pane_buttons(
        editor_state.panel_state_yz_mut(),
        panes.top_right_rect(),
        hash!("yz"),
    );

    // Bottom-right pane (XY)
    draw_pane_buttons(
        editor_state.panel_state_xy_mut(),
        panes.bottom_right_rect(),
        hash!("xy"),
    );
}

fn draw_pane_buttons(panel_state: &mut PanelState2D, pane_rect: Rect, id: u64) {
    const BUTTON_WIDTH: f32 = 20.0;
    const BUTTON_HEIGHT: f32 = 20.0;
    const PADDING: f32 = 5.0;

    let btn_x = pane_rect.x + pane_rect.w - BUTTON_WIDTH - PADDING;
    let btn_y = pane_rect.y + pane_rect.h - BUTTON_HEIGHT - PADDING;

    let label = if panel_state.is_flipped() { "-" } else { "+" };

    if Button::new(label)
        .position(vec2(btn_x, btn_y))
        .size(vec2(20.0, 20.0)) // fixed width/height
        .ui(&mut root_ui())
    {
        *panel_state.is_flipped_mut() = !panel_state.is_flipped();
    }
}
