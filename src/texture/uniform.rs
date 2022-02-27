extern crate nalgebra as na;

use image::Rgb;
use na::Point3;

use crate::texture::TextureMaterial;

#[derive(Debug)]
pub struct UniformTexture {
    pub kd: f32,
    pub ks: f32,
    pub ka: f32,
    pub color: Rgb<u8>,
}

impl TextureMaterial for UniformTexture {
    fn color(&self, _point: Point3<f32>) -> Rgb<u8> {
        self.color
    }
    fn diffusion(&self, _point: Point3<f32>) -> f32 {
        self.kd
    }
}
