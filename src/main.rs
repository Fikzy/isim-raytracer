extern crate nalgebra as na;

mod camera;
mod light;
mod object;
mod ray;
mod scene;
mod texture;

use std::f32::consts::FRAC_PI_2;

fn main() {
    let width = 1920 / 2;
    let height = 1080 / 2;

    let scene = scene::Scene {
        objects: vec![
            Box::new(object::sphere::Sphere {
                center: na::point![0.0, 0.0, 0.0],
                radius: 0.5,
                texture: texture::uniform::UniformTexture {
                    kd: 1.0,
                    ks: 0.1,
                    ka: 1.0,
                    color: image::Rgb([66, 135, 245]),
                },
            }),
            Box::new(object::sphere::Sphere {
                center: na::point![-3.0, 0.0, 3.0],
                radius: 2.0,
                texture: texture::uniform::UniformTexture {
                    kd: 1.0,
                    ks: 0.1,
                    ka: 1.0,
                    color: image::Rgb([227, 66, 245]),
                },
            }),
        ],
        lights: vec![Box::new(light::point::PointLight {
            intensity: 1.0,
            position: na::point![3.0, 0.0, -3.0],
        })],
        camera: camera::Camera::new(
            na::point![0.0, 0.0, -3.0],
            na::vector![0.0, 1.0, 0.0],
            na::point![0.0, 0.0, 0.0],
            FRAC_PI_2,
            FRAC_PI_2,
            1.0,
        ),
    };

    scene.save_buffer(width, height).save("test.png").unwrap();
}
