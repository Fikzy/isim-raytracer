pub mod uniform;

use nalgebra::{Point3, Vector3};

pub trait TextureMaterial {
    fn color(&self, point: Point3<f32>) -> Vector3<f32>;
    fn propeties(&self, point: Point3<f32>) -> (f32, f32, f32);
}
