pub mod uniform;

extern crate nalgebra as na;
use na::Point3;

pub trait TextureMaterialTrait {
    fn find(&self, point: Point3<f32>) -> (f32, f32, f32);
}
