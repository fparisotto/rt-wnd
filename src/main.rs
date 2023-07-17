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

fn main() {
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 200;
    let max_depth = 10;

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

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for y in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", y);
        for x in 0..image_width {
            let pixel = render::render_pixel(
                x,
                y,
                image_width,
                image_height,
                &cam,
                &world,
                max_depth,
                samples_per_pixel,
            );
            println!(
                "{} {} {}",
                (256.0 * pixel.x().clamp(0.0, 0.999)) as u32,
                (256.0 * pixel.y().clamp(0.0, 0.999)) as u32,
                (256.0 * pixel.z().clamp(0.0, 0.999)) as u32
            );
        }
    }

    eprintln!("Done.");
}
