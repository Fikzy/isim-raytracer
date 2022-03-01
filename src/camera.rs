use nalgebra::{Point3, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub up: Vector3<f32>,
    pub forward: Vector3<f32>,
    pub right: Vector3<f32>,
    pub fov: f32,
}

impl Camera {
    pub fn new(center: Point3<f32>, up: Vector3<f32>, target: Point3<f32>, fov: f32) -> Camera {
        let forward: Vector3<f32> = (target - center).normalize();
        let right: Vector3<f32> = up.cross(&forward).normalize();
        let up = forward.cross(&right).normalize(); // override user up
        Camera {
            position: center,
            up,
            forward,
            right,
            fov,
        }
    }
}
