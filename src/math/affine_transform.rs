use super::prelude::*;

pub trait Transformable {
    fn transform_by(&mut self, transform: &AffineTransform);
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// A 3D affine transformation
pub struct AffineTransform {
    pub rotation: Mat3,
    pub scale: Vec3,
    pub translation: Vec3,
}

impl AffineTransform {
    pub fn transform_point(&self, p: Vec3) -> Vec3 {
        let scale = Vec3::new(self.scale.x, self.scale.y, self.scale.z);
        (self.rotation * p) * scale + self.translation
    }

    pub fn transform_normal(&self, v: Vec3) -> Vec3 {
        self.rotation * v
    }

    pub fn transform_vector(&self, v: Vec3) -> Vec3 {
        (self.rotation * v) * self.scale
    }

    pub fn to_matrix(&self) -> Mat4 {
        // switch to Quaternion? 
        let mut m: Mat4 = mat4(self.rotation.x_axis.extend(0.0), self.rotation.y_axis.extend(0.0), self.rotation.z_axis.extend(0.0), vec4(0.0, 0.0, 0.0, 1.0));
        m = m * Mat4::from_scale(self.scale);
        m.w_axis = self.translation.extend(1.0);
        m
    }

    pub fn is_similarity(&self) -> bool {
        self.scale.x == self.scale.y && self.scale.y == self.scale.z
    }

    pub fn _is_isometry(&self) -> bool {
        self.scale.x.abs() == 1.0 && self.scale.y.abs() == 1.0 && self.scale.z.abs() == 1.0
    }
}