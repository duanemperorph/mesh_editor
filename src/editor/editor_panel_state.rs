//
// editor_panel_state -> panel-specific state for the editor
//

use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub enum PanelViewingPlane {
    XZ,
    YZ,
    XY,
}

#[derive(Copy, Clone)]
pub struct PanelState2D {
    viewing_plane: PanelViewingPlane,
    is_flipped: bool,
    pan: Vec2,
    distance: f32,
}

#[derive(Copy, Clone)]
pub struct PanelStateFreeCam {
    rotation: Vec2,
    distance: f32,
}

impl PanelState2D {
    pub fn new(viewing_plane: PanelViewingPlane) -> PanelState2D {
        PanelState2D {
            is_flipped: false,
            pan: vec2(0.0, 0.0),
            distance: 5.0,
            viewing_plane,
        }
    }

    pub fn viewing_plane(&self) -> PanelViewingPlane {
        return self.viewing_plane;
    }

    pub fn is_flipped(&self) -> bool {
        self.is_flipped
    }

    pub fn is_flipped_mut(&mut self) -> &mut bool {
        &mut self.is_flipped
    }

    pub fn pan(&self) -> Vec2 {
        self.pan
    }

    pub fn pan_mut(&mut self) -> &mut Vec2 {
        &mut self.pan
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn distance_mut(&mut self) -> &mut f32 {
        &mut self.distance
    }
}

impl PanelStateFreeCam {
    pub fn new() -> PanelStateFreeCam {
        PanelStateFreeCam {
            rotation: vec2(0.0, 0.0),
            distance: 10.0,
        }
    }

    pub fn rotation(&self) -> Vec2 {
        self.rotation
    }

    pub fn rotation_mut(&mut self) -> &mut Vec2 {
        &mut self.rotation
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn distance_mut(&mut self) -> &mut f32 {
        &mut self.distance
    }
}
