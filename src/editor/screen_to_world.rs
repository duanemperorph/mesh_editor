//
// TODO: all this stuff for converting coords
//

fn screen_to_world_ortho(screen_pos: Vec2, viewport: Rect, panel: &PanelState2D) -> Vec3 {
    // Normalize to -1..1 within the viewport
    let ndc_x = ((screen_pos.x - viewport.x) / viewport.w) * 2.0 - 1.0;
    let ndc_y = 1.0 - ((screen_pos.y - viewport.y) / viewport.h) * 2.0; // flip y

    // Scale by camera distance (ortho zoom)
    let aspect = viewport.w / viewport.h;
    let world_x = ndc_x * panel.distance() * aspect + panel.pan().x;
    let world_y = ndc_y * panel.distance() + panel.pan().y;

    // Map to 3D based on viewing plane
    match panel.viewing_plane() {
        PanelViewingPlane::XY => vec3(world_x, world_y, 0.0),
        PanelViewingPlane::XZ => vec3(world_x, 0.0, world_y),
        PanelViewingPlane::YZ => vec3(0.0, world_x, world_y),
    }
}
