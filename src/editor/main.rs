//
// mesh_editor: custom 3d mesh editor
//

use macroquad::prelude::*;
use mesh_editor::mesh::{Mesh as MeshData, *};

mod editor_state;
use editor_state::*;

mod editor_panel_state;

mod insert_preview_state;

mod status_bar;
use status_bar::*;

mod panes;
use panes::*;

mod render_pane;
use render_pane::*;

mod keyboard;

mod mouse_commands;
use mouse_commands::*;

mod screen_to_world;

mod selection;

mod keyboard_commands;
use keyboard_commands::*;

mod panel_ui_controls;
use panel_ui_controls::*;

#[macroquad::main("Mesh Editor")]
async fn main() {
    let mut current_mesh = MeshData::new_tapered_box();
    let mut editor_state = EditorState::new();

    loop {
        let panes = Panes::calc_from_screen_dims();

        handle_keyboard_commands(&mut editor_state, &mut current_mesh);
        handle_mouse_commands(&mut editor_state, &current_mesh, &panes);

        clear_background(BLACK);

        if *editor_state.viewer_mode() == ViewerMode::EditorPanels {
            render_editor_pane_viewport(
                editor_state.panel_state_xz(),
                &current_mesh,
                &editor_state.selection(),
                panes.left_viewport(),
            );
            render_editor_pane_viewport(
                editor_state.panel_state_yz(),
                &current_mesh,
                &editor_state.selection(),
                panes.top_right_viewport(),
            );
            render_editor_pane_viewport(
                editor_state.panel_state_xy(),
                &current_mesh,
                &editor_state.selection(),
                panes.bottom_right_viewport(),
            );
        } else {
            render_editor_pane_viewport(
                editor_state.panel_state_free_cam(),
                &current_mesh,
                &editor_state.selection(),
                panes.full_content_viewport(),
            );
        }

        if *editor_state.viewer_mode() == ViewerMode::EditorPanels {
            panes.draw_all_borders();
        } else {
            panes.draw_bottom_border();
        }
        add_panel_ui_controls(&mut editor_state, &panes);
        draw_status_text(&editor_state, &current_mesh);

        next_frame().await
    }
}
