//
// mesh_bfs -> use bfs to find the closest route between 2 points
//
use super::{Line, VertIndex};
use std::collections::{HashMap, HashSet};

pub fn mesh_bfs(start_index: VertIndex, target_index: VertIndex, lines: &[Line]) -> Vec<VertIndex> {
    let mut visited = HashSet::<VertIndex>::from([start_index]);
    let mut next_indicies = vec![start_index];
    let mut prev_index_map = HashMap::<VertIndex, VertIndex>::new();

    'outta_here: while !next_indicies.is_empty() {
        let mut next_next_indicies: Vec<VertIndex> = Vec::new();

        for vert_index in next_indicies {
            if vert_index == target_index {
                break 'outta_here;
            }
            let linked_verts = find_linked_verts(vert_index, lines, &visited);
            next_next_indicies.extend(&linked_verts);

            for next_vert in linked_verts.iter() {
                visited.insert(*next_vert);
                prev_index_map.insert(*next_vert, vert_index);
            }
        }
        next_indicies = next_next_indicies;
    }

    let mut return_path = vec![target_index];
    let mut current_node = target_index;

    while let Some(next_node) = prev_index_map.get(&current_node) {
        return_path.push(*next_node);
        current_node = *next_node;
    }

    return_path
}

fn find_linked_verts(
    from_point: VertIndex,
    lines: &[Line],
    visited: &HashSet<VertIndex>,
) -> Vec<VertIndex> {
    lines
        .iter()
        .filter(|(i1, i2)| *i1 == from_point || *i2 == from_point)
        .map(|(i1, i2)| if *i1 == from_point { *i2 } else { *i1 })
        .filter(|i| !visited.contains(&i))
        .collect()
}
