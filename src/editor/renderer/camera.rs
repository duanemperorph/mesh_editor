use crate::editor_panel_state::*;
use macroquad::prelude::*;

pub trait PanelCameraVectors {
    fn to_target_vec(&self) -> Vec3;
    fn to_camera_pos_vec(&self) -> Vec3;
    fn to_up_vec(&self) -> Vec3;
    fn to_model_rotation(&self) -> Vec3;
    fn distance(&self) -> f32;
}

impl PanelCameraVectors for PanelState2D {
    fn to_target_vec(&self) -> Vec3 {
        match self.viewing_plane() {
            PanelViewingPlane::XY => vec3(-self.pan().x, self.pan().y, 0.0),
            PanelViewingPlane::XZ => vec3(self.pan().x, 0.0, self.pan().y),
            PanelViewingPlane::YZ => vec3(0.0, self.pan().y, self.pan().x),
        }
    }

    fn to_camera_pos_vec(&self) -> Vec3 {
        let offset = self.distance() * if self.is_flipped() { -1.0 } else { 1.0 };
        match self.viewing_plane() {
            PanelViewingPlane::XY => vec3(-self.pan().x, self.pan().y, offset),
            PanelViewingPlane::XZ => vec3(self.pan().x, offset, self.pan().y),
            PanelViewingPlane::YZ => vec3(offset, self.pan().y, self.pan().x),
        }
    }

    fn to_up_vec(&self) -> Vec3 {
        match self.viewing_plane() {
            PanelViewingPlane::XY => vec3(0.0, 1.0, 0.0),
            PanelViewingPlane::XZ => vec3(0.0, 0.0, 1.0),
            PanelViewingPlane::YZ => vec3(0.0, 1.0, 0.0),
        }
    }

    fn to_model_rotation(&self) -> Vec3 {
        vec3(0.0, 0.0, 0.0)
    }

    fn distance(&self) -> f32 {
        PanelState2D::distance(self)
    }
}

impl PanelCameraVectors for PanelStateFreeCam {
    fn to_target_vec(&self) -> Vec3 {
        vec3(0.0, 0.0, 0.0)
    }

    fn to_camera_pos_vec(&self) -> Vec3 {
        // Fixed camera position along Z axis at distance
        vec3(0.0, 0.0, self.distance())
    }

    fn to_up_vec(&self) -> Vec3 {
        vec3(0.0, 1.0, 0.0)
    }

    fn to_model_rotation(&self) -> Vec3 {
        // rotation.x = yaw (Y axis), rotation.y = pitch (X axis)
        vec3(self.rotation().y, self.rotation().x, 0.0)
    }

    fn distance(&self) -> f32 {
        PanelStateFreeCam::distance(self)
    }
}
