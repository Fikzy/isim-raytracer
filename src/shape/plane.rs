use nalgebra::{Point3, Vector3};

use super::Shape;

pub struct Plane {
    pub normal: Vector3<f32>,
}

impl Plane {
    pub fn new(normal: Vector3<f32>) -> Self {
        Plane { normal }
    }
}

impl Shape for Plane {
    fn intersects(&self, origin: &Point3<f32>, ray: &crate::ray::Ray) -> Option<f32> {
        if ray.direction.dot(&self.normal) == 0.0 {
            return None;
        }

        match ((origin - ray.origin).dot(&self.normal)) / ray.direction.dot(&self.normal) {
            t if t > 0.0 => Some(t),
            _ => None,
        }
    }

    fn normal(
        &self,
        _origin: &nalgebra::Point3<f32>,
        _point: &nalgebra::Point3<f32>,
        ray: &crate::ray::Ray,
    ) -> Vector3<f32> {
        self.normal * -self.normal.dot(&ray.direction).signum()
    }
}
