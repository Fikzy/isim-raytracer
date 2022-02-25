extern crate nalgebra as na;
use na::{Point3, Vector3};

pub struct Camera {
    pub center: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub fov_x: f32,
    pub fov_y: f32,
    pub z_min: f32,
}
