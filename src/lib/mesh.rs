//
//
//

#[derive(Copy, Clone)]
pub struct Coord3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Line = (VertIndex, VertIndex);

pub type VertIndex = usize;
pub type LineIndex = usize;

pub struct Mesh {
    verticies: Vec<Coord3D>,
    lines: Vec<Line>,
}

impl Coord3D {
    pub fn new(x: f32, y: f32, z: f32) -> Coord3D {
        return Coord3D { x, y, z };
    }
}

impl Mesh {
    pub fn new() -> Mesh {
        return Mesh {
            verticies: Vec::new(),
            lines: Vec::new(),
        };
    }

    pub fn verticies(&self) -> &Vec<Coord3D> {
        &self.verticies
    }

    pub fn lines(&self) -> &Vec<Line> {
        &self.lines
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

    pub fn remove_line(&mut self, index: LineIndex) -> Option<Line> {
        if index >= self.lines.len() {
            return None;
        }
        return Some(self.lines.swap_remove(index));
    }

    fn remove_lines_containing_vert(&mut self, vert_index: VertIndex) {
        self.lines
            .retain(|l| l.0 != vert_index && l.1 != vert_index);
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
}
