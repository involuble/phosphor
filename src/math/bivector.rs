use glam::*;

// TODO

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Bivector3<S: BaseFloat> {
    pub x: S,
    pub y: S,
    pub z: S,
}
