use nalgebra::{Matrix3, Point3, Vector3};
extern crate nalgebra as na;
use super::Shape;

pub struct Triangle {
    pub points: [Point3<f32>; 3],
}

impl Triangle {
    pub fn new(points: [Point3<f32>; 3]) -> Self {
        Triangle { points }
    }
}

impl Shape for Triangle {
    fn intersects(&self, _origin: &Point3<f32>, ray: &crate::ray::Ray) -> Option<f32> {
        let normal = self.normal(&Point3::origin(), &Point3::origin(), ray);
        if ray.direction.dot(&normal) == 0.0 {
            return None;
        }

        let o_vectors = self.points.map(|p| p - ray.origin);
        let mut o_basis = Matrix3::<f32>::from_element(0.0);
        for (i, v) in o_vectors.into_iter().enumerate() {
            o_basis.set_column(i, &v);
        }

        if let Some(inv) = o_basis.try_inverse() {
            let coefs = inv * ray.direction;
            if coefs.into_iter().all(|c| c >= &0.0) || coefs.into_iter().all(|c| c <= &0.0) {
                let og = (coefs.x * o_vectors[0] + coefs.y * o_vectors[1] + coefs.z * o_vectors[2])
                    / coefs.sum();
                Some(og.norm())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn normal(
        &self,
        _origin: &nalgebra::Point3<f32>,
        _point: &nalgebra::Point3<f32>,
        ray: &crate::ray::Ray,
    ) -> Vector3<f32> {
        let ab = self.points[1] - self.points[0];
        let ac = self.points[2] - self.points[0];
        let normal = ac.cross(&ab).normalize();
        normal * -normal.dot(&ray.direction).signum()
    }
}
