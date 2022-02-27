pub mod uniform;

extern crate nalgebra as na;
use image::Rgb;
use na::Point3;

pub trait TextureMaterial {
    fn find(&self, point: Point3<f32>) -> Rgb<u8>;
}
