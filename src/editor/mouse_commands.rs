//
// mouse -> handle mouse input
//
use crate::editor_panel_state::*;
use crate::editor_state::*;
use crate::insert_operation::*;
use crate::keyboard::check_modifier_keys;
use crate::panes::*;
use crate::screen_to_world::*;
use macroquad::prelude::*;
use mesh_editor::mesh::{Mesh as MeshData, *};
use std::f32::consts::PI;

#[derive(Copy, Clone)]
enum PanelSelector {
    Editor2D(PanelViewingPlane),
    FreeCam,
    BottomBar,
}

type SelectedPanelInfo = (PanelSelector, Rect);

pub fn handle_mouse_commands<'a>(
    editor_state_mut: &'a mut EditorState,
    mesh: &MeshData,
    panes: &Panes,
) {
    let current_mouse_coords = mouse_position().into();

    let Some((selector, viewport)) =
        get_panel_selector_under_coords(current_mouse_coords, editor_state_mut, panes)
    else {
        return;
    };

    if let PanelSelector::Editor2D(viewing_plane) = selector {
        if is_mouse_button_down(MouseButton::Right) {
            let panel_mut = editor_state_mut.panel_state_2d_from_plane_mut(viewing_plane);
            handle_mouse_pan(panel_mut, viewport);
        } else if is_mouse_button_pressed(MouseButton::Middle) {
            let panel_mut = editor_state_mut.panel_state_2d_from_plane_mut(viewing_plane);
            handle_reset_pan(panel_mut);
        } else if is_mouse_button_pressed(MouseButton::Left) {
            match editor_state_mut.input_mode() {
                InputMode::Select => handle_left_click_selection(
                    current_mouse_coords,
                    editor_state_mut,
                    mesh,
                    viewing_plane,
                    viewport,
                ),
                InputMode::Insert => handle_left_click_insert(
                    current_mouse_coords,
                    editor_state_mut,
                    mesh,
                    viewing_plane,
                    viewport,
                ),
                InputMode::Connect => handle_left_click_connect(
                    current_mouse_coords,
                    editor_state_mut,
                    mesh,
                    viewing_plane,
                    viewport,
                ),
                InputMode::Groups => handle_left_click_groups(
                    current_mouse_coords,
                    editor_state_mut,
                    mesh,
                    viewing_plane,
                    viewport,
                ),
                InputMode::Edit => handle_left_click_edit(
                    current_mouse_coords,
                    editor_state_mut,
                    mesh,
                    viewing_plane,
                    viewport,
                ),
            }
        }
        let panel_mut = editor_state_mut.panel_state_2d_from_plane_mut(viewing_plane);
        handle_mouse_wheel_2d(panel_mut);
    } else if let PanelSelector::FreeCam = selector {
        let free_cam_panel_mut = editor_state_mut.panel_state_free_cam_mut();

        if is_mouse_button_down(MouseButton::Right) {
            handle_mouse_rotation(free_cam_panel_mut, viewport);
        } else if is_mouse_button_pressed(MouseButton::Middle) {
            handle_reset_free_cam(free_cam_panel_mut);
        }
        handle_mouse_wheel_free_cam(free_cam_panel_mut);
    }
}

fn handle_left_click_selection(
    current_mouse_coords: Vec2,
    editor_state_mut: &mut EditorState,
    mesh: &MeshData,
    viewing_plane: PanelViewingPlane,
    viewport: Rect,
) {
    let panel = editor_state_mut.panel_state_2d_from_plane(viewing_plane);
    if let Some(index) = get_vert_index_under_mouse(current_mouse_coords, mesh, &panel, viewport) {
        let mod_keys = check_modifier_keys();
        let selection_mut = editor_state_mut.selection_mut();

        if mod_keys.shift_key {
            if let Some(&start_index) = selection_mut.selected_vert_indicies().first() {
                let verts_between = mesh.find_verts_between(start_index, index);
                selection_mut.replace_selected_vert_indicies(&verts_between);
            }
        } else if mod_keys.meta_key {
            selection_mut.toggle_selected_vert_index(index);
        } else {
            selection_mut.replace_selected_vert_indicies(&[index]);
        }
    }
}

fn handle_left_click_edit(
    current_mouse_coords: Vec2,
    editor_state_mut: &mut EditorState,
    mesh: &MeshData,
    viewing_plane: PanelViewingPlane,
    viewport: Rect,
) {
    //TODO: THIS
}

fn handle_left_click_insert(
    mouse_coord: Vec2,
    editor_state_mut: &mut EditorState,
    mesh: &MeshData,
    viewing_plane: PanelViewingPlane,
    viewport: Rect,
) {
    let panel = editor_state_mut.panel_state_2d_from_plane(viewing_plane);
    let world_coord = mouse_coord_to_world_coord_vec2(mouse_coord, &panel, viewport);

    let (start_vert_index, start_origin) = if let Some(&selected_index) =
        editor_state_mut.selection().selected_vert_indicies().last()
        && let Some(&selected_vert) = mesh.verts().get(selected_index)
    {
        (Some(selected_index), selected_vert)
    } else {
        (None, Vec3::ZERO)
    };

    let new_vert = match viewing_plane {
        PanelViewingPlane::XY => vec3(world_coord.x, world_coord.y, start_origin.z),
        PanelViewingPlane::XZ => vec3(world_coord.x, start_origin.y, world_coord.y),
        PanelViewingPlane::YZ => vec3(start_origin.x, world_coord.y, world_coord.x),
    };

    let op = InsertVertOperation::new(new_vert, start_vert_index);
    editor_state_mut.set_pending_insert_operation(InsertOperation::Vert(op));
}

fn handle_left_click_connect(
    current_mouse_coords: Vec2,
    editor_state_mut: &mut EditorState,
    mesh: &MeshData,
    viewing_plane: PanelViewingPlane,
    viewport: Rect,
) {
    let panel = editor_state_mut.panel_state_2d_from_plane(viewing_plane);
    let mod_keys = check_modifier_keys();

    let Some(clicked_index) =
        get_vert_index_under_mouse(current_mouse_coords, mesh, &panel, viewport)
    else {
        return;
    };

    let Some(&selected_index) = editor_state_mut.selection().selected_vert_indicies().last() else {
        return;
    };

    let completes_poly = !mod_keys.alt_key;
    let op = InsertLineOperation::new((clicked_index, selected_index), completes_poly);
    editor_state_mut.set_pending_insert_operation(InsertOperation::Line(op));
}

fn handle_left_click_groups(
    current_mouse_coords: Vec2,
    editor_state_mut: &mut EditorState,
    mesh: &MeshData,
    viewing_plane: PanelViewingPlane,
    viewport: Rect,
) {
    //TODO: THIS
}

fn handle_mouse_pan(panel: &mut PanelState2D, viewport: Rect) {
    let mut mouse_delta = mouse_delta_position();

    if panel.is_flipped() {
        mouse_delta.x = -mouse_delta.x;
    }

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

fn get_panel_selector_under_coords(
    coords: Vec2,
    editor_state: &EditorState,
    panes: &Panes,
) -> Option<SelectedPanelInfo> {
    let is_in_full_content_mode = *editor_state.viewer_mode() == ViewerMode::FreeCam;
    let Some(pane) = panes.get_pane_under_coords(coords, is_in_full_content_mode) else {
        return None;
    };

    let selector = match pane.pane_id {
        PaneId::FullContent => PanelSelector::FreeCam,
        PaneId::Left => PanelSelector::Editor2D(PanelViewingPlane::XZ),
        PaneId::TopRight => PanelSelector::Editor2D(PanelViewingPlane::YZ),
        PaneId::BottomRight => PanelSelector::Editor2D(PanelViewingPlane::XY),
        PaneId::BottomBar => PanelSelector::BottomBar,
    };

    return Some((selector, pane.viewport_rect));
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

    // println!("world_coord: {world_coord}");
    let found_verts = get_verts_from_mesh_near_coord(world_coord, panel.viewing_plane(), mesh);

    if panel.is_flipped() {
        found_verts.first().copied()
    } else {
        found_verts.last().copied()
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
