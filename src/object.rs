extern crate nalgebra as na;
use na::{Point3, Vector3};

use crate::ray::Ray;
use crate::texture::TextureMaterialTrait;

struct SceneObject<T: TextureMaterialTrait> {
    pub texture: T,
}

pub trait SceneObjectTrait {
    fn intersects(&self, ray: Ray) -> bool;
    fn norm(&self, point: Point3<f32>) -> Vector3<f32>;
    fn find_texture(&self, point: Point3<f32>) -> Point3<f32>;
}

pub struct Sphere<T: TextureMaterialTrait> {
    object: SceneObject<T>,
    origin: Point3<f32>,
    radius: f32,
}

impl<T> SceneObjectTrait for Sphere<T>
where
    T: TextureMaterialTrait,
{
    fn intersects(&self, ray: Ray) -> bool {
        todo!()
    }
    fn norm(&self, point: Point3<f32>) -> Vector3<f32> {
        todo!()
    }
    fn find_texture(&self, point: Point3<f32>) -> Point3<f32> {
        todo!()
    }
}
