use crate::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};
use rand::random;

pub fn render_pixel(
    (x, y): (u32, u32),
    (width, height): (u32, u32),
    cam: &Camera,
    world: &HittableList,
    max_depth: u32,
    samples_per_pixel: u32,
) -> Vec3 {
    let mut pixel_color = Vec3::empty();

    // Pre-compute reciprocals and constants
    let inv_width_minus_one = 1.0 / (width as f32 - 1.0);
    let inv_height_minus_one = 1.0 / (height as f32 - 1.0);
    let x_f32 = x as f32;
    let y_f32 = y as f32;
    let inv_samples = 1.0 / samples_per_pixel as f32;

    for _ in 0..samples_per_pixel {
        let u_rng: f32 = random();
        let v_rng: f32 = random();
        let u = (x_f32 + u_rng) * inv_width_minus_one;
        let v = (y_f32 + v_rng) * inv_height_minus_one;
        let r = cam.get_ray(u, v);
        pixel_color += ray_color(r, world, max_depth);
    }

    pixel_color *= inv_samples;

    // Apply gamma correction in place
    let r = pixel_color.x().sqrt();
    let g = pixel_color.y().sqrt();
    let b = pixel_color.z().sqrt();
    Vec3::new(r, g, b)
}

fn ray_color(r: Ray, world: &HittableList, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::empty();
    }
    if let Some(rec) = world.hit(r, 0.001, f32::INFINITY)
        && let Some(mat) = &rec.mat
        && let Some(scatter_rec) = mat.scatter(r, &rec)
    {
        return scatter_rec.attenuation * ray_color(scatter_rec.scattered, world, depth - 1);
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
