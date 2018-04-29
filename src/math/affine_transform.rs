use cgmath::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AffineTransform {
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
    pub translation: Vector3<f32>,
}

impl AffineTransform {
    pub fn to_matrix(&self) -> Matrix4<f32> {
        let mut m = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        m = m * Matrix4::from(self.rotation);
        m.w = self.translation.extend(1.0);
        m
    }
}