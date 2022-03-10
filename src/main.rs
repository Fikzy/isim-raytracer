mod camera;
mod light;
mod object;
mod ray;
mod scene;
mod shape;
mod texture;
mod utils;

extern crate nalgebra as na;
use light::Light;
use object::Object;
use shape::{plane::Plane, sphere::Sphere, triangle::Triangle};
use texture::uniform::UniformTexture;

fn main() {
    let width = 1920 * 1;
    let height = 1080 * 1;

    let scene = scene::Scene {
        objects: vec![
            // Object {
            //     position: na::point![0.0, 0.5, 0.0],
            //     shape: Box::new(Sphere::new(0.5)),
            //     texture: Box::new(UniformTexture::new(1.0, 0.9, 1.0, na::vector![0, 255, 0])),
            // },
            Object {
                position: na::point![-2.0, 1.0, 0.0],
                shape: Box::new(Sphere::new(1.0)),
                texture: Box::new(UniformTexture::new(1.0, 0.9, 1.0, na::vector![255, 255, 0])),
            },
            Object {
                position: na::point![2.0, 0.75, 0.0],
                shape: Box::new(Sphere::new(0.75)),
                texture: Box::new(UniformTexture::new(1.0, 0.1, 1.0, na::vector![255, 0, 255])),
            },
            Object {
                position: na::Point3::origin(),
                shape: Box::new(Triangle::new([
                    na::point![0.0, 0.0, 0.0],
                    na::point![-0.5, 1.0, 0.0],
                    na::point![0.5, 1.0, 1.0],
                ])),
                texture: Box::new(UniformTexture::new(1.0, 0.1, 1.0, na::vector![255, 0, 255])),
            },
            Object {
                position: na::Point3::origin(),
                shape: Box::new(Triangle::new([
                    na::point![-0.5, 1.0, 0.5],
                    na::point![0.5, 1.0, -0.5],
                    na::point![0.0, 2.0, 0.0],
                ])),
                texture: Box::new(UniformTexture::new(1.0, 0.1, 1.0, na::vector![255, 0, 255])),
            },
            Object {
                position: na::point![0.0, 0.0, 0.0],
                shape: Box::new(Plane::new(na::vector![0.0, 1.0, 0.0])),
                texture: Box::new(UniformTexture::new(
                    1.0,
                    0.1,
                    1.0,
                    na::vector![255, 255, 255],
                )),
            },
        ],
        lights: vec![
            Light::PointLight {
                position: na::point![1.0, 3.0, -3.0],
                intensity: 2.0,
                // radius: 1.0,
            },
            Light::PointLight {
                position: na::point![1.0, 3.0, 3.0],
                intensity: 2.0,
                // radius: 1.0,
            },
        ],
        camera: camera::Camera::new(
            na::point![0.0, 3.0, -5.0],
            na::vector![0.0, 1.0, 0.0],
            na::point![0.0, 0.5, 0.0],
            90.0,
        ),
    };

    let now = std::time::Instant::now();

    scene.save_buffer(width, height).save("test.png").unwrap();

    let elapsed_time = now.elapsed();
    println!("Took {} seconds.", elapsed_time.as_millis() as f32 / 1000.0);
}
