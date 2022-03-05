use nalgebra::Point3;

pub enum Light {
    PointLight {
        position: Point3<f32>,
        intensity: f32,
    },
    SphereLight {
        position: Point3<f32>,
        intensity: f32,
        radius: f32,
    },
}

impl Light {
    pub fn get_position(&self) -> Point3<f32> {
        match self {
            &Light::PointLight {
                position,
                intensity: _,
            } => position,
            &Light::SphereLight {
                position,
                intensity: _,
                radius: _,
            } => position,
        }
    }
}
