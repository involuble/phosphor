use cgmath::*;

use colour::*;

pub struct MaterialParams {
    // These apply to both specular and diffuse surfaces
    pub base_colour: Colour,
    pub roughness: f32,

    //  Diffuse parameters
    // pub subsurface: f32,

    //  Parameters for a specular surface
    pub f0: f32,
    // pub anisotropic: f32,
}