use crate::hittable::{HitRecord, Hittable};
use crate::material::Materials;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    #[allow(dead_code)]
    radius: f32,
    radius_squared: f32,
    inv_radius: f32,
    mat: Materials,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, m: Materials) -> Sphere {
        Sphere {
            center,
            radius,
            radius_squared: radius * radius,
            inv_radius: 1.0 / radius,
            mat: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius_squared;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let inv_a = 1.0 / a;

        let mut root = (-half_b - sqrt_discriminant) * inv_a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_discriminant) * inv_a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) * self.inv_radius; // Use pre-computed inverse radius
        let front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Some(HitRecord {
            p,
            normal,
            mat: Some(&self.mat),
            t,
            front_face,
        })
    }
}
