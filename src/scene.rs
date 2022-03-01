use crate::ray::Ray;
use crate::{camera::Camera, light::Light, object::Object};
use image::{ImageBuffer, Pixel, Rgb, RgbImage};

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Box<dyn Light>>,
    pub camera: Camera,
}

impl Scene {
    fn cast_ray(&self, ray: &Ray) -> Option<(f32, &Object)> {
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

                if let Some((d, obj)) = self.cast_ray(&ray) {
                    let inter_p = ray.origin + ray.direction.normalize() * d;

                    for light in &self.lights {
                        let light_ray = Ray {
                            origin: inter_p,
                            direction: (light.get_position() - inter_p).normalize(),
                        };

                        let n = obj.normal(inter_p);

                        let d = inter_p - ray.origin;
                        let s = d - 2.0 * (d.dot(&n)) * n;

                        let (kd, ks, _ka) = obj.texture.propeties(inter_p);
                        let nl = n.dot(&light_ray.direction);
                        let li = light.get_intensity();

                        let id = kd * nl * li;
                        let is = ks * li * s.dot(&light_ray.direction).powi(5);

                        let mut pixel = obj.texture.color(inter_p);
                        pixel.apply(|c| ((c as f32 * id + is) as u8).clamp(0, 255));

                        img.put_pixel(x, y, pixel);
                    }
                }
            }
        }

        img
    }
}
