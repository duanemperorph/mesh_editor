//
// selection - tracks and manipulated vert selection
//
use mesh_editor::mesh::*;
use std::collections::HashSet;

pub struct Selection {
    selected_vert_indicies: Vec<VertIndex>,
}

impl Selection {
    pub fn new() -> Selection {
        Selection {
            selected_vert_indicies: Vec::new(),
        }
    }

    pub fn selected_vert_indicies(&self) -> &Vec<VertIndex> {
        &self.selected_vert_indicies
    }

    pub fn selected_vert_indicies_set(&self) -> HashSet<VertIndex> {
        self.selected_vert_indicies.iter().copied().collect()
    }

    pub fn replace_selected_vert_indicies(&mut self, new_indicies: &[VertIndex]) {
        self.selected_vert_indicies = new_indicies.to_vec();
    }

    pub fn add_selected_vert_indicies(&mut self, new_indicies: &[VertIndex]) {
        self.selected_vert_indicies.extend(new_indicies);
    }

    pub fn remove_selected_vert_indicies(&mut self, remove_indicies: &[VertIndex]) {
        self.selected_vert_indicies
            .retain(|i| !remove_indicies.contains(i));
    }

    pub fn toggle_selected_vert_index(&mut self, index: VertIndex) {
        if let Some(pos) = self.selected_vert_indicies.iter().position(|&i| i == index) {
            self.selected_vert_indicies.remove(pos);
        } else {
            self.selected_vert_indicies.push(index);
        }
    }

    pub fn clear(&mut self) {
        self.selected_vert_indicies.clear();
    }
}
