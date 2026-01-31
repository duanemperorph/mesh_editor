//
// editor_panel_state -> panel-specific state for the editor
//

use macroquad::prelude::*;

pub trait PanelCameraVectors {
    fn to_target_vec(&self) -> Vec3;
    fn to_camera_pos_vec(&self) -> Vec3;
    fn to_up_vec(&self) -> Vec3;
    fn to_model_rotation(&self) -> Vec3;
}

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

impl PanelCameraVectors for PanelState2D {
    fn to_target_vec(&self) -> Vec3 {
        match self.viewing_plane {
            PanelViewingPlane::XY => vec3(self.pan.x, self.pan.y, 0.0),
            PanelViewingPlane::XZ => vec3(self.pan.x, 0.0, self.pan.y),
            PanelViewingPlane::YZ => vec3(0.0, self.pan.x, self.pan.y),
        }
    }

    fn to_camera_pos_vec(&self) -> Vec3 {
        let offset = self.distance * if self.is_flipped { -1.0 } else { 1.0 };
        match self.viewing_plane {
            PanelViewingPlane::XY => vec3(0.0, 0.0, offset),
            PanelViewingPlane::XZ => vec3(0.0, offset, 0.0),
            PanelViewingPlane::YZ => vec3(offset, 0.0, 0.0),
        }
    }

    fn to_up_vec(&self) -> Vec3 {
        match self.viewing_plane {
            PanelViewingPlane::XY => vec3(0.0, 1.0, 0.0),
            PanelViewingPlane::XZ => vec3(0.0, 0.0, 1.0),
            PanelViewingPlane::YZ => vec3(0.0, 1.0, 0.0),
        }
    }

    fn to_model_rotation(&self) -> Vec3 {
        return vec3(0.0, 0.0, 0.0);
    }
}

impl PanelCameraVectors for PanelStateFreeCam {
    fn to_target_vec(&self) -> Vec3 {
        vec3(0.0, 0.0, 0.0)
    }

    fn to_camera_pos_vec(&self) -> Vec3 {
        // Fixed camera position along Z axis at distance
        vec3(0.0, 0.0, self.distance)
    }

    fn to_up_vec(&self) -> Vec3 {
        vec3(0.0, 1.0, 0.0)
    }

    fn to_model_rotation(&self) -> Vec3 {
        // rotation.x = yaw (Y axis), rotation.y = pitch (X axis)
        vec3(self.rotation.y, self.rotation.x, 0.0)
    }
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
