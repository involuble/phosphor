use glam::*;

// TODO

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Unit<V: InnerSpace>(V);