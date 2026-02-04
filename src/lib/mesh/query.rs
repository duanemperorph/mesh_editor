//
// Finding points
// (e.g. used for selection)
//

use super::{LineIndex, Mesh, PolyIndex, VertIndex};
use itertools::Itertools;
use macroquad::prelude::*;
use std::collections::HashSet;

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

    pub fn lines_in_vertex_indicies<C>(&self, index_set: &HashSet<VertIndex>) -> C
    where
        C: FromIterator<LineIndex>,
    {
        self.lines
            .iter()
            .enumerate()
            .filter(|(_, l)| index_set.contains(&l.0) && index_set.contains(&l.1))
            .map(|(i, _)| i)
            .collect()
    }

    pub fn polys_in_vertex_indicies<C>(&self, index_set: &HashSet<VertIndex>) -> C
    where
        C: FromIterator<LineIndex>,
    {
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
