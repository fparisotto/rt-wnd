mod camera;
mod hittable;
mod material;
mod ray;
mod render;
mod scene;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::vec3::Vec3;

use rayon::prelude::*;

fn main() {
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = scene::random_scene();

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

    let mut coords: Vec<(u32, u32)> = Vec::new();
    for y in (0..image_height).rev() {
        for x in 0..image_width {
            coords.push((x, y));
        }
    }

    let pixels: Vec<Vec3> = coords
        .into_par_iter()
        .map(|coords| {
            render::render_pixel(
                coords,
                (image_width, image_height),
                &cam,
                &world,
                max_depth,
                samples_per_pixel,
            )
        })
        .collect();

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for pixel in pixels {
        println!(
            "{} {} {}",
            (256.0 * pixel.x().clamp(0.0, 0.999)) as u32,
            (256.0 * pixel.y().clamp(0.0, 0.999)) as u32,
            (256.0 * pixel.z().clamp(0.0, 0.999)) as u32
        );
    }

    eprintln!("Done.");
}
