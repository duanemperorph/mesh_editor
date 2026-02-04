//
// mesh -> custom mesh format
//

mod bfs;
mod duplicate;
mod mutations;
mod primitives;
mod query;
mod split;
mod types;

use bfs::mesh_bfs;
use macroquad::prelude::Vec3;
use std::collections::HashSet;
use std::fmt;
pub use types::*;

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
    pub(crate) verticies: Vec<Vec3>,
    pub(crate) lines: Vec<Line>,
    pub(crate) polys: Vec<Poly>,
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
            .map(|(_, v)| v)
            .copied()
            .collect()
    }

    pub fn selected_indicies_to_lines(&self, indicies: &[LineIndex]) -> Vec<Line> {
        self.lines
            .iter()
            .enumerate()
            .filter(|(i, _)| indicies.contains(i))
            .map(|(_, l)| l)
            .copied()
            .collect()
    }

    pub fn selected_indicies_to_polys(&self, indicies: &[PolyIndex]) -> Vec<&Poly> {
        self.polys
            .iter()
            .enumerate()
            .filter(|(i, _)| indicies.contains(i))
            .map(|(_, p)| p)
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

    pub fn find_verts_between(
        &self,
        start_index: VertIndex,
        end_index: VertIndex,
    ) -> Vec<VertIndex> {
        mesh_bfs(start_index, end_index, &self.lines)
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
