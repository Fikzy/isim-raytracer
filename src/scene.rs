extern crate nalgebra as na;
use image::{Rgb, RgbImage};

use crate::ray::Ray;
use crate::{camera::Camera, light::Light, object::Object};

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
    pub camera: Camera,
}

impl Scene {
    pub fn save(&self, width: u32, height: u32) {
        let mut img = RgbImage::from_pixel(width, height, Rgb([255, 255, 255]));

        for x in 0..width {
            for y in 0..height {
                let d = 1.0 / (self.camera.fov_x / 2.0).tan();
                let aspect_ratio = width as f32 / height as f32;

                let p_x = x as f32 + 0.5;
                let p_y = y as f32 + 0.5;

                let ray = Ray {
                    origin: self.camera.position,
                    direction: na::vector![
                        ((2.0 * p_x / width as f32) - 1.0) * aspect_ratio,
                        1.0 - (2.0 * p_y / height as f32),
                        d
                    ],
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

                if closest_obj.is_some() {
                    img.put_pixel(x, y, closest_obj.unwrap().get_color());
                }
            }
        }

        img.save("test.png").unwrap();
    }
}
