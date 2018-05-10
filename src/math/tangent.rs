use cgmath::*;

/// An orthonormal basis on a surface
#[derive(Debug, Clone, Copy)]
pub struct TangentFrame {
    pub tangent: Vector3<f32>,
    pub bitangent: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl TangentFrame {
    pub fn from_normal(n: Vector3<f32>) -> Self {
        let (t, b) = make_orthonormal_basis(n);
        TangentFrame {
            tangent: t,
            bitangent: b,
            normal: n,
        }
    }

    pub fn transform(&self, v: Vector3<f32>) -> Vector3<f32> {
        self.tangent * v.x + self.bitangent * v.y + self.normal * v.z
    }
}

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