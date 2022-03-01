use crate::ray::Ray;
use crate::shape::Shape;
use nalgebra::{Point3, Vector3};

#[derive(Debug)]
pub struct Sphere {
    pub radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Sphere { radius }
    }
}

impl Shape for Sphere {
    fn intersects(&self, origin: &Point3<f32>, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - origin;

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
    fn normal(&self, origin: &Point3<f32>, point: Point3<f32>) -> Vector3<f32> {
        (point - origin).normalize()
    }
}
