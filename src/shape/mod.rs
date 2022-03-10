pub mod plane;
pub mod sphere;
pub mod triangle;

use crate::ray::Ray;
use nalgebra::{Point3, Vector3};

pub trait Shape {
    fn intersects(&self, origin: &Point3<f32>, ray: &Ray) -> Option<f32>;
    fn normal(&self, origin: &Point3<f32>, point: &Point3<f32>, ray: &Ray) -> Vector3<f32>;
}
