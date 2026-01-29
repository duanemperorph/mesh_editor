use macroquad::prelude::*;
use mesh_editor::editor_state::*;
use mesh_editor::mesh::{Mesh as MeshData, *};

mod status_text;
use status_text::*;

#[macroquad::main("Mesh Editor")]
async fn main() {
    let mut current_mesh = MeshData::new_cube();
    let mut editor_state = EditorState::new();

    loop {
        clear_background(BLACK);

        // Your rendering code here
        draw_status_text(&editor_state, &current_mesh);

        next_frame().await
    }
}
