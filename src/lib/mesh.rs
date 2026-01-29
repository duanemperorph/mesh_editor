//
//
//
use std::collections::HashSet;

pub type VertIndex = usize;
pub type LineIndex = usize;
pub type PolyIndex = usize;

#[derive(Copy, Clone)]
pub struct Coord3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Line = (VertIndex, VertIndex);

pub type Poly = Vec<LineIndex>;

pub enum MirrorMode {
    None,
    Split,
    Radial(u32),
}

pub struct Mesh {
    mirror_mode: MirrorMode,
    verticies: Vec<Coord3D>,
    lines: Vec<Line>,
    polys: Vec<Poly>,
}

impl Coord3D {
    pub fn new(x: f32, y: f32, z: f32) -> Coord3D {
        return Coord3D { x, y, z };
    }
}

impl Mesh {
    pub fn new() -> Mesh {
        return Mesh {
            mirror_mode: MirrorMode::Split,
            verticies: Vec::new(),
            lines: Vec::new(),
            polys: Vec::new(),
        };
    }

    pub fn verticies(&self) -> &Vec<Coord3D> {
        &self.verticies
    }

    pub fn lines(&self) -> &Vec<Line> {
        &self.lines
    }

    pub fn mirror_mode(&self) -> &MirrorMode {
        &self.mirror_mode
    }

    pub fn set_mirror_mode(&mut self, mode: MirrorMode) {
        self.mirror_mode = mode;
    }

    pub fn add_vert(&mut self, coord: Coord3D) -> VertIndex {
        self.verticies.push(coord);
        return self.verticies.len() - 1;
    }

    pub fn update_vert(&mut self, index: VertIndex, coord: Coord3D) -> Option<()> {
        *self.verticies.get_mut(index as usize)? = coord;
        return Some(());
    }

    pub fn delete_vert(&mut self, index: VertIndex) -> Option<Coord3D> {
        if index >= self.verticies.len() {
            return None;
        }
        let last_vert_index = self.verticies.len() - 1;
        let removed_value = self.verticies.swap_remove(index);
        self.remove_lines_containing_vert(index);
        if index != last_vert_index {
            self.remap_swaped_vertex_indicies(last_vert_index, index);
        }
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
        self.cleanup_polys_after_line_removal(index, self.lines.len() - 1);
        return Some(self.lines.swap_remove(index));
    }

    pub fn remove_poly(&mut self, index: PolyIndex) -> Option<Poly> {
        if index >= self.polys.len() {
            return None;
        }
        return Some(self.polys.swap_remove(index));
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
            .all(|&line_index| self.lines.get(line_index).is_some())
            .then_some(())
    }

    fn check_for_poly_dup_indicies(poly: &Poly) -> bool {
        let mut seen = HashSet::new();
        !poly.iter().all(|i| seen.insert(i))
    }

    fn cleanup_polys_after_line_removal(
        &mut self,
        removed_line_index: LineIndex,
        replaced_line_index: LineIndex,
    ) {
        let poly_iter = self.polys.iter_mut();

        for poly in poly_iter {
            poly.retain(|&line_index| line_index != removed_line_index);

            for line_index in poly.iter_mut() {
                // acount for remapping caused by swap_remove
                if *line_index == replaced_line_index {
                    *line_index = removed_line_index;
                }
            }
        }

        self.polys.retain(|poly| poly.len() > 2);
    }
}
