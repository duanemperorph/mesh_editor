// Unit tests for the Mesh struct and its operations.
//
// Test coverage includes:
// - Vertex operations: add, update, delete
// - Line operations: add, remove
// - Edge cases: invalid indices, empty mesh, extreme coordinates
// - State consistency: vertex deletion cascading to lines, index management
//
// pub struct Coord3D {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
// }

// pub type Line = (VertIndex, VertIndex);

// pub type VertIndex = usize;
// pub type LineIndex = usize;

// pub struct Mesh {
//     verticies: Vec<Coord3D>,
//     lines: Vec<Line>,
// }

use crate::mesh::{Coord3D, Line, Mesh};

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a new empty mesh
    fn empty_mesh() -> Mesh {
        Mesh::new()
    }

    // Helper to create a coordinate
    fn coord(x: f32, y: f32, z: f32) -> Coord3D {
        Coord3D::new(x, y, z)
    }

    // ==================== add_vert tests ====================

    #[test]
    fn test_add_single_vertex() {
        let mut mesh = empty_mesh();
        let idx = mesh.add_vert(coord(1.0, 2.0, 3.0));
        assert_eq!(idx, 0);
        assert_eq!(mesh.verticies().len(), 1);
    }

    #[test]
    fn test_add_multiple_vertices() {
        let mut mesh = empty_mesh();
        let idx0 = mesh.add_vert(coord(0.0, 0.0, 0.0));
        let idx1 = mesh.add_vert(coord(1.0, 1.0, 1.0));
        let idx2 = mesh.add_vert(coord(2.0, 2.0, 2.0));

        assert_eq!(idx0, 0);
        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
        assert_eq!(mesh.verticies().len(), 3);
    }

    #[test]
    fn test_add_vertex_extreme_values() {
        let mut mesh = empty_mesh();
        mesh.add_vert(coord(f32::MAX, f32::MIN, 0.0));
        mesh.add_vert(coord(-0.0, f32::INFINITY, f32::NEG_INFINITY));
        assert_eq!(mesh.verticies().len(), 2);
    }

    // ==================== update_vert tests ====================

    #[test]
    fn test_update_existing_vertex() {
        let mut mesh = empty_mesh();
        let idx = mesh.add_vert(coord(1.0, 2.0, 3.0));
        mesh.update_vert(idx, coord(4.0, 5.0, 6.0));

        let updated = mesh.verticies().get(idx).unwrap();
        assert_eq!(updated.x, 4.0);
        assert_eq!(updated.y, 5.0);
        assert_eq!(updated.z, 6.0);
    }

    #[test]
    fn test_update_invalid_index() {
        let mut mesh = empty_mesh();
        mesh.add_vert(coord(1.0, 2.0, 3.0));
        // Should not panic, but should handle gracefully
        mesh.update_vert(999, coord(0.0, 0.0, 0.0));
        assert_eq!(mesh.verticies().len(), 1);
    }

    // ==================== delete_vert tests ====================

    #[test]
    fn test_delete_vertex() {
        let mut mesh = empty_mesh();
        let idx = mesh.add_vert(coord(1.0, 2.0, 3.0));
        mesh.delete_vert(idx);
        assert_eq!(mesh.verticies().len(), 0);
    }

    #[test]
    fn test_delete_invalid_index() {
        let mut mesh = empty_mesh();
        mesh.add_vert(coord(1.0, 2.0, 3.0));
        mesh.delete_vert(999);
        assert_eq!(mesh.verticies().len(), 1);
    }

    #[test]
    fn test_delete_from_empty_mesh() {
        let mut mesh = empty_mesh();
        mesh.delete_vert(0);
        assert_eq!(mesh.verticies().len(), 0);
    }

    // ==================== add_line tests ====================

    #[test]
    fn test_add_valid_line() {
        let mut mesh = empty_mesh();
        let v0 = mesh.add_vert(coord(0.0, 0.0, 0.0));
        let v1 = mesh.add_vert(coord(1.0, 1.0, 1.0));

        mesh.add_line((v0, v1));
        assert_eq!(mesh.lines().len(), 1);
    }

    #[test]
    fn test_add_multiple_lines() {
        let mut mesh = empty_mesh();
        let v0 = mesh.add_vert(coord(0.0, 0.0, 0.0));
        let v1 = mesh.add_vert(coord(1.0, 0.0, 0.0));
        let v2 = mesh.add_vert(coord(0.0, 1.0, 0.0));

        mesh.add_line((v0, v1));
        mesh.add_line((v1, v2));
        mesh.add_line((v2, v0));
        assert_eq!(mesh.lines().len(), 3);
    }

    #[test]
    fn test_add_line_invalid_vertices() {
        let mut mesh = empty_mesh();
        mesh.add_vert(coord(0.0, 0.0, 0.0));
        // Adding line with non-existent vertex should handle gracefully
        mesh.add_line((0, 999));
        // Behavior depends on implementation - may reject or accept
    }

    // ==================== remove_line tests ====================

    #[test]
    fn test_remove_line() {
        let mut mesh = empty_mesh();
        let v0 = mesh.add_vert(coord(0.0, 0.0, 0.0));
        let v1 = mesh.add_vert(coord(1.0, 1.0, 1.0));
        mesh.add_line((v0, v1));

        mesh.remove_line(0);
        assert_eq!(mesh.lines().len(), 0);
    }

    #[test]
    fn test_remove_invalid_line_index() {
        let mut mesh = empty_mesh();
        let v0 = mesh.add_vert(coord(0.0, 0.0, 0.0));
        let v1 = mesh.add_vert(coord(1.0, 1.0, 1.0));
        mesh.add_line((v0, v1));

        mesh.remove_line(999);
        assert_eq!(mesh.lines().len(), 1);
    }

    // ==================== Integration tests ====================

    #[test]
    fn test_delete_vertex_removes_associated_lines() {
        let mut mesh = empty_mesh();
        let v0 = mesh.add_vert(coord(0.0, 0.0, 0.0));
        let v1 = mesh.add_vert(coord(1.0, 0.0, 0.0));
        let v2 = mesh.add_vert(coord(0.0, 1.0, 0.0));

        mesh.add_line((v0, v1));
        mesh.add_line((v1, v2));

        // Deleting v1 should remove both lines that reference it
        mesh.delete_vert(v1);
        assert_eq!(mesh.lines().len(), 0);
    }

    #[test]
    fn test_delete_vertex_reindexes_remaining_lines() {
        let mut mesh = empty_mesh();
        // Create vertices: v0=0, v1=1, v2=2, v3=3
        let v0 = mesh.add_vert(coord(0.0, 0.0, 0.0));
        let _v1 = mesh.add_vert(coord(1.0, 0.0, 0.0));
        let v2 = mesh.add_vert(coord(0.0, 1.0, 0.0));
        let v3 = mesh.add_vert(coord(1.0, 1.0, 0.0));

        // Line between v2 and v3 (indices 2 and 3)
        mesh.add_line((v2, v3));

        // Delete v1 (index 1) - swap_remove moves v3 (last) to index 1
        // So v2 stays at index 2, v3 moves from index 3 to index 1
        mesh.delete_vert(1);

        // Line should still exist, with v3's index updated from 3 to 1
        assert_eq!(mesh.lines().len(), 1);
        let line = mesh.lines().get(0).copied().unwrap();
        // After swap_remove: v2 stays at 2, v3 (was 3) -> 1
        assert_eq!(line, (2, 1));
    }

    #[test]
    fn test_delete_last_vertex_removes_its_lines() {
        let mut mesh = empty_mesh();
        let v0 = mesh.add_vert(coord(0.0, 0.0, 0.0));
        let v1 = mesh.add_vert(coord(1.0, 0.0, 0.0));
        let v2 = mesh.add_vert(coord(0.0, 1.0, 0.0)); // last vertex (index 2)

        mesh.add_line((v0, v1));
        mesh.add_line((v1, v2)); // references last vertex

        // Delete last vertex - should remove line (v1, v2) but keep (v0, v1)
        mesh.delete_vert(v2);
        assert_eq!(mesh.verticies().len(), 2);
        assert_eq!(mesh.lines().len(), 1);
        let line = mesh.lines().get(0).copied().unwrap();
        assert_eq!(line, (0, 1));
    }

    #[test]
    fn test_delete_first_vertex_reindexes_all_lines() {
        let mut mesh = empty_mesh();
        let _v0 = mesh.add_vert(coord(0.0, 0.0, 0.0));
        let v1 = mesh.add_vert(coord(1.0, 0.0, 0.0));
        let v2 = mesh.add_vert(coord(0.0, 1.0, 0.0));

        // Line between v1 and v2 (indices 1 and 2)
        mesh.add_line((v1, v2));

        // Delete v0 (index 0) - swap_remove moves v2 (last) to index 0
        // So v1 stays at index 1, v2 moves from index 2 to index 0
        mesh.delete_vert(0);

        assert_eq!(mesh.verticies().len(), 2);
        assert_eq!(mesh.lines().len(), 1);
        let line = mesh.lines().get(0).copied().unwrap();
        // After swap_remove: v1 stays at 1, v2 (was 2) -> 0
        assert_eq!(line, (1, 0));
    }

    #[test]
    fn test_delete_only_vertices_with_lines() {
        let mut mesh = empty_mesh();
        let v0 = mesh.add_vert(coord(0.0, 0.0, 0.0));
        let v1 = mesh.add_vert(coord(1.0, 0.0, 0.0));

        mesh.add_line((v0, v1));

        // Delete v1 (index 1, also the last) - line referencing it is removed
        mesh.delete_vert(v1);
        assert_eq!(mesh.verticies().len(), 1);
        assert_eq!(mesh.lines().len(), 0);

        mesh.delete_vert(0); // v0 is at index 0
        assert_eq!(mesh.verticies().len(), 0);
        assert_eq!(mesh.lines().len(), 0);
    }

    #[test]
    fn test_roundtrip_add_delete_vertex() {
        let mut mesh = empty_mesh();
        let idx = mesh.add_vert(coord(1.0, 2.0, 3.0));
        assert_eq!(mesh.verticies().len(), 1);

        mesh.delete_vert(idx);
        assert_eq!(mesh.verticies().len(), 0);
    }

    #[test]
    fn test_roundtrip_add_remove_line() {
        let mut mesh = empty_mesh();
        let v0 = mesh.add_vert(coord(0.0, 0.0, 0.0));
        let v1 = mesh.add_vert(coord(1.0, 1.0, 1.0));
        mesh.add_line((v0, v1));
        assert_eq!(mesh.lines().len(), 1);

        mesh.remove_line(0);
        assert_eq!(mesh.lines().len(), 0);
    }
}
