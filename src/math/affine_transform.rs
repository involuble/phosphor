use cgmath::*;

pub trait Transformable {
    fn transform_by(&mut self, transform: &AffineTransform);
}

#[repr(C)]
#[derive(Clone, Copy)]
/// A 3D affine transformation
pub struct AffineTransform {
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
    pub translation: Vector3<f32>,
}

impl AffineTransform {
    pub fn transform_point(&self, p: Point3<f32>) -> Point3<f32> {
        let scale = Point3::new(self.scale.x, self.scale.y, self.scale.z);
        self.rotation.rotate_point(p).mul_element_wise(scale) + self.translation
    }

    pub fn transform_vector(&self, v: Vector3<f32>) -> Vector3<f32> {
        self.rotation.rotate_vector(v).mul_element_wise(self.scale)
    }

    pub fn to_matrix(&self) -> Matrix4<f32> {
        let mut m = Matrix4::from(self.rotation);
        m = m * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        m.w = self.translation.extend(1.0);
        m
    }

    pub fn is_similarity(&self) -> bool {
        self.scale.x == self.scale.y && self.scale.y == self.scale.z
    }

    pub fn is_isometry(&self) -> bool {
        self.scale.x.abs() == 1.0 && self.scale.y.abs() == 1.0 && self.scale.z.abs() == 1.0 
    }
}