use super::Light;
use nalgebra::Point3;

#[derive(Debug)]
pub struct PointLight {
    pub intensity: f32,
    pub position: Point3<f32>,
}

impl Light for PointLight {
    fn get_intensity(&self) -> f32 {
        self.intensity
    }
    fn get_position(&self) -> Point3<f32> {
        self.position
    }
}
