extern crate nalgebra as na;

mod camera;
mod light;
mod object;
mod ray;
mod texture;

use object::SceneObjectTrait;

fn main() {
    let img = image::ImageBuffer::from_fn(32, 32, |x, y| {
        if x % 2 == 0 || y % 2 == 0 {
            image::Rgb([0u8, 0u8, 0u8])
        } else {
            image::Rgb([255u8, 255u8, 255u8])
        }
    });

    img.save("test.ppm").unwrap();

    let sphere = object::sphere::Sphere {
        center: na::point![2.0, 2.0, 2.0],
        radius: 1.0,
        object: object::SceneObject {
            texture: texture::uniform::UniformTexture {
                kd: 0.0,
                ks: 0.0,
                ka: 0.0,
            },
        },
    };

    let ray = ray::Ray {
        origin: na::point![0.0, 0.0, 0.0],
        direction: na::vector![1.0, 1.0, 1.0],
    };

    println!("{:?}", sphere);
    println!("{:?}", ray);

    match sphere.intersects(ray) {
        Some(t) => println!("Distance: {}", t),
        None => println!("No intersection"),
    }
}
