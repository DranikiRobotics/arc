use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coord3D {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}