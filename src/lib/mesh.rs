//
// mesh -> custom mesh format
//

use itertools::Itertools;
use macroquad::prelude::*;
use std::collections::HashSet;
use std::fmt;

pub type VertIndex = usize;
pub type LineIndex = usize;
pub type PolyIndex = usize;

pub type Line = (VertIndex, VertIndex);
pub type Poly = Vec<VertIndex>;
pub type TriangleVerts = (Vec3, Vec3, Vec3);

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

    pub fn polys_to_triangle_indicies(&self) -> Vec<VertIndex> {
        self.polys
            .iter()
            .map(|poly| Self::poly_indicies_to_triangle_indicies(poly))
            .flatten()
            .collect()
    }

    //
    // uses triangle fan to get indicies for each triangle
    //
    fn poly_indicies_to_triangle_indicies(poly: &Poly) -> Vec<VertIndex> {
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
// Finding points within radius of target coord
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

    pub fn lines_in_vertex_indicies(&self, indicies: Vec<VertIndex>) -> Vec<LineIndex> {
        let index_set: HashSet<VertIndex> = indicies.iter().copied().collect();

        self.lines
            .iter()
            .enumerate()
            .filter(|(_, l)| index_set.contains(&l.0) && index_set.contains(&l.1))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn polys_in_vertex_indicies(&self, indicies: Vec<VertIndex>) -> Vec<PolyIndex> {
        let index_set: HashSet<VertIndex> = indicies.iter().copied().collect();

        self.polys
            .iter()
            .enumerate()
            .filter(|(_, poly)| {
                let matching_indicies_count = poly.iter().filter(|i| index_set.contains(i)).count();
                return matching_indicies_count == poly.len();
            })
            .map(|(i, _)| i)
            .collect()
    }

    pub fn polys_partially_in_vertex_indicies(
        &self,
        indicies: Vec<VertIndex>,
        min_matching_verts: usize,
    ) -> Vec<PolyIndex> {
        let index_set: HashSet<VertIndex> = indicies.iter().copied().collect();

        self.polys
            .iter()
            .enumerate()
            .filter(|(_, poly)| {
                let matching_indicies_count = poly.iter().filter(|i| index_set.contains(i)).count();
                return matching_indicies_count >= min_matching_verts;
            })
            .map(|(i, _)| i)
            .collect()
    }
}
