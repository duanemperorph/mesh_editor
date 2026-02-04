use macroquad::prelude::*;

pub type VertIndex = usize;
pub type LineIndex = usize;
pub type PolyIndex = usize;

pub type Line = (VertIndex, VertIndex);
pub type Poly = Vec<VertIndex>;
pub type TriangleVerts = (Vec3, Vec3, Vec3);

#[derive(Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub enum MirrorMode {
    None,
    Bilateral,
    Radial(u8),
}
