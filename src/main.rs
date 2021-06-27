mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use rand::prelude::*;
use std::rc::Rc;

fn write_color(v: &Vec3, samples_per_pixel: u32) {
    let mut r = v.x();
    let mut g = v.y();
    let mut b = v.z();
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();
    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as u32,
        (256.0 * g.clamp(0.0, 0.999)) as u32,
        (256.0 * b.clamp(0.0, 0.999)) as u32
    );
}

fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::empty();
    }
    if let Some(rec) = world.hit(r, 0.001, std::f64::INFINITY) {
        if let Some(mat) = rec.mat_ptr.as_ref() {
            if let Some(scatter_rec) = mat.scatter(&r, &rec) {
                return scatter_rec.attenuation
                    * ray_color(&scatter_rec.scattered, &world, depth - 1);
            }
        }
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 600;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    // let material_left = Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0)));
    // let material_right = Rc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0)));
    // let R = (std::f64::consts::PI / 4.0).cos();
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(-R, 0.0, -1.0),
    //     R,
    //     material_left.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(R, 0.0, -1.0),
    //     R,
    //     material_right.clone(),
    // )));

    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

    // Camera
    let cam = Camera::new(
        &Vec3::new(-2.0, 2.0, 1.0),
        &Vec3::new(0.0, 0.0, -1.0),
        &Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
    );

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color: Vec3 = Vec3::empty();
            for _ in 0..samples_per_pixel {
                let u_rng: f64 = random();
                let v_rng: f64 = random();
                let u: f64 = (i as f64 + u_rng) / (image_width as f64 - 1.0);
                let v: f64 = (j as f64 + v_rng) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(&pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Done.");
}
