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
use shape::sphere::Sphere;
use texture::uniform::UniformTexture;

fn main() {
    let width = 1920 / 2;
    let height = 1080 / 2;

    let mut spheres: Vec<Object> = Vec::new();
    let w = 9;
    let h = 5;

    for i in 0..w {
        for j in 0..h {
            spheres.push(Object {
                position: na::point![(i - w / 2) as f32, (j - h / 2) as f32, 0.0],
                shape: Box::new(Sphere::new(0.5)),
                texture: Box::new(UniformTexture::new(
                    (1.0 + i as f32) / (1.0 + w as f32),
                    (1.0 + j as f32) / (1.0 + h as f32),
                    1.0,
                    Rgb([0, 255, 0]),
                )),
            });
        }
    }

    let scene = scene::Scene {
        objects: spheres,
        lights: vec![Box::new(light::point::PointLight {
            intensity: 1.0,
            position: na::point![0.0, 0.0, -3.0],
        })],
        camera: camera::Camera::new(
            na::point![0.0, 0.0, -5.0],
            na::vector![0.0, 1.0, 0.0],
            na::point![0.0, 0.0, 0.0],
            90.0,
        ),
    };

    scene.save_buffer(width, height).save("test.png").unwrap();
}
