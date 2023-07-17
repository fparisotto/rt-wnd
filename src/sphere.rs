use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, m: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt = discriminant.sqrt();

        let mut root = (-half_b - sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec = HitRecord::empty();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = Some(self.mat_ptr.clone());
        Some(rec)
    }
}
