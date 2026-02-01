//
// selection - tracks and manipulated vert selection
//
use mesh_editor::mesh::*;
use std::collections::HashSet;

pub struct Selection {
    selected_vert_indicies: HashSet<VertIndex>,
}

impl Selection {
    pub fn new() -> Selection {
        Selection {
            selected_vert_indicies: HashSet::new(),
        }
    }

    pub fn selected_vert_indicies(&self) -> &HashSet<VertIndex> {
        &self.selected_vert_indicies
    }

    pub fn replace_selected_vert_indicies(&mut self, new_indicies: &[VertIndex]) {
        self.selected_vert_indicies = new_indicies.iter().copied().collect()
    }

    pub fn replace_selected_vert_indicies_set(&mut self, new_indicies: HashSet<VertIndex>) {
        self.selected_vert_indicies = new_indicies
    }

    pub fn add_selected_vert_indicies(&mut self, new_indicies: &[VertIndex]) {
        self.selected_vert_indicies.extend(new_indicies);
    }

    pub fn remove_selected_vert_indicies(&mut self, remove_indicies: &[VertIndex]) {
        for remove_index in remove_indicies {
            self.selected_vert_indicies.remove(&remove_index);
        }
    }

    pub fn toggle_selected_vert_index(&mut self, index: VertIndex) {
        if !self.selected_vert_indicies.remove(&index) {
            self.selected_vert_indicies.insert(index);
        }
    }

    pub fn clear(&mut self) {
        self.selected_vert_indicies.clear();
    }
}
