//
// status_text.rs -> draws the status text at the bottom
//

use crate::editor_state::*;
use crate::insert_operation::*;
use crate::selection::Selection;
use macroquad::prelude::*;
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
    let offset_incr = screen_width() / 3.0;
    let x_offset_0 = X_PADDING;
    let x_offset_1 = offset_incr;
    let x_offset_2 = offset_incr * 2.0;
    let y_offset = screen_height() - Y_PADDING;

    let input_mode_desc = format_input_mode(editor_state.input_mode());
    draw_text(&input_mode_desc, x_offset_0, y_offset, TEXT_HEIGHT, WHITE);

    if let InsertOperation::Vert(vert_op) = editor_state.pending_insert_operation() {
        let insert_preview_desc = format!("Vert: {}", vert_op.new_vert);
        draw_text(
            &insert_preview_desc,
            x_offset_1,
            y_offset,
            TEXT_HEIGHT,
            WHITE,
        );
    } else if let InsertOperation::Line(_) = editor_state.pending_insert_operation() {
        let insert_preview_desc = "Ins: Line";
        draw_text(
            &insert_preview_desc,
            x_offset_1,
            y_offset,
            TEXT_HEIGHT,
            WHITE,
        );
    } else {
        let selection_desc = format_selection(editor_state.selection(), mesh);
        draw_text(&selection_desc, x_offset_1, y_offset, TEXT_HEIGHT, WHITE);
    }

    let mirror_desc = format_mirror(mesh);
    draw_text(&mirror_desc, x_offset_2, y_offset, TEXT_HEIGHT, WHITE);
}

fn format_input_mode(input_mode: &InputMode) -> String {
    format!("Mode: {}", input_mode)
}

fn format_selection(selection: &Selection, mesh: &MeshData) -> String {
    let verts = selection.selected_vert_indicies();

    if verts.len() == 1 {
        let vert = mesh.verts()[0];
        format!("Sel: {}", vert)
    } else {
        format!("Sel: Verts ({})", verts.len())
    }
}

fn format_mirror(mesh: &MeshData) -> String {
    format!("Mir: {}", mesh.mirror_mode())
}
