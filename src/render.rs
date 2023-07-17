use crate::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    ray::Ray,
    vec3::Vec3,
};
use rand::prelude::*;

pub fn render_pixel(
    x: u32,
    y: u32,
    image_width: u32,
    image_height: u32,
    cam: &Camera,
    world: &HittableList,
    max_depth: u32,
    samples_per_pixel: u32,
) -> Vec3 {
    let mut pixel_color: Vec3 = Vec3::empty();
    for _ in 0..samples_per_pixel {
        let u_rng: f64 = random();
        let v_rng: f64 = random();
        let u: f64 = (x as f64 + u_rng) / (image_width as f64 - 1.0);
        let v: f64 = (y as f64 + v_rng) / (image_height as f64 - 1.0);
        let r = cam.get_ray(u, v);
        pixel_color += ray_color(&r, &world, max_depth);
    }
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (scale * pixel_color.x()).sqrt();
    let g = (scale * pixel_color.y()).sqrt();
    let b = (scale * pixel_color.z()).sqrt();
    Vec3::new(r, g, b)
}

fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::empty();
    }
    if let Some(rec) = world.hit(r, 0.001, std::f64::INFINITY) {
        if let Some(mat) = rec.mat_ptr.as_ref() {
            if let Some(scatter_rec) = mat.scatter(r, &rec) {
                return scatter_rec.attenuation
                    * ray_color(&scatter_rec.scattered, world, depth - 1);
            }
        }
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
