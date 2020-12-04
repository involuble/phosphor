use super::prelude::*;

/// An orthonormal basis on a surface
#[derive(Debug, Clone, Copy)]
pub struct TangentFrame {
    pub tangent: Vec3,
    pub bitangent: Vec3,
    pub normal: Vec3,
}

impl TangentFrame {
    pub fn from_normal(n: Vec3) -> Self {
        let (t, b) = make_orthonormal_basis(n);
        TangentFrame {
            tangent: t,
            bitangent: b,
            normal: n,
        }
    }

    pub fn transform(&self, v: Vec3) -> Vec3 {
        self.tangent * v.x + self.bitangent * v.y + self.normal * v.z
    }

    pub fn inv_transform(&self, v: Vec3) -> Vec3 {
        Vec3::new(dot(v, self.tangent), dot(v, self.bitangent), dot(v, self.normal))
    }
}

/// Creates an orthonormal basis given a vector.
/// The vectors are returned in a tuple as (x, y) and form a right hand
///  coordinate system of (x,y,z)
pub fn make_orthonormal_basis(n: Vec3) -> (Vec3, Vec3) {
    // Reference: [Duff2017Basis]
    //  http://jcgt.org/published/0006/01/01/paper.pdf or
    //  http://graphics.pixar.com/library/OrthonormalB/paper.pdf
    let sign = n.z.signum();
    let a = -1.0 / (sign + n.z);
    let b = n.x * n.y * a;
    let b1 = Vec3::new(1.0 + sign*n.x*n.x*a, sign*b, -sign*n.x);
    let b2 = Vec3::new(b, sign + n.y*n.y*a, -n.y);
    (b1, b2)
}