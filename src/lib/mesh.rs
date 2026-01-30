//
// mesh -> custom mesh format
//

use glam::Vec3;
use std::collections::HashSet;
use std::fmt;

pub type VertIndex = usize;
pub type LineIndex = usize;
pub type PolyIndex = usize;

pub type Line = (VertIndex, VertIndex);
pub type Poly = Vec<VertIndex>;

pub enum MirrorMode {
    None,
    Bilateral,
    Quadralateral,
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
            mirror_mode: MirrorMode::Bilateral,
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

    // pub fn polys_to_triangle_verts(&self) -> Vec<(Vec3, Vec3, Vec3)> {

    // }

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
            Self::Quadralateral => write!(formatter, "Quadralateral"),
        }
    }
}

//
// Test data
//

impl Mesh {
    pub fn new_cube() -> Mesh {
        let mut mesh = Mesh::new();

        // 8 vertices of a box centered at origin (0.5 tall, 2.0 long in z)
        mesh.add_vert(Vec3::new(-1.0, -0.25, -1.0)); // 0: front bottom left
        mesh.add_vert(Vec3::new(1.0, -0.25, -1.0)); // 1: front bottom right
        mesh.add_vert(Vec3::new(0.8, 0.25, -0.8)); // 2: front top right
        mesh.add_vert(Vec3::new(-0.8, 0.25, -0.8)); // 3: front top left
        mesh.add_vert(Vec3::new(-1.0, -0.25, 1.0)); // 4: back bottom left
        mesh.add_vert(Vec3::new(1.0, -0.25, 1.0)); // 5: back bottom right
        mesh.add_vert(Vec3::new(0.8, 0.25, 0.8)); // 6: back top right
        mesh.add_vert(Vec3::new(-0.8, 0.25, 0.8)); // 7: back top left

        // 12 edges of the cube
        mesh.add_line((0, 1)); // 0: front bottom
        mesh.add_line((1, 2)); // 1: front right
        mesh.add_line((2, 3)); // 2: front top
        mesh.add_line((3, 0)); // 3: front left
        mesh.add_line((4, 5)); // 4: back bottom
        mesh.add_line((5, 6)); // 5: back right
        mesh.add_line((6, 7)); // 6: back top
        mesh.add_line((7, 4)); // 7: back left
        mesh.add_line((0, 4)); // 8: bottom left
        mesh.add_line((1, 5)); // 9: bottom right
        mesh.add_line((2, 6)); // 10: top right
        mesh.add_line((3, 7)); // 11: top left

        // 6 faces of the cube (vertex indices in winding order)
        mesh.add_poly(vec![0, 1, 2, 3]); // front
        mesh.add_poly(vec![5, 4, 7, 6]); // back
        mesh.add_poly(vec![0, 4, 5, 1]); // bottom
        mesh.add_poly(vec![3, 2, 6, 7]); // top
        mesh.add_poly(vec![0, 3, 7, 4]); // left
        mesh.add_poly(vec![1, 5, 6, 2]); // right

        mesh
    }
}
