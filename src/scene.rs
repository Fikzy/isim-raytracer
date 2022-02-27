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
    fn cast_ray(&self, ray: &Ray) -> Option<(f32, &Box<dyn Object>)> {
        let mut intersection: Option<(f32, &Box<dyn Object>)> = None;

        for object in &self.objects {
            if let Some(d) = object.intersects(&ray) {
                if intersection.is_none() || d < intersection.unwrap().0 {
                    intersection = Some((d, object));
                }
            }
        }

        return intersection;
    }

    pub fn save_buffer(&self, width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = RgbImage::from_pixel(width, height, Rgb([0, 0, 0]));

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

                if let Some((d, obj)) = self.cast_ray(&ray) {
                    let intersection_point = self.camera.position + ray.direction.normalize() * d;

                    for light in &self.lights {
                        let kd = obj.get_diffusion(intersection_point);
                        let mut c = obj.get_color(intersection_point);
                        let light_ray = (light.get_position() - intersection_point).normalize();
                        let nl = obj.normal(intersection_point).dot(&light_ray);
                        let li = light.get_intensity();

                        c.apply(|f| ((kd * f as f32 * nl * li) as u8).clamp(0, 255));

                        img.put_pixel(x, y, c);
                    }
                }
            }
        }

        img
    }
}
