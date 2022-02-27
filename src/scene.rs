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
                    let inter_p = self.camera.position + ray.direction.normalize() * d;

                    for light in &self.lights {
                        let light_ray = Ray {
                            origin: inter_p,
                            direction: (light.get_position() - inter_p).normalize(),
                        };

                        let n = obj.normal(inter_p);

                        let d = inter_p - ray.origin;
                        let s = d - 2.0 * (d.dot(&n)) * n;

                        let (kd, ks, _ka) = obj.get_properties(inter_p);
                        let mut color = obj.get_color(inter_p);
                        let nl = n.dot(&light_ray.direction);
                        let li = light.get_intensity();

                        let id = kd * nl * li;
                        let is = ks * li * s.dot(&light_ray.direction).powi(3);

                        let i = id + is;

                        color.apply(|c| ((c as f32 * i) as u8).clamp(0, 255));

                        img.put_pixel(x, y, color);
                    }
                }
            }
        }

        img
    }
}
