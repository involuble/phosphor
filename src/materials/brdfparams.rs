use na::*;

use colour::*;
use materialparams::*;

pub struct BRDFParams {
    pub n: Vector3<f32>,
    // pub n_geom: Vector3<f32>,
    pub tangent: Vector3<f32>,
    pub bitang: Vector3<f32>,
    pub mat: MaterialParams,
}