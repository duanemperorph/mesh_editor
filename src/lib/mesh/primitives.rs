//
// Test data
//

use super::Mesh;
use macroquad::prelude::*;

impl Mesh {
    pub fn new_cube() -> Mesh {
        let mut mesh = Mesh::new();

        // 8 vertices of a unit cube centered at origin
        mesh.add_vert(Vec3::new(-1.0, -1.0, -1.0)); // 0: front bottom left
        mesh.add_vert(Vec3::new(1.0, -1.0, -1.0)); // 1: front bottom right
        mesh.add_vert(Vec3::new(1.0, 1.0, -1.0)); // 2: front top right
        mesh.add_vert(Vec3::new(-1.0, 1.0, -1.0)); // 3: front top left
        mesh.add_vert(Vec3::new(-1.0, -1.0, 1.0)); // 4: back bottom left
        mesh.add_vert(Vec3::new(1.0, -1.0, 1.0)); // 5: back bottom right
        mesh.add_vert(Vec3::new(1.0, 1.0, 1.0)); // 6: back top right
        mesh.add_vert(Vec3::new(-1.0, 1.0, 1.0)); // 7: back top left

        add_cube_lines_and_faces(&mut mesh);
        mesh
    }

    pub fn new_tapered_box() -> Mesh {
        let mut mesh = Mesh::new();

        // 8 vertices of a tapered box (0.5 tall, 4.0 long in z, top face smaller)
        mesh.add_vert(Vec3::new(-1.0, -0.25, -2.0)); // 0: front bottom left
        mesh.add_vert(Vec3::new(1.0, -0.25, -2.0)); // 1: front bottom right
        mesh.add_vert(Vec3::new(0.8, 0.25, -1.6)); // 2: front top right
        mesh.add_vert(Vec3::new(-0.8, 0.25, -1.6)); // 3: front top left
        mesh.add_vert(Vec3::new(-1.0, -0.25, 2.0)); // 4: back bottom left
        mesh.add_vert(Vec3::new(1.0, -0.25, 2.0)); // 5: back bottom right
        mesh.add_vert(Vec3::new(0.8, 0.25, 1.6)); // 6: back top right
        mesh.add_vert(Vec3::new(-0.8, 0.25, 1.6)); // 7: back top left

        add_cube_lines_and_faces(&mut mesh);
        mesh
    }
}

fn add_cube_lines_and_faces(mesh: &mut Mesh) {
    // 12 edges
    mesh.add_line((0, 1)); // front bottom
    mesh.add_line((1, 2)); // front right
    mesh.add_line((2, 3)); // front top
    mesh.add_line((3, 0)); // front left
    mesh.add_line((4, 5)); // back bottom
    mesh.add_line((5, 6)); // back right
    mesh.add_line((6, 7)); // back top
    mesh.add_line((7, 4)); // back left
    mesh.add_line((0, 4)); // bottom left
    mesh.add_line((1, 5)); // bottom right
    mesh.add_line((2, 6)); // top right
    mesh.add_line((3, 7)); // top left

    // 6 faces (vertex indices in winding order)
    mesh.add_poly(vec![0, 1, 2, 3]); // front
    mesh.add_poly(vec![5, 4, 7, 6]); // back
    mesh.add_poly(vec![0, 4, 5, 1]); // bottom
    mesh.add_poly(vec![3, 2, 6, 7]); // top
    mesh.add_poly(vec![0, 3, 7, 4]); // left
    mesh.add_poly(vec![1, 5, 6, 2]); // right
}
