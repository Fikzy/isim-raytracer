pub mod point;

use nalgebra::Point3;

pub trait Light {
    fn get_intensity(&self) -> f32 {
        unreachable!()
    }
    fn get_position(&self) -> Point3<f32> {
        unreachable!()
    }
}
