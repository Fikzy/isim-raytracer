pub mod uniform;

use image::Rgb;
use nalgebra::Point3;

pub trait TextureMaterial {
    fn color(&self, point: Point3<f32>) -> Rgb<u8>;
    fn propeties(&self, point: Point3<f32>) -> (f32, f32, f32);
}
