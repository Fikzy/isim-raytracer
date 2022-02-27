extern crate nalgebra as na;
use image::Rgb;
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
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);

        let delta = b.powi(2) - 4.0 * a * c;
        if delta < 0.0 {
            return None;
        }

        let r1 = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
        let r2 = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);

        if delta == 0.0 && r2 > 0.0 {
            Some(r1)
        } else if delta > 0.0 {
            [r1, r2]
                .into_iter()
                .filter(|t| t >= &0.0)
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less))
        } else {
            None
        }
    }
    fn normal(&self, point: Point3<f32>) -> Vector3<f32> {
        (point - self.center).normalize()
    }
    fn get_color(&self, point: Point3<f32>) -> Rgb<u8> {
        self.texture.color(point)
    }
    fn get_diffusion(&self, point: Point3<f32>) -> f32 {
        self.texture.diffusion(point)
    }
}
