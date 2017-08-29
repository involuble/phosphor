use na::*;
use rand;

use colour::*;
use linalg::*;
use material::*;
use materialparams::*;


// G term in a microfacet distribution
pub trait ShadowingFn {
    pub fn shadowing(n: Vector3<f32>, alpha: f32, view: Vector3<f32>, light: Vector3<f32>, half: Vector3<f32>) -> f32;
}

// N/D term in a microfacet distribution
pub trait NormalDistFn {
    pub fn ndf(alpha: f32, half: Vector3<f32>) -> f32;

    pub fn sample_microfacet<R: rand::Rng>(rng: &mut R, alpha: f32) -> (f32, f32);
}

// This was an attempt to generalize over various microfacet distributions but I don't think there's any point
// TODO: Revisit this
pub struct Microfacet<N: NormalDistFn, G: ShadowingFn>;
