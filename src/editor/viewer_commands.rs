//
// mouse -> handle mouse input
//
use crate::editor_panel_state::*;
use crate::editor_state::*;
use crate::panes::*;
use crate::screen_to_world::*;
use macroquad::prelude::*;
use mesh_editor::mesh::{Mesh as MeshData, *};
use std::f32::consts::PI;

enum SelectedPanel<'a> {
    Panel2DView(&'a mut PanelState2D),
    PanelFreeCam(&'a mut PanelStateFreeCam),
    BottomBar,
}

type SelectedPanelInfo<'a> = (SelectedPanel<'a>, Rect);

pub fn handle_viewer_commands<'a>(
    editor_state: &'a mut EditorState,
    mesh: &MeshData,
    panes: &Panes,
) {
    let current_mouse_coords = mouse_position().into();

    let Some((panel, viewport)) =
        get_panel_under_coords_mut(current_mouse_coords, editor_state, panes)
    else {
        return;
    };

    if let SelectedPanel::Panel2DView(panel) = panel {
        if is_mouse_button_down(MouseButton::Right) {
            handle_mouse_pan(panel, viewport);
        } else if is_mouse_button_pressed(MouseButton::Middle) {
            handle_reset_pan(panel);
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let panel = *panel;
            if let Some(index) =
                get_vert_index_under_mouse(current_mouse_coords, mesh, &panel, viewport)
            {
                editor_state
                    .selection_mut()
                    .toggle_selected_vert_index(index);
            }
        }
        handle_mouse_wheel_2d(panel);
    } else if let SelectedPanel::PanelFreeCam(panel) = panel {
        if is_mouse_button_down(MouseButton::Right) {
            handle_mouse_rotation(panel, viewport);
        } else if is_mouse_button_pressed(MouseButton::Middle) {
            handle_reset_free_cam(panel);
        }
        handle_mouse_wheel_free_cam(panel);
    }
}

fn handle_mouse_pan(panel: &mut PanelState2D, viewport: Rect) {
    let mouse_delta = mouse_delta_position();
    let pan_delta = mouse_delta_to_world_scale_vec2(mouse_delta, panel, viewport);
    *panel.pan_mut() += pan_delta;
}

fn handle_reset_pan(panel: &mut PanelState2D) {
    *panel.pan_mut() = vec2(0.0, 0.0);
    *panel.distance_mut() = 10.0;
}

fn handle_mouse_wheel_2d(panel: &mut PanelState2D) {
    let (_, wheel_y) = mouse_wheel();
    if wheel_y != 0.0 {
        *panel.distance_mut() -= wheel_y / 500.0;
    }
}

fn handle_mouse_rotation(panel: &mut PanelStateFreeCam, viewport: Rect) {
    let mouse_delta = mouse_delta_position();
    let rotation_delta = rotation_from_mouse_delta(mouse_delta, viewport);
    *panel.rotation_mut() += rotation_delta;
}

fn handle_mouse_wheel_free_cam(panel: &mut PanelStateFreeCam) {
    let (_, wheel_y) = mouse_wheel();
    if wheel_y != 0.0 {
        *panel.distance_mut() -= wheel_y / 500.0;
    }
}

fn handle_reset_free_cam(panel: &mut PanelStateFreeCam) {
    *panel.rotation_mut() = vec2(0.0, 0.0);
    *panel.distance_mut() = 10.0;
}

fn get_panel_under_coords_mut<'a>(
    coords: Vec2,
    editor_state: &'a mut EditorState,
    panes: &Panes,
) -> Option<SelectedPanelInfo<'a>> {
    let is_in_full_content_mode = *editor_state.viewer_mode() == ViewerMode::FreeCam;
    let Some(pane) = panes.get_pane_under_coords(coords, is_in_full_content_mode) else {
        return None;
    };

    let panel = match pane.pane_id {
        PaneId::FullContent => {
            SelectedPanel::PanelFreeCam(editor_state.panel_state_rotate_cam_mut())
        }
        PaneId::Left => SelectedPanel::Panel2DView(editor_state.panel_state_xz_mut()),
        PaneId::TopRight => SelectedPanel::Panel2DView(editor_state.panel_state_yz_mut()),
        PaneId::BottomRight => SelectedPanel::Panel2DView(editor_state.panel_state_xy_mut()),
        PaneId::BottomBar => SelectedPanel::BottomBar,
    };

    return Some((panel, pane.viewport_rect));
}

pub fn rotation_from_mouse_delta(mouse_delta: Vec2, viewport: Rect) -> Vec2 {
    let screen_width_fraction = (viewport.w as f32) / screen_width();
    let screen_height_fraction = (viewport.h as f32) / screen_height();
    let rot_x = -mouse_delta.x / screen_width_fraction * PI;
    let rot_y = -mouse_delta.y / screen_height_fraction * PI / 2.0;
    return vec2(rot_x, rot_y);
}

fn get_vert_index_under_mouse(
    mouse_coord: Vec2,
    mesh: &MeshData,
    panel: &PanelState2D,
    viewport: Rect,
) -> Option<VertIndex> {
    let world_coord = mouse_coord_to_world_coord_vec2(mouse_coord, &panel, viewport);
    println!("world_coord: {world_coord}");
    let found_verts = get_verts_from_mesh_near_coord(world_coord, panel.viewing_plane(), mesh);
    found_verts.get(0).copied()
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
