use crate::texture::TextureMaterial;
use image::Rgb;
use nalgebra::Point3;

#[derive(Debug)]
pub struct UniformTexture {
    pub kd: f32,
    pub ks: f32,
    pub ka: f32,
    pub color: Rgb<u8>,
}

impl UniformTexture {
    pub fn new(kd: f32, ks: f32, ka: f32, color: Rgb<u8>) -> Self {
        UniformTexture { kd, ks, ka, color }
    }
}

impl TextureMaterial for UniformTexture {
    fn color(&self, _: Point3<f32>) -> Rgb<u8> {
        self.color
    }
    fn propeties(&self, _: Point3<f32>) -> (f32, f32, f32) {
        (self.kd, self.ks, self.ka)
    }
}
