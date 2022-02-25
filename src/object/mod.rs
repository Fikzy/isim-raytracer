pub mod sphere;

extern crate nalgebra as na;
use na::{Point3, Vector3};

use crate::ray::Ray;
use crate::texture::TextureMaterialTrait;

#[derive(Debug)]
pub struct Object<T: TextureMaterialTrait> {
    pub texture: T,
}

pub trait ObjectTrait {
    fn intersects(&self, ray: Ray) -> Option<f32>;
    fn normal(&self, point: Point3<f32>) -> Vector3<f32>;
    fn find_texture(&self, point: Point3<f32>) -> (f32, f32, f32);
}
