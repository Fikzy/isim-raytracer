use crate::{ray::Ray, shape::Shape, texture::TextureMaterial};
use nalgebra::{Point3, Vector3};

pub struct Object {
    pub position: Point3<f32>,
    pub shape: Box<dyn Shape>,
    pub texture: Box<dyn TextureMaterial>,
}

impl Object {
    pub fn intersects(&self, ray: &Ray) -> Option<f32> {
        self.shape.intersects(&self.position, ray)
    }
    pub fn normal(&self, point: Point3<f32>) -> Vector3<f32> {
        self.shape.normal(&self.position, point)
    }
}
