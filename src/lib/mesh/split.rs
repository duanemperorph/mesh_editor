//
// Addative mutations (split)
//

use super::{Line, LineIndex, Mesh, VertIndex};
use std::collections::HashSet;

impl Mesh {
    pub fn split_lines(&mut self, line_indicies: &HashSet<LineIndex>) {
        let lines_ordered: Vec<Line> = self
            .lines
            .iter()
            .enumerate()
            .filter(|(i, _)| line_indicies.contains(i))
            .map(|(_, &l)| l)
            .collect();

        let center_vecs: Vec<macroquad::prelude::Vec3> = self
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
