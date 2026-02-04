//
// Mesh Vert Mutation
//

use super::{Axis, Mesh, VertIndex};
use macroquad::prelude::{Vec2, Vec3, vec2, vec3};

impl Mesh {
    pub fn mutate_verts(
        &mut self,
        vert_indicies: &[VertIndex],
        mutation: impl Fn(Vec3, Vec3) -> Vec3,
    ) {
        let vecs = self.selected_indicies_to_verts(vert_indicies);
        let center = vecs.iter().sum::<Vec3>() / vert_indicies.len() as f32;

        for (i, &vert_index) in vert_indicies.iter().enumerate() {
            let original_vec = vecs[i];
            let new_vec = mutation(original_vec, center);
            self.verticies[vert_index] = new_vec;
        }
    }

    pub fn translate_verts(&mut self, vert_indicies: &[VertIndex], delta: Vec3) {
        self.mutate_verts(vert_indicies, |v, _| v + delta);
    }

    pub fn scale_verts(&mut self, vert_indicies: &[VertIndex], factor: f32) {
        self.mutate_verts(vert_indicies, |v, center| v + (v - center) * factor);
    }

    pub fn rotate_verts(&mut self, vert_indicies: &[VertIndex], radians: f32, axis: Axis) {
        self.mutate_verts(vert_indicies, |v, center| {
            let d = v - center;
            let dv2 = Self::vec2_from_vec3_from_axis(d, axis);
            let dv2_rotated = Self::rotate_vec2(dv2, radians);
            let d_rotated = Self::vec3_from_vec2_from_axis(dv2_rotated, axis);
            return center + d_rotated;
        });
    }

    pub fn scale_verts_along_axis(&mut self, vert_indicies: &[VertIndex], scale: f32, axis: Axis) {
        self.mutate_verts(vert_indicies, |v, center| {
            let d = v - center;

            match axis {
                Axis::X => vec3(center.x + d.x * scale, v.y, v.z),
                Axis::Y => vec3(v.x, center.y + d.y * scale, v.z),
                Axis::Z => vec3(v.x, v.y, center.z + d.z * scale),
            }
        });
    }

    pub fn scale_verts_around_axis(&mut self, vert_indicies: &[VertIndex], scale: f32, axis: Axis) {
        self.mutate_verts(vert_indicies, |v, center| {
            let d = v - center;
            let dv2 = Self::vec2_from_vec3_from_axis(d, axis);
            let dv2_scaled = dv2 * scale;
            let d_scaled = Self::vec3_from_vec2_from_axis(dv2_scaled, axis);
            return center + d_scaled;
        });
    }

    pub fn scale_verts_3d(&mut self, vert_indicies: &[VertIndex], scale: f32) {
        self.mutate_verts(vert_indicies, |v, center| {
            let d = v - center;
            let d_scaled = d * scale;
            return center + d_scaled;
        })
    }

    fn vec2_from_vec3_from_axis(v3: Vec3, axis: Axis) -> Vec2 {
        match axis {
            Axis::X => vec2(v3.z, v3.y),
            Axis::Y => vec2(v3.x, v3.z),
            Axis::Z => vec2(v3.x, v3.y),
        }
    }

    fn vec3_from_vec2_from_axis(v2: Vec2, axis: Axis) -> Vec3 {
        match axis {
            Axis::X => Vec3::new(0.0, v2.y, v2.x),
            Axis::Y => Vec3::new(v2.x, 0.0, v2.y),
            Axis::Z => Vec3::new(v2.x, v2.y, 0.0),
        }
    }

    fn rotate_vec2(v: Vec2, radians: f32) -> Vec2 {
        Vec2::new(
            v.x * radians.cos() - v.y * radians.sin(),
            v.x * radians.sin() + v.y * radians.cos(),
        )
    }
}
