mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

extern crate num_cpus;
use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use rand::prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    let mut rng = thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let rng_x: f64 = random();
            let rng_z: f64 = random();
            let center: Vec3 = Vec3::new(a as f64 + 0.9 * rng_x, 0.2, b as f64 + 0.9 * rng_z);

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // difuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    return world;
}

fn main() {
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // let aspect_ratio: f64 = 3.0 / 2.0;
    // let image_width = 200;
    // let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    // let samples_per_pixel = 10;
    // let max_depth = 2;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    // for j in (0..image_height).rev() {
    //     eprintln!("Scanlines remaining: {}", j);
    //     for i in 0..image_width {
    //         let mut pixel_color: Vec3 = Vec3::empty();
    //         for _ in 0..samples_per_pixel {
    //             let u_rng: f64 = random();
    //             let v_rng: f64 = random();
    //             let u: f64 = (i as f64 + u_rng) / (image_width as f64 - 1.0);
    //             let v: f64 = (j as f64 + v_rng) / (image_height as f64 - 1.0);
    //             let r = cam.get_ray(u, v);
    //             pixel_color += ray_color(&r, &world, max_depth);
    //         }
    //         write_color(&pixel_color, samples_per_pixel);
    //     }
    // }

    let cam_arc = Arc::new(cam);
    let world_arc = Arc::new(world);

    let mut tasks: VecDeque<(usize, u32, u32)> = VecDeque::new();
    let mut pixel_order: usize = 0;
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            tasks.push_back((pixel_order, i, j));
            pixel_order += 1;
        }
    }

    let results = vec![Vec3::empty(); tasks.len()];
    let results_mutex = Arc::new(Mutex::new(results));

    let tasks_mutex = Arc::new(Mutex::new(tasks));

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    for _ in 0..num_cpus::get() {
        let tasks_clone = Arc::clone(&tasks_mutex);
        let results_clone = Arc::clone(&results_mutex);
        let cam_arc_clone = Arc::clone(&cam_arc);
        let world_arc_clone = Arc::clone(&world_arc);
        let handle = thread::spawn(move || loop {
            let task = { tasks_clone.lock().unwrap().pop_front() };
            match task {
                Some((index, i, j)) => {
                    let pixel = render_pixel(
                        samples_per_pixel,
                        max_depth,
                        image_width,
                        image_height,
                        i,
                        j,
                        cam_arc_clone.as_ref(),
                        world_arc_clone.as_ref(),
                    );
                    let mut result_ref = results_clone.lock().unwrap();
                    result_ref[index] = pixel;
                }
                None => break,
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    for pixel in &*results_mutex.lock().unwrap() {
        write_color(&pixel, samples_per_pixel);
    }

    eprintln!("Done.");
}

fn render_pixel(
    samples_per_pixel: u32,
    max_depth: u32,
    image_width: u32,
    image_height: u32,
    i: u32,
    j: u32,
    cam: &Camera,
    world: &HittableList,
) -> Vec3 {
    let mut pixel_color: Vec3 = Vec3::empty();
    for _ in 0..samples_per_pixel {
        let u_rng: f64 = random();
        let v_rng: f64 = random();
        let u: f64 = (i as f64 + u_rng) / (image_width as f64 - 1.0);
        let v: f64 = (j as f64 + v_rng) / (image_height as f64 - 1.0);
        let r = cam.get_ray(u, v);
        pixel_color += ray_color(&r, &world, max_depth);
    }
    return pixel_color;
}
