use nalgebra::{Point3, Vector3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}
