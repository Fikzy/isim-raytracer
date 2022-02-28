extern crate nalgebra as na;

mod camera;
mod light;
mod object;
mod ray;
mod scene;
mod texture;

fn main() {
    let width = 1920 / 2;
    let height = 1080 / 2;

    let scene = scene::Scene {
        objects: vec![
            Box::new(object::sphere::Sphere {
                center: na::point![-3.0, 0.0, 0.0],
                radius: 0.5,
                texture: texture::uniform::UniformTexture {
                    kd: 1.0,
                    ks: 1.0,
                    ka: 1.0,
                    color: image::Rgb([66, 135, 245]),
                },
            }),
            Box::new(object::sphere::Sphere {
                center: na::point![-2.0, 1.0, 0.0],
                radius: 0.5,
                texture: texture::uniform::UniformTexture {
                    kd: 1.0,
                    ks: 1.0,
                    ka: 1.0,
                    color: image::Rgb([66, 135, 245]),
                },
            }),
            Box::new(object::sphere::Sphere {
                center: na::point![-1.0, 0.0, 0.0],
                radius: 0.5,
                texture: texture::uniform::UniformTexture {
                    kd: 1.0,
                    ks: 1.0,
                    ka: 1.0,
                    color: image::Rgb([66, 135, 245]),
                },
            }),
            Box::new(object::sphere::Sphere {
                center: na::point![0.0, -1.0, 0.0],
                radius: 0.5,
                texture: texture::uniform::UniformTexture {
                    kd: 1.0,
                    ks: 1.0,
                    ka: 1.0,
                    color: image::Rgb([66, 135, 245]),
                },
            }),
            Box::new(object::sphere::Sphere {
                center: na::point![1.0, 0.0, 0.0],
                radius: 0.5,
                texture: texture::uniform::UniformTexture {
                    kd: 1.0,
                    ks: 1.0,
                    ka: 1.0,
                    color: image::Rgb([66, 135, 245]),
                },
            }),
            Box::new(object::sphere::Sphere {
                center: na::point![2.0, 1.0, 0.0],
                radius: 0.5,
                texture: texture::uniform::UniformTexture {
                    kd: 1.0,
                    ks: 1.0,
                    ka: 1.0,
                    color: image::Rgb([66, 135, 245]),
                },
            }),
            Box::new(object::sphere::Sphere {
                center: na::point![3.0, 0.0, 0.0],
                radius: 0.5,
                texture: texture::uniform::UniformTexture {
                    kd: 1.0,
                    ks: 1.0,
                    ka: 1.0,
                    color: image::Rgb([66, 135, 245]),
                },
            }),
        ],
        lights: vec![Box::new(light::point::PointLight {
            intensity: 1.0,
            position: na::point![0.0, 0.0, -3.0],
        })],
        camera: camera::Camera::new(
            na::point![0.0, 0.0, -3.0],
            na::vector![0.0, 1.0, 0.0],
            na::point![0.0, 0.0, 0.0],
            90.0,
        ),
    };

    scene.save_buffer(width, height).save("test.png").unwrap();
}
