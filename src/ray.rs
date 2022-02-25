extern crate nalgebra as na;
use na::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ray(\n    origin: {},\n    direction: {}\n)",
            self.origin, self.direction
        )
    }
}
