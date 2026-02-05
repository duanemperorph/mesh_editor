use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

pub type VertIndex = usize;
pub type LineIndex = usize;
pub type PolyIndex = usize;

pub type Line = (VertIndex, VertIndex);
pub type Poly = Vec<VertIndex>;
pub type TriangleVerts = (Vec3, Vec3, Vec3);

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum MirrorMode {
    None,
    Bilateral,
    RadialX(u8),
    RadialY(u8),
    RadialZ(u8),
}
