//
// status_text.rs -> draws the status text at the bottom
//

use macroquad::prelude::*;
use mesh_editor::editor_state::*;
use mesh_editor::mesh::{Mesh as MeshData, *};

pub fn draw_status_text(editor_state: &EditorState, mesh: &MeshData) {
    // editor state to display
    // selection (single point / other)
    // input mode (debug)
    // input preview mode
    // mirroring mode
    const TEXT_HEIGHT: f32 = 16.0;
    const Y_PADDING: f32 = 6.0;
    const X_PADDING: f32 = 20.0;
    let x_offset_0 = X_PADDING;
    let x_offset_1 = screen_width() / 2.0;
    let y_offset = screen_height() - Y_PADDING;

    if let Some(_) = editor_state.insert_preview().vert() {
        let insert_preview_desc = format_insert(editor_state.insert_preview());
        draw_text(
            &insert_preview_desc,
            x_offset_0,
            y_offset,
            TEXT_HEIGHT,
            WHITE,
        );
    } else {
        let selection_desc = format_selection(editor_state.selection(), mesh);
        draw_text(&selection_desc, x_offset_0, y_offset, TEXT_HEIGHT, WHITE);
    }

    let mirror_desc = format_mirror(mesh);
    draw_text(&mirror_desc, x_offset_1, y_offset, TEXT_HEIGHT, WHITE);
}

fn format_selection(selection: &Selection, mesh: &MeshData) -> String {
    match selection {
        Selection::None => format!("Sel: None"),
        Selection::Verticies(verts) if verts.len() == 1 => {
            let vert = mesh.verts()[0];
            format!("Sel: {}", vert)
        }
        Selection::Verticies(verts) => format!("Sel: Verts ({})", verts.len()),
        Selection::Lines(lines) => format!("Sel: Lines ({})", lines.len()),
        Selection::Polys(polys) => format!("Sel: Polys ({})", polys.len()),
    }
}

fn format_insert(insert_preview: &InsertPreview) -> String {
    let Some(vert) = insert_preview.vert() else {
        return String::from("");
    };
    format!("Ins: {}", vert)
}

fn format_mirror(mesh: &MeshData) -> String {
    format!("Mir: {}", mesh.mirror_mode())
}
