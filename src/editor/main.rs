//
// mesh_editor: custom 3d mesh editor
//

use clap::Parser;
use macroquad::prelude::*;
use std::process;

mod editor_state;
use editor_state::*;

mod editor_panel_state;

mod insert_operation;

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

mod mesh_document;

mod cli;
use cli::Cli;

#[macroquad::main("Mesh Editor")]
async fn main() {
    let cli = Cli::parse();

    let mut document = match cli.load_document() {
        Ok(doc) => doc,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
    let mut editor_state = EditorState::new();

    loop {
        let panes = Panes::calc_from_screen_dims();

        handle_keyboard_commands(&mut editor_state, &mut document);
        handle_mouse_commands(&mut editor_state, document.current_mesh(), &panes);

        clear_background(BLACK);

        if *editor_state.viewer_mode() == ViewerMode::EditorPanels {
            render_editor_pane_viewport(
                editor_state.panel_state_xz(),
                document.current_mesh(),
                &editor_state.selection(),
                panes.left_viewport(),
            );
            render_editor_pane_viewport(
                editor_state.panel_state_yz(),
                document.current_mesh(),
                &editor_state.selection(),
                panes.top_right_viewport(),
            );
            render_editor_pane_viewport(
                editor_state.panel_state_xy(),
                document.current_mesh(),
                &editor_state.selection(),
                panes.bottom_right_viewport(),
            );
        } else {
            render_editor_pane_viewport(
                editor_state.panel_state_free_cam(),
                document.current_mesh(),
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
        draw_status_text(&editor_state, document.current_mesh());

        next_frame().await
    }
}
