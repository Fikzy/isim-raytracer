use crate::ray::Ray;
use crate::{camera::Camera, light::Light, object::Object};
use image::{ImageBuffer, Rgb, RgbImage};
use nalgebra::Vector3;

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Box<dyn Light>>,
    pub camera: Camera,
}

const NS: i32 = 50;
const REFLECT_ITER: u8 = 15;

impl Scene {
    fn check_intersection(&self, ray: &Ray) -> Option<(f32, &Object)> {
        let mut intersection: Option<(f32, &Object)> = None;

        for object in &self.objects {
            if let Some(d) = object.intersects(&ray) {
                if intersection.is_none() || d < intersection.unwrap().0 {
                    intersection = Some((d, object));
                }
            }
        }

        return intersection;
    }

    fn cast_ray(&self, ray: &Ray, depth: f32, iter: u8) -> Vector3<f32> {
        let mut color = Vector3::from_element(0.0);

        if let Some((d, obj)) = self.check_intersection(&ray) {
            let inter_p = ray.origin + ray.direction.normalize() * d;

            for light in &self.lights {
                let lr = light.get_position() - inter_p;
                let lr_dist = lr.norm();
                let lr_dir = lr.normalize();

                let light_ray = Ray {
                    origin: inter_p + lr_dir * 0.0001,
                    direction: lr_dir,
                };

                let normal = obj.normal(inter_p);

                let reflect_dir =
                    (ray.direction - 2.0 * (ray.direction.dot(&normal)) * normal).normalize();

                let (kd, ks, _ka) = obj.texture.propeties(inter_p);
                let nl = normal.dot(&lr_dir);
                let mut li = light.get_intensity();

                if let Some((lr_inter_d, _)) = self.check_intersection(&light_ray) {
                    if lr_inter_d <= lr_dist {
                        li = 0.0;
                    }
                }

                let id = kd * obj.texture.color(inter_p) * nl * li;
                let is = Vector3::from_element(ks * li * reflect_dir.dot(&lr_dir).powi(NS) * 255.0);

                let mut p_color = id + is;
                p_color *= 1.0 / (depth + d);

                if iter < REFLECT_ITER {
                    let reflect_ray = Ray {
                        origin: inter_p + reflect_dir * 0.0001,
                        direction: reflect_dir,
                    };
                    let s_color = self.cast_ray(&reflect_ray, depth + d, iter + 1);

                    p_color += ks as f32 * s_color;
                }

                color += p_color;
            }
        }

        color.apply(|c| *c = c.clamp(0.0, 255.0));
        color
    }

    pub fn save_buffer(&self, width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = RgbImage::from_pixel(width, height, Rgb([0, 0, 0]));

        let forward = self.camera.forward;
        let right = self.camera.right;
        let up = self.camera.up;

        let gx = (self.camera.fov.to_radians() / 2.0).tan();
        let gy = gx * ((height - 1) as f32 / (width - 1) as f32);

        let qx = 2.0 * gx / ((width - 1) as f32) * right;
        let qy = 2.0 * gy / ((height - 1) as f32) * up;

        let p_top_left = self.camera.position + forward - gx * right + gy * up;

        for x in 0..width {
            for y in 0..height {
                let p_pixel = p_top_left + qx * (x as f32) - qy * (y as f32);
                let ray = Ray {
                    origin: self.camera.position,
                    direction: (p_pixel - self.camera.position).normalize(),
                };

                let color = self.cast_ray(&ray, 0.0, 0);
                img.put_pixel(x, y, Rgb([color.x as u8, color.y as u8, color.z as u8]));
            }
        }

        img
    }
}
