use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

#[derive(Clone)]
pub enum Materials {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { ir: f64 },
}

impl Material for Materials {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Materials::Lambertian { albedo } => lambertian_scatter(albedo, r_in, rec),
            Materials::Metal { albedo, fuzz } => metal_scatter(albedo, *fuzz, r_in, rec),
            Materials::Dielectric { ir } => dielectric_scatter(*ir, r_in, rec),
        }
    }
}

fn lambertian_scatter(albedo: &Vec3, _: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
    let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
    if scatter_direction.near_zero() {
        scatter_direction = rec.normal;
    }
    Some(ScatterRecord {
        attenuation: *albedo,
        scattered: Ray::new(rec.p, scatter_direction),
    })
}

fn metal_scatter(albedo: &Vec3, fuzz: f64, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
    let reflected = Vec3::reflect(&r_in.direction.unit(), &rec.normal);
    let scattered = Ray::new(rec.p, reflected + fuzz * Vec3::random_in_unit_sphere());
    if Vec3::dot(&scattered.direction, &rec.normal) > 0.0 {
        Some(ScatterRecord {
            attenuation: *albedo,
            scattered,
        })
    } else {
        None
    }
}

fn dielectric_scatter(ir: f64, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
    let refraction_ratio = if rec.front_face { 1.0 / ir } else { ir };
    let unit_direction = r_in.direction.unit();
    let cos_theta = Vec3::dot(&-unit_direction, &rec.normal).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    let reflectance = {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
    };
    let direction = if cannot_refract || reflectance > random() {
        Vec3::reflect(&unit_direction, &rec.normal)
    } else {
        Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
    };
    Some(ScatterRecord {
        attenuation: Vec3::new(1.0, 1.0, 1.0),
        scattered: Ray::new(rec.p, direction),
    })
}
