extern crate nalgebra as na;
use image::{ImageBuffer, Pixel, Rgb, RgbImage};

use crate::ray::Ray;
use crate::{camera::Camera, light::Light, object::Object};

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
    pub camera: Camera,
}

impl Scene {
    pub fn save_buffer(&self, width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = RgbImage::from_pixel(width, height, Rgb([255, 255, 255]));

        let d = 1.0 / (self.camera.fov_x / 2.0).tan();
        let aspect_ratio = width as f32 / height as f32;

        let camera_basis = na::Rotation3::from_basis_unchecked(&[
            self.camera.right,
            self.camera.up,
            self.camera.forward,
        ]);

        for x in 0..width {
            for y in 0..height {
                let ndc_ray_dir = na::vector![
                    ((2.0 * (x as f32 + 0.5) / width as f32) - 1.0) * aspect_ratio,
                    -((2.0 * (y as f32 + 0.5) / height as f32) - 1.0),
                    d
                ];

                let ray = Ray {
                    origin: self.camera.position,
                    direction: camera_basis * ndc_ray_dir,
                };

                let mut min: Option<f32> = None;
                let mut closest_obj: Option<&Box<dyn Object>> = None;

                for object in &self.objects {
                    let inter = object.intersects(&ray);
                    match inter {
                        Some(i) => {
                            if min.is_none() || min.is_some() && i < min.unwrap() {
                                min = inter;
                                closest_obj = Some(object);
                            }
                        }
                        None => (),
                    }
                }

                if min.is_some() && closest_obj.is_some() {
                    let obj = closest_obj.unwrap();
                    let intersection_point =
                        self.camera.position + ray.direction.normalize() * min.unwrap();

                    for light in &self.lights {
                        let light_ray = light.get_position() - intersection_point;
                        let coef = obj.normal(intersection_point).dot(&light_ray);

                        let mut color = obj.find_texture(intersection_point);
                        color.apply(|f| (f as f32 * coef).clamp(0.0, 255.0) as u8);
                        img.put_pixel(x, y, color);
                    }
                }
            }
        }

        img
    }
}
