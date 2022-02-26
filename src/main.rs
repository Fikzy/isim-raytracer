extern crate nalgebra as na;

mod camera;
mod light;
mod object;
mod ray;
mod scene;
mod texture;

use std::f32::consts::FRAC_PI_2;

fn main() {
    let scene = scene::Scene {
        objects: vec![
            Box::new(object::sphere::Sphere {
                center: na::point![0.0, 0.0, 2.0],
                radius: 0.5,
                texture: texture::uniform::UniformTexture {
                    kd: 0.0,
                    ks: 0.0,
                    ka: 0.0,
                    color: image::Rgb([255, 0, 0]),
                },
            }),
            Box::new(object::sphere::Sphere {
                center: na::point![1.0, 1.0, 2.0],
                radius: 0.5,
                texture: texture::uniform::UniformTexture {
                    kd: 0.0,
                    ks: 0.0,
                    ka: 0.0,
                    color: image::Rgb([0, 255, 0]),
                },
            }),
            Box::new(object::sphere::Sphere {
                center: na::point![-3.0, 0.0, 2.0],
                radius: 1.0,
                texture: texture::uniform::UniformTexture {
                    kd: 0.0,
                    ks: 0.0,
                    ka: 0.0,
                    color: image::Rgb([0, 0, 255]),
                },
            }),
        ],
        lights: vec![],
        camera: camera::Camera::new(
            na::point![0.0, 0.0, 0.0],
            na::vector![0.0, 1.0, 0.0],
            na::point![0.0, 0.0, 1.0],
            FRAC_PI_2,
            FRAC_PI_2,
            1.0,
        ),
    };

    scene.save(256, 128);
}
