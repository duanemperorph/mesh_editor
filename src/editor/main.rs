//
// mesh_editor: custom 3d mesh editor
//

use macroquad::prelude::*;
use mesh_editor::editor_state::*;
use mesh_editor::mesh::{Mesh as MeshData, *};

mod status_text;
use status_text::*;

mod panes;
use panes::*;

#[macroquad::main("Mesh Editor")]
async fn main() {
    let mut current_mesh = MeshData::new_cube();
    let mut editor_state = EditorState::new();

    loop {
        let panes = Panes::calc_from_screen_dims();

        clear_background(BLACK);

        panes.draw_borders();
        draw_status_text(&editor_state, &current_mesh);

        next_frame().await
    }
}
