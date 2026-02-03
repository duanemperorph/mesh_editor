use macroquad::prelude::Vec3;

pub type VertIndex = usize;
pub type LineIndex = usize;
pub type PolyIndex = usize;

pub type Line = (VertIndex, VertIndex);
pub type Poly = Vec<VertIndex>;
pub type TriangleVerts = (Vec3, Vec3, Vec3);
