use nalgebra::Point3;

pub mod point;

pub trait Light {
    fn get_intensity(&self) -> f32 {
        unreachable!()
    }
    fn get_position(&self) -> Point3<f32> {
        unreachable!()
    }
}
