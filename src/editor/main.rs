//
// mesh_editor: custom 3d mesh editor
//

use macroquad::prelude::*;
use mesh_editor::mesh::{Mesh as MeshData, *};

mod editor_state;
use editor_state::*;

mod status_text;
use status_text::*;

mod panes;
use panes::*;

mod render_pane;
use render_pane::*;

mod keyboard;
use keyboard::*;

#[macroquad::main("Mesh Editor")]
async fn main() {
    let mut current_mesh = MeshData::new_cube();
    let mut editor_state = EditorState::new();

    loop {
        handle_key_presses(&mut editor_state);

        let panes = Panes::calc_from_screen_dims();

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
