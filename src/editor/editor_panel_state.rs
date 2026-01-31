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

    pub fn pan(&self) -> Vec2 {
        self.pan
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        self.is_flipped = flipped;
    }

    pub fn set_pan(&mut self, pan: Vec2) {
        self.pan = pan;
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
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

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn set_rotation(&mut self, rotation: Vec2) {
        self.rotation = rotation;
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
    }
}
