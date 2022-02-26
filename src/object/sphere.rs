extern crate nalgebra as na;
use na::{Point3, Vector3};

use crate::object::Object;
use crate::ray::Ray;
use crate::texture::TextureMaterial;

#[derive(Debug)]
pub struct Sphere<T: TextureMaterial> {
    pub center: Point3<f32>,
    pub radius: f32,
    pub texture: T,
}

impl<T> Object for Sphere<T>
where
    T: TextureMaterial,
{
    fn intersects(&self, ray: Ray) -> Option<f32> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);

        let delta = b.powi(2) - 4.0 * a * c;
        if delta < 0.0 {
            None
        } else if delta == 0.0 {
            Some(-b / (2.0 * a))
        } else {
            let r1 = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
            let r2 = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
            Some(*na::partial_min(&r1, &r2).unwrap())
        }
    }
    fn normal(&self, point: Point3<f32>) -> Vector3<f32> {
        (self.center - point).normalize()
    }
    fn find_texture(&self, point: Point3<f32>) -> (f32, f32, f32) {
        self.texture.find(point)
    }
}
