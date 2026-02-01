//
// viewer_selection - Handle selection in any of the "2D" editor panes
//
use crate::editor_panel_state::*;
use crate::editor_state::*;
use crate::panes::*;
use crate::screen_to_world::*;
use macroquad::prelude::*;
use mesh_editor::mesh::{Mesh as MeshData, *};

pub fn select_point_under_mouse(
    mouse_coord: Vec2,
    mesh: &MeshData,
    panel: &PanelState2D,
    viewport: Rect,
) {
    let world_coord = mouse_coord_to_world_coord_vec2(mouse_coord, &panel, viewport);
    println!("world_coord: {world_coord}");
    let found_verts = get_verts_from_mesh_near_coord(world_coord, panel.viewing_plane(), mesh);

    if let Some(vert) = found_verts.get(0) {
        println!("Found vert: {}", vert)
    } else {
        println!("No vert found.")
    }
}

fn get_verts_from_mesh_near_coord(
    world_coord: Vec2,
    viewing_plane: PanelViewingPlane,
    mesh: &MeshData,
) -> Vec<VertIndex> {
    let search_radius = 0.1;

    match viewing_plane {
        PanelViewingPlane::XY => mesh.find_verts_xy(world_coord, search_radius),
        PanelViewingPlane::XZ => mesh.find_verts_xz(world_coord, search_radius),
        PanelViewingPlane::YZ => mesh.find_verts_yz(world_coord.yx(), search_radius),
    }
}
