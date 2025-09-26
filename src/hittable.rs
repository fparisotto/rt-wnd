use crate::material::Materials;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Option<&'a Materials>,
    pub t: f32,
    pub front_face: bool,
}


pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>>;
}

pub enum HittableEnum {
    Sphere(Sphere),
    // HittableList(HittableList),
}

impl Hittable for HittableEnum {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
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
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
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
