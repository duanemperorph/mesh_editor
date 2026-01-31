//
// mesh_editor: custom 3d mesh editor
//

use macroquad::prelude::*;
use mesh_editor::mesh::{Mesh as MeshData, *};

mod editor_state;
use editor_state::*;

mod editor_panel_state;
use editor_panel_state::*;

mod insert_preview_state;
use insert_preview_state::*;

mod status_text;
use status_text::*;

mod panes;
use panes::*;

mod render_pane;
use render_pane::*;

mod global_commands;
use global_commands::*;

mod viewer_commands;
use viewer_commands::*;

#[macroquad::main("Mesh Editor")]
async fn main() {
    let mut current_mesh = MeshData::new_tapered_box();
    let mut editor_state = EditorState::new();

    loop {
        let panes = Panes::calc_from_screen_dims();

        handle_global_keyboard_commands(&mut editor_state);
        handle_viewer_commands(&mut editor_state, &panes);

        clear_background(BLACK);

        if *editor_state.viewer_mode() == ViewerMode::EditorPanels {
            render_editor_pane_viewport(
                editor_state.panel_state_xz(),
                &current_mesh,
                panes.left_viewport(),
            );
            render_editor_pane_viewport(
                editor_state.panel_state_yz(),
                &current_mesh,
                panes.top_right_viewport(),
            );
            render_editor_pane_viewport(
                editor_state.panel_state_xy(),
                &current_mesh,
                panes.bottom_right_viewport(),
            );
        } else {
            render_editor_pane_viewport(
                editor_state.panel_state_rotate_cam(),
                &current_mesh,
                panes.full_content_viewport(),
            );
        }

        if *editor_state.viewer_mode() == ViewerMode::EditorPanels {
            panes.draw_all_borders();
        } else {
            panes.draw_bottom_border();
        }
        draw_status_text(&editor_state, &current_mesh);

        next_frame().await
    }
}
