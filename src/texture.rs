extern crate nalgebra as na;
use na::Point3;

pub trait TextureMaterialTrait {
    fn find(&self, point: Point3<f32>) -> (f32, f32, f32);
}

#[derive(Debug)]
pub struct UniformTexture {
    pub kd: f32,
    pub ks: f32,
    pub ka: f32,
}

impl TextureMaterialTrait for UniformTexture {
    fn find(&self, _point: Point3<f32>) -> (f32, f32, f32) {
        (self.kd, self.ks, self.ka)
    }
}
