//
// mesh -> custom mesh format
//

use crate::mesh_bfs::mesh_bfs;
pub use crate::mesh_types::*;
use itertools::Itertools;
use macroquad::prelude::*;
use std::collections::HashSet;
use std::fmt;

#[derive(Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub enum MirrorMode {
    None,
    Bilateral,
    Radial(u8),
}

pub struct Mesh {
    mirror_mode: MirrorMode,
    verticies: Vec<Vec3>,
    lines: Vec<Line>,
    polys: Vec<Poly>,
}

impl Mesh {
    pub fn new() -> Mesh {
        return Mesh {
            mirror_mode: MirrorMode::None,
            verticies: Vec::new(),
            lines: Vec::new(),
            polys: Vec::new(),
        };
    }

    pub fn verts(&self) -> &Vec<Vec3> {
        &self.verticies
    }

    pub fn lines(&self) -> &Vec<Line> {
        &self.lines
    }

    pub fn polys(&self) -> &Vec<Poly> {
        &self.polys
    }

    pub fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    pub fn set_mirror_mode(&mut self, mode: MirrorMode) {
        self.mirror_mode = mode;
    }

    pub fn add_vert(&mut self, coord: Vec3) -> VertIndex {
        self.verticies.push(coord);
        return self.verticies.len() - 1;
    }

    pub fn update_vert(&mut self, index: VertIndex, coord: Vec3) -> Option<()> {
        *self.verticies.get_mut(index as usize)? = coord;
        return Some(());
    }

    pub fn delete_vert(&mut self, index: VertIndex) -> Option<Vec3> {
        if index >= self.verticies.len() {
            return None;
        }
        let last_vert_index = self.verticies.len() - 1;
        let removed_value = self.verticies.swap_remove(index);
        self.remove_lines_containing_vert(index);
        if index != last_vert_index {
            self.remap_swaped_vertex_indicies(last_vert_index, index);
        }
        self.cleanup_polys_after_point_removal(index, last_vert_index);
        return Some(removed_value);
    }

    pub fn add_line(&mut self, line: Line) -> Option<()> {
        self.validate_line_indicies(line)?;
        self.lines.push(line);
        return Some(());
    }

    pub fn add_poly(&mut self, poly: Poly) -> Option<()> {
        self.validate_poly_indicies(&poly)?;
        self.polys.push(poly);
        return Some(());
    }

    pub fn remove_line(&mut self, index: LineIndex) -> Option<Line> {
        if index >= self.lines.len() {
            return None;
        }
        return Some(self.lines.swap_remove(index));
    }

    pub fn remove_poly(&mut self, index: PolyIndex) -> Option<Poly> {
        if index >= self.polys.len() {
            return None;
        }
        return Some(self.polys.swap_remove(index));
    }

    pub fn selected_indicies_to_verts(&self, indicies: &[VertIndex]) -> Vec<Vec3> {
        self.verticies
            .iter()
            .enumerate()
            .filter(|(i, _)| indicies.contains(i))
            .map(|(i, v)| v)
            .copied()
            .collect()
    }

    pub fn lines_to_vert_pairs(&self) -> Vec<(Vec3, Vec3)> {
        return self
            .lines
            .iter()
            .map(|line| {
                let v1 = *self.verticies.get(line.0).unwrap_or(&Vec3::ZERO);
                let v2 = *self.verticies.get(line.1).unwrap_or(&Vec3::ZERO);
                (v1, v2)
            })
            .collect();
    }

    pub fn lines_to_vert_pairs_from_list(&self, lines: &[Line]) -> Vec<(Vec3, Vec3)> {
        lines
            .iter()
            .map(|line| {
                let v1 = *self.verticies.get(line.0).unwrap_or(&Vec3::ZERO);
                let v2 = *self.verticies.get(line.1).unwrap_or(&Vec3::ZERO);
                (v1, v2)
            })
            .collect()
    }

    pub fn polys_to_triangle_indicies(&self) -> Vec<VertIndex> {
        self.polys
            .iter()
            .map(|poly| Self::poly_indicies_to_triangle_indicies(poly))
            .flatten()
            .collect()
    }

    pub fn selected_polys_to_triangle_indicies(
        &self,
        selected_poly_indicies: &HashSet<PolyIndex>,
    ) -> Vec<VertIndex> {
        self.polys
            .iter()
            .enumerate()
            .filter(|(i, _)| selected_poly_indicies.contains(i))
            .map(|(_, p)| Self::poly_indicies_to_triangle_indicies(p))
            .flatten()
            .collect()
    }

    //
    // uses triangles fan to get indicies for each triangles
    //
    pub fn poly_indicies_to_triangle_indicies(poly: &Poly) -> Vec<VertIndex> {
        if poly.len() < 3 {
            return vec![];
        }

        let v0 = poly[0];

        poly.windows(2)
            .skip(1)
            .map(|slice| {
                let v1 = slice[0];
                let v2 = slice[1];
                [v0, v1, v2]
            })
            .flatten()
            .collect()
    }

    fn remove_lines_containing_vert(&mut self, vert_index: VertIndex) {
        let line_indicies_to_remove: Vec<LineIndex> = self
            .lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.0 == vert_index || line.1 == vert_index)
            .map(|(i, _)| i)
            .collect();

        for line_index in line_indicies_to_remove.into_iter().rev() {
            self.remove_line(line_index);
        }
    }

    fn remap_swaped_vertex_indicies(&mut self, old_index: VertIndex, new_index: VertIndex) {
        for line in self.lines.iter_mut() {
            if line.0 == old_index {
                line.0 = new_index;
            }
            if line.1 == old_index {
                line.1 = new_index;
            }
        }
    }

    fn validate_line_indicies(&self, line: Line) -> Option<()> {
        _ = self.verticies.get(line.0)?;
        _ = self.verticies.get(line.1)?;
        return Some(());
    }

    fn validate_poly_indicies(&self, poly: &Poly) -> Option<()> {
        if poly.len() < 3 {
            return None;
        }
        if Self::check_for_poly_dup_indicies(poly) {
            return None;
        }
        poly.iter()
            .all(|&vert_index| self.verticies.get(vert_index).is_some())
            .then_some(())
    }

    fn check_for_poly_dup_indicies(poly: &Poly) -> bool {
        let mut seen = HashSet::new();
        !poly.iter().all(|i| seen.insert(i))
    }

    fn cleanup_polys_after_point_removal(
        &mut self,
        removed_vert_index: VertIndex,
        replaced_vert_index: VertIndex,
    ) {
        let poly_iter = self.polys.iter_mut();

        for poly in poly_iter {
            poly.retain(|&vert_index| vert_index != removed_vert_index);

            for vert_index in poly.iter_mut() {
                // acount for remapping caused by swap_remove
                if *vert_index == replaced_vert_index {
                    *vert_index = removed_vert_index;
                }
            }
        }

        self.polys.retain(|poly| poly.len() > 2);
    }
}

//
// Mesh Vert Mutation
//

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
            let dv2 = Self::vec2_from_vec3_for_rotation(d, axis);
            let dv2_rotated = Self::rotate_vec2(dv2, radians);
            let d_rotated = Self::vec3_from_vec2_for_rotation(dv2_rotated, axis);
            return center + d_rotated;
        });
    }

    fn vec2_from_vec3_for_rotation(v3: Vec3, axis: Axis) -> Vec2 {
        match axis {
            Axis::X => vec2(v3.z, v3.y),
            Axis::Y => vec2(v3.x, v3.z),
            Axis::Z => vec2(v3.x, v3.y),
        }
    }

    fn vec3_from_vec2_for_rotation(v2: Vec2, axis: Axis) -> Vec3 {
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

//
// Addative mutations (split)
//

impl Mesh {
    pub fn split_lines(&mut self, line_indicies: &HashSet<LineIndex>) {
        let lines_ordered: Vec<Line> = self
            .lines
            .iter()
            .enumerate()
            .filter(|(i, _)| line_indicies.contains(i))
            .map(|(_, &l)| l)
            .collect();

        let center_vecs: Vec<Vec3> = self
            .lines_to_vert_pairs_from_list(&lines_ordered)
            .iter()
            .map(|&(v1, v2)| (v1 + v2) / 2.0)
            .collect();

        self.verticies.extend(center_vecs);

        // remove old lines
        self.lines = self
            .lines
            .drain(..)
            .into_iter()
            .enumerate()
            .filter(|(i, _)| line_indicies.contains(i))
            .map(|(_, l)| l)
            .collect();

        // add new lines and update polys
        let new_verts_start_index = self.verticies.len();

        for (i, old_line) in lines_ordered.iter().enumerate() {
            let new_vert_index = i + new_verts_start_index;
            let new_line_1 = (old_line.0, new_vert_index);
            self.lines.push(new_line_1);
            let new_line_2 = (new_vert_index, old_line.1);
            self.lines.push(new_line_2);
            self.update_polys_with_new_vert_between_old_verts(old_line, new_vert_index);
        }
    }

    fn update_polys_with_new_vert_between_old_verts(
        &mut self,
        old_verts: &Line,
        new_vert: VertIndex,
    ) {
        for poly in self.polys.iter_mut() {
            if poly.len() < 3 {
                continue;
            }
            let first_vert_index = poly.first().unwrap();
            let last_vert_index = poly.last().unwrap();
            let wrap_around_pair = [*last_vert_index, *first_vert_index];
            let index = poly
                .windows(2)
                .chain(std::iter::once(wrap_around_pair.as_slice()))
                .position(|pair| {
                    (pair[0] == old_verts.0 && pair[1] == old_verts.1)
                        || (pair[0] == old_verts.1 && pair[1] == old_verts.0)
                });

            if let Some(index) = index {
                poly.insert(index, new_vert);
            }
        }
    }
}

//
// Formatting
//

impl fmt::Display for MirrorMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(formatter, "None"),
            Self::Bilateral => write!(formatter, "Bilateral"),
            Self::Radial(axes) => write!(formatter, "Radial({})", axes),
        }
    }
}

//
// Test data
//

impl Mesh {
    pub fn new_cube() -> Mesh {
        let mut mesh = Mesh::new();

        // 8 vertices of a unit cube centered at origin
        mesh.add_vert(Vec3::new(-1.0, -1.0, -1.0)); // 0: front bottom left
        mesh.add_vert(Vec3::new(1.0, -1.0, -1.0)); // 1: front bottom right
        mesh.add_vert(Vec3::new(1.0, 1.0, -1.0)); // 2: front top right
        mesh.add_vert(Vec3::new(-1.0, 1.0, -1.0)); // 3: front top left
        mesh.add_vert(Vec3::new(-1.0, -1.0, 1.0)); // 4: back bottom left
        mesh.add_vert(Vec3::new(1.0, -1.0, 1.0)); // 5: back bottom right
        mesh.add_vert(Vec3::new(1.0, 1.0, 1.0)); // 6: back top right
        mesh.add_vert(Vec3::new(-1.0, 1.0, 1.0)); // 7: back top left

        add_cube_lines_and_faces(&mut mesh);
        mesh
    }

    pub fn new_tapered_box() -> Mesh {
        let mut mesh = Mesh::new();

        // 8 vertices of a tapered box (0.5 tall, 4.0 long in z, top face smaller)
        mesh.add_vert(Vec3::new(-1.0, -0.25, -2.0)); // 0: front bottom left
        mesh.add_vert(Vec3::new(1.0, -0.25, -2.0)); // 1: front bottom right
        mesh.add_vert(Vec3::new(0.8, 0.25, -1.6)); // 2: front top right
        mesh.add_vert(Vec3::new(-0.8, 0.25, -1.6)); // 3: front top left
        mesh.add_vert(Vec3::new(-1.0, -0.25, 2.0)); // 4: back bottom left
        mesh.add_vert(Vec3::new(1.0, -0.25, 2.0)); // 5: back bottom right
        mesh.add_vert(Vec3::new(0.8, 0.25, 1.6)); // 6: back top right
        mesh.add_vert(Vec3::new(-0.8, 0.25, 1.6)); // 7: back top left

        add_cube_lines_and_faces(&mut mesh);
        mesh
    }
}

fn add_cube_lines_and_faces(mesh: &mut Mesh) {
    // 12 edges
    mesh.add_line((0, 1)); // front bottom
    mesh.add_line((1, 2)); // front right
    mesh.add_line((2, 3)); // front top
    mesh.add_line((3, 0)); // front left
    mesh.add_line((4, 5)); // back bottom
    mesh.add_line((5, 6)); // back right
    mesh.add_line((6, 7)); // back top
    mesh.add_line((7, 4)); // back left
    mesh.add_line((0, 4)); // bottom left
    mesh.add_line((1, 5)); // bottom right
    mesh.add_line((2, 6)); // top right
    mesh.add_line((3, 7)); // top left

    // 6 faces (vertex indices in winding order)
    mesh.add_poly(vec![0, 1, 2, 3]); // front
    mesh.add_poly(vec![5, 4, 7, 6]); // back
    mesh.add_poly(vec![0, 4, 5, 1]); // bottom
    mesh.add_poly(vec![3, 2, 6, 7]); // top
    mesh.add_poly(vec![0, 3, 7, 4]); // left
    mesh.add_poly(vec![1, 5, 6, 2]); // right
}

//
// Finding points
// (e.g. used for selection)
//

impl Mesh {
    pub fn find_verts_xy(&self, target_coord_xy: Vec2, radius: f32) -> Vec<VertIndex> {
        self.verticies
            .iter()
            .enumerate()
            .filter(|(_, v)| {
                let v2d = vec2(v.x, v.y);
                return target_coord_xy.distance(v2d) <= radius;
            })
            .sorted_by(|a, b| a.1.z.total_cmp(&b.1.z))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn find_verts_xz(&self, target_coord_xz: Vec2, radius: f32) -> Vec<VertIndex> {
        self.verticies
            .iter()
            .enumerate()
            .filter(|(_, v)| {
                let v2d = vec2(v.x, v.z);
                return target_coord_xz.distance(v2d) <= radius;
            })
            .sorted_by(|a, b| a.1.y.total_cmp(&b.1.y))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn find_verts_yz(&self, target_coord_yz: Vec2, radius: f32) -> Vec<VertIndex> {
        self.verticies
            .iter()
            .enumerate()
            .filter(|(_, v)| {
                let v2d = vec2(v.y, v.z);
                return target_coord_yz.distance(v2d) <= radius;
            })
            .sorted_by(|a, b| a.1.x.total_cmp(&b.1.x))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn lines_in_vertex_indicies(&self, index_set: &HashSet<VertIndex>) -> HashSet<LineIndex> {
        self.lines
            .iter()
            .enumerate()
            .filter(|(_, l)| index_set.contains(&l.0) && index_set.contains(&l.1))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn polys_in_vertex_indicies(&self, index_set: &HashSet<VertIndex>) -> HashSet<PolyIndex> {
        self.polys
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                let count = p.iter().filter(|i| index_set.contains(i)).count();
                count == p.len()
            })
            .map(|(i, _)| i)
            .collect()
    }

    pub fn polys_partially_in_vertex_indicies(
        &self,
        index_set: &HashSet<VertIndex>,
    ) -> HashSet<PolyIndex> {
        self.polys
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                let count = p.iter().filter(|i| index_set.contains(i)).count();
                count >= 3 // at least 3 points bordering the poly
            })
            .map(|(i, _)| i)
            .collect()
    }

    pub fn find_verts_between(
        &self,
        start_index: VertIndex,
        end_index: VertIndex,
    ) -> Vec<VertIndex> {
        mesh_bfs(start_index, end_index, &self.lines)
    }

    pub fn vert_indicies_from_poly_indicies(
        &self,
        poly_indicies: HashSet<PolyIndex>,
    ) -> Vec<VertIndex> {
        poly_indicies
            .iter()
            .filter_map(|&i| self.polys.get(i))
            .flatten()
            .copied()
            .collect()
    }
}
