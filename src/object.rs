extern crate nalgebra as na;
use na::{Point3, Vector3};

use crate::ray::Ray;
use crate::texture::TextureMaterialTrait;

pub struct SceneObject<T: TextureMaterialTrait> {
    pub texture: T,
}

pub trait SceneObjectTrait {
    fn intersects(&self, ray: Ray) -> Option<f32>;
    fn norm(&self, point: Point3<f32>) -> Vector3<f32>;
    fn find_texture(&self, point: Point3<f32>) -> (f32, f32, f32);
}

pub struct Sphere<T: TextureMaterialTrait> {
    pub object: SceneObject<T>,
    pub center: Point3<f32>,
    pub radius: f32,
}

impl<T> SceneObjectTrait for Sphere<T>
where
    T: TextureMaterialTrait,
{
    fn intersects(&self, ray: Ray) -> Option<f32> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * (ray.direction.dot(&oc));
        let c = oc.dot(&oc) - self.radius.powi(2);

        if b.powi(2) - 4.0 * a * c < 0.0 {
            None
        } else {
            Some((-b - (b.powi(2) - 4.0 * a * c).sqrt()) / 2.0 * a)
        }
    }
    fn norm(&self, point: Point3<f32>) -> Vector3<f32> {
        (self.center - point).normalize()
    }
    fn find_texture(&self, point: Point3<f32>) -> (f32, f32, f32) {
        self.object.texture.find(point)
    }
}

impl<T> std::fmt::Display for Sphere<T>
where
    T: TextureMaterialTrait,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Sphere(\n    center: {},\n    radius: {}\n)",
            self.center, self.radius
        )
    }
}
