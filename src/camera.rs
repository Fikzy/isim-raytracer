extern crate nalgebra as na;
use na::{Point3, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub up: Vector3<f32>,
    pub forward: Vector3<f32>,
    pub right: Vector3<f32>,
    pub fov_x: f32, // alpha
    pub fov_y: f32, // beta
    pub z_min: f32,
}

impl Camera {
    pub fn new(
        center: Point3<f32>,
        up: Vector3<f32>,
        target: Point3<f32>,
        fov_x: f32,
        fov_y: f32,
        z_min: f32,
    ) -> Camera {
        let forward: Vector3<f32> = (target - center).normalize();
        let right: Vector3<f32> = up.cross(&forward).normalize();
        let up = forward.cross(&right).normalize(); // override user up
        Camera {
            position: center,
            up,
            forward,
            right,
            fov_x,
            fov_y,
            z_min,
        }
    }
}
