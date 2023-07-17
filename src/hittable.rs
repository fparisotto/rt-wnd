use crate::material::Materials;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Option<Materials>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            p: Vec3::empty(),
            normal: Vec3::empty(),
            mat: None,
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub enum HittableEnum {
    Sphere(Sphere),
    // HittableList(HittableList),
}

impl Hittable for HittableEnum {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            HittableEnum::Sphere(s) => s.hit(r, t_min, t_max),
            // HittableEnum::HittableList(hl) => hl.hit(r, t_min, t_max),
        }
    }
}

pub struct HittableList {
    objects: Vec<HittableEnum>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.objects.push(HittableEnum::Sphere(sphere))
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit)
            }
        }
        hit_anything
    }
}
