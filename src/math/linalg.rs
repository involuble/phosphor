#![allow(dead_code)]

use cgmath::*;

// Creates an orthonormal basis given a normal vector.
//   The vectors are returned in a tuple as tangent and bitangent
// Reference: [Duff2017Basis]
//  http://jcgt.org/published/0006/01/01/paper.pdf or
//  http://graphics.pixar.com/library/OrthonormalB/paper.pdf
pub fn make_orthonormal_basis(n: Vector3<f32>) -> (Vector3<f32>, Vector3<f32>) {
    let sign = n.z.signum();
    let a = -1.0 / (sign + n.z);
    let b = n.x * n.y * a;
    let b1 = Vector3::new(1.0 + sign*n.x*n.x*a, sign*b, -sign*n.x);
    let b2 = Vector3::new(b, sign + n.y*n.y*a, -n.y);
    (b1, b2)
}

pub fn polar_angles_to_cartesian(theta: f32, phi: f32) -> Vector3<f32> {
    let sin_theta = theta.sin();
    Vector3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), theta.cos())
}

pub fn polar_to_cartesian(sin_theta: f32, cos_theta: f32, phi: f32) -> Vector3<f32> {
    Vector3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
}

pub fn same_hemisphere(a: Vector3<f32>, b: Vector3<f32>) -> bool {
    dot(a, b) > 0.0
}

pub fn same_hemisphere_normal(normal: Vector3<f32>, a: Vector3<f32>, b: Vector3<f32>) -> bool {
    dot(normal, a) * dot(normal, b) > 0.0
}