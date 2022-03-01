mod camera;
mod light;
mod object;
mod ray;
mod scene;
mod shape;
mod texture;

extern crate nalgebra as na;
use image::Rgb;
use object::Object;
use shape::{plane::Plane, sphere::Sphere};
use texture::uniform::UniformTexture;

fn main() {
    let width = 1920 / 2;
    let height = 1080 / 2;

    let scene = scene::Scene {
        objects: vec![
            Object {
                position: na::point![0.0, 0.5, 0.0],
                shape: Box::new(Sphere::new(0.5)),
                texture: Box::new(UniformTexture::new(1.0, 1.0, 1.0, Rgb([0, 255, 0]))),
            },
            Object {
                position: na::point![-2.0, 1.0, 1.0],
                shape: Box::new(Sphere::new(1.0)),
                texture: Box::new(UniformTexture::new(1.0, 1.0, 1.0, Rgb([255, 255, 0]))),
            },
            Object {
                position: na::point![0.0, 0.0, 0.0],
                shape: Box::new(Plane::new(na::vector![0.0, 1.0, 0.0])),
                texture: Box::new(UniformTexture::new(1.0, 0.0, 1.0, Rgb([255, 255, 255]))),
            },
        ],
        lights: vec![Box::new(light::point::PointLight {
            intensity: 1.0,
            position: na::point![1.0, 3.0, -3.0],
        })],
        camera: camera::Camera::new(
            na::point![0.0, 1.0, -5.0],
            na::vector![0.0, 1.0, 0.0],
            na::point![0.0, 0.5, 0.0],
            90.0,
        ),
    };

    scene.save_buffer(width, height).save("test.png").unwrap();
}
