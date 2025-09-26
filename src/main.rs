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

use anyhow::Context;
use rand::prelude::*;
use raylib::prelude::*;
use rayon::prelude::*;
use std::fmt::Write;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() -> anyhow::Result<()> {
    // Image
    let aspect_ratio: f32 = 3.0 / 2.0;
    let image_width = 1200;
    let image_height: u32 = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 10;

    // World
    let world = scene::random_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Generate coordinates
    let mut coords: Vec<(u32, u32)> = (0..image_width)
        .flat_map(|x| (0..image_height).map(move |y| (x, y)))
        .collect();
    coords.shuffle(&mut rand::rng());

    let (sender, receiver) = mpsc::channel::<(Vec3, (u32, u32))>();

    let render_start_time = Instant::now();

    // Rendering thread
    thread::spawn(move || {
        coords.into_par_iter().for_each(|coords| {
            let pixel = render::render_pixel(
                coords,
                (image_width, image_height),
                &camera,
                &world,
                max_depth,
                samples_per_pixel,
            );
            sender.send((pixel, coords)).expect("send rendered pixel");
        });
    });

    let (mut rl, thread) = raylib::init()
        .width(image_width as i32)
        .height(image_height as i32)
        .title("RTWND")
        .resizable()
        .build();
    rl.set_target_fps(60);

    let mut image = Image::gen_image_color(image_width as i32, image_height as i32, Color::BLACK);
    let mut texture = rl
        .load_texture_from_image(&thread, &image)
        .context("Failed to create initial texture")?;

    let mut pixels_rendered = 0;
    let total_pixels = (image_width * image_height) as usize;
    let mut rendering_complete = false;
    let mut render_time: Option<std::time::Duration> = None;
    let mut last_texture_update = Instant::now();
    let mut text_buffer = String::with_capacity(64); // Reusable buffer for text

    const UPDATE_INTERVAL: Duration = Duration::from_millis(100);

    while !rl.window_should_close() {
        for (pixel, (x, y)) in receiver.try_iter() {
            let red = (256.0 * pixel.x().clamp(0.0, 0.999)) as u8;
            let green = (256.0 * pixel.y().clamp(0.0, 0.999)) as u8;
            let blue = (256.0 * pixel.z().clamp(0.0, 0.999)) as u8;

            // flip y-axis for correct rendering
            let y = image_height - y - 1;

            image.draw_pixel(x as i32, y as i32, Color::new(red, green, blue, 255));
            pixels_rendered += 1;
        }

        let time_for_update = last_texture_update.elapsed() >= UPDATE_INTERVAL;
        let just_completed = pixels_rendered >= total_pixels && !rendering_complete;
        if time_for_update || just_completed {
            texture = rl
                .load_texture_from_image(&thread, &image)
                .context("Failed to update texture")?;
            last_texture_update = Instant::now();

            if just_completed {
                rendering_complete = true;
                render_time = Some(render_start_time.elapsed());
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_texture(&texture, 0, 0, Color::WHITE);

        if rendering_complete {
            if let Some(duration) = render_time {
                let seconds = duration.as_secs();
                let millis = duration.subsec_millis();
                text_buffer.clear();
                write!(
                    &mut text_buffer,
                    "Rendering complete! Time: {}.{:03}s",
                    seconds, millis
                )
                .ok();
                d.draw_text(&text_buffer, 10, 10, 20, Color::GREEN);
            } else {
                d.draw_text("Rendering complete!", 10, 10, 20, Color::GREEN);
            }
        } else {
            let progress = (pixels_rendered as f32 / total_pixels as f32) * 100.0;
            text_buffer.clear();
            write!(&mut text_buffer, "Rendering: {:.1}%", progress).ok();
            d.draw_text(&text_buffer, 10, 10, 20, Color::YELLOW);
        }
    }

    Ok(())
}
