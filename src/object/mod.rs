pub mod sphere;

extern crate nalgebra as na;
use image::Rgb;
use na::{Point3, Vector3};

use crate::ray::Ray;

pub trait Object {
    fn intersects(&self, ray: &Ray) -> Option<f32>;
    fn normal(&self, point: Point3<f32>) -> Vector3<f32>;
    fn get_color(&self, point: Point3<f32>) -> Rgb<u8>;
    fn get_properties(&self, point: Point3<f32>) -> (f32, f32, f32);
}
