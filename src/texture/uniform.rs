use crate::texture::TextureMaterial;
use nalgebra::{Point3, Vector3};

#[derive(Debug)]
pub struct UniformTexture {
    pub kd: f32,
    pub ks: f32,
    pub ka: f32,
    pub color: Vector3<f32>,
}

impl UniformTexture {
    pub fn new(kd: f32, ks: f32, ka: f32, color: Vector3<u8>) -> Self {
        UniformTexture {
            kd,
            ks,
            ka,
            color: nalgebra::vector![color.x as f32, color.y as f32, color.z as f32],
        }
    }
}

impl TextureMaterial for UniformTexture {
    fn color(&self, _: Point3<f32>) -> Vector3<f32> {
        self.color
    }
    fn propeties(&self, _: Point3<f32>) -> (f32, f32, f32) {
        (self.kd, self.ks, self.ka)
    }
}
