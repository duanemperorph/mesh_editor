//
// duplicate - duplicate mutation functionality
//

use super::Mesh;
use super::types::*;
use std::collections::{HashMap, HashSet};

impl Mesh {
    pub fn duplicate_verts(&mut self, vert_indicies_to_dup: &[VertIndex], with_extrusion: bool) {
        let vert_indicies_to_dup_set: HashSet<VertIndex> =
            vert_indicies_to_dup.iter().copied().collect();
        let line_indicies_to_dup: Vec<LineIndex> =
            self.lines_in_vertex_indicies(&vert_indicies_to_dup_set);
        let poly_indicies_to_dup: Vec<PolyIndex> =
            self.polys_in_vertex_indicies(&vert_indicies_to_dup_set);

        let new_vert_mapping = self.duplicate_and_add_verts(vert_indicies_to_dup);
        let new_line_mapping =
            self.duplicate_and_add_lines(&line_indicies_to_dup, &new_vert_mapping);
        let _ = self.duplicate_and_add_polys(&poly_indicies_to_dup, &new_vert_mapping);

        if with_extrusion {
            self.connect_duplicated_verts(&vert_indicies_to_dup, &new_vert_mapping);
            self.connect_duplicated_lines(&line_indicies_to_dup, &new_line_mapping);
        }
    }

    fn duplicate_and_add_verts(
        &mut self,
        vert_indicies_to_dup: &[VertIndex],
    ) -> HashMap<VertIndex, VertIndex> {
        let verts_to_dup = self.selected_indicies_to_verts(vert_indicies_to_dup);
        let new_verts_start_index = self.verticies.len();
        self.verticies.extend(verts_to_dup);
        let mut new_vert_mapping = HashMap::<VertIndex, VertIndex>::new();

        for (i, &vert_index) in vert_indicies_to_dup.iter().enumerate() {
            let new_index = new_verts_start_index + i;
            new_vert_mapping.insert(vert_index, new_index);
        }
        new_vert_mapping
    }

    fn duplicate_and_add_lines(
        &mut self,
        line_indicies_to_dup: &[LineIndex],
        vert_mapping: &HashMap<VertIndex, VertIndex>,
    ) -> HashMap<LineIndex, LineIndex> {
        let lines_to_dup = self.selected_indicies_to_lines(line_indicies_to_dup);
        let new_lines_start_index = self.lines.len();
        let new_lines = lines_to_dup
            .iter()
            .map(|&old_line| Self::remap_line_verts(old_line, vert_mapping));
        self.lines.extend(new_lines);

        line_indicies_to_dup
            .iter()
            .enumerate()
            .map(|(i, &line_index)| (line_index, new_lines_start_index + i))
            .collect()
    }

    fn duplicate_and_add_polys(
        &mut self,
        poly_indicies_to_dup: &[PolyIndex],
        vert_mapping: &HashMap<VertIndex, VertIndex>,
    ) -> HashMap<PolyIndex, PolyIndex> {
        let polys_to_dup = self.selected_indicies_to_polys(poly_indicies_to_dup);
        let new_polys_start_index = self.polys.len();
        let new_polys: Vec<Poly> = polys_to_dup
            .iter()
            .map(|&old_poly| Self::remap_poly_verts(old_poly, vert_mapping))
            .collect();
        self.polys.extend(new_polys);

        poly_indicies_to_dup
            .iter()
            .enumerate()
            .map(|(i, &poly_index)| (poly_index, new_polys_start_index + i))
            .collect()
    }

    fn remap_line_verts(old_line: Line, vert_mapping: &HashMap<VertIndex, VertIndex>) -> Line {
        let &new_0 = vert_mapping.get(&old_line.0).unwrap();
        let &new_1 = vert_mapping.get(&old_line.1).unwrap();
        (new_0, new_1)
    }

    fn remap_poly_verts(old_poly: &Poly, vert_mapping: &HashMap<VertIndex, VertIndex>) -> Poly {
        old_poly
            .iter()
            .map(|&old_index| *vert_mapping.get(&old_index).unwrap())
            .collect()
    }

    fn connect_duplicated_verts(
        &mut self,
        original_points: &[VertIndex],
        new_index_map: &HashMap<VertIndex, VertIndex>,
    ) {
        for &original_point in original_points.iter() {
            let &new_point = new_index_map.get(&original_point).unwrap();
            self.lines.push((original_point, new_point));
        }
    }

    fn connect_duplicated_lines(
        &mut self,
        original_line_indicies: &[LineIndex],
        line_mapping: &HashMap<LineIndex, LineIndex>,
    ) {
        for &original_line_index in original_line_indicies {
            let &original_line = self.lines.get(original_line_index).unwrap();
            let &new_line_index = line_mapping.get(&original_line_index).unwrap();
            let &new_line = self.lines.get(new_line_index).unwrap();
            let new_poly: Poly = vec![original_line.0, original_line.1, new_line.1, new_line.0];
            self.polys.push(new_poly);
        }
    }
}
