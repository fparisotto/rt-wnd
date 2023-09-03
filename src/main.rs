mod camera;
mod hittable;
mod material;
mod ray;
mod render;
mod scene;
mod sphere;
mod vec3;

use std::num::NonZeroU32;
use std::thread;

use crate::camera::Camera;
use crate::vec3::Vec3;

use rand::prelude::*;
use rayon::prelude::*;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
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
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut coords: Vec<(u32, u32)> = Vec::new();
    for x in 0..image_width {
        for y in 0..image_height {
            coords.push((x, y));
        }
    }
    // Shuffle the coordinates to make the rendering more interesting
    coords.shuffle(&mut rand::thread_rng());

    let (sender, receive) = std::sync::mpsc::channel::<(Vec3, (u32, u32))>();

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

    let event_loop = EventLoop::new();
    let size = PhysicalSize::new(image_width, image_height);
    let window = WindowBuilder::new()
        .with_min_inner_size(size)
        .with_max_inner_size(size)
        .with_resizable(false)
        .build(&event_loop)?;

    let context = unsafe { softbuffer::Context::new(&window) }
        .map_err(|err| anyhow!("fail to create context, reason: {:?}", err))?;
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }
        .map_err(|err| anyhow!("fail to create surface, reason: {:?}", err))?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                surface
                    .resize(
                        NonZeroU32::new(width).expect("no width"),
                        NonZeroU32::new(height).expect("no height"),
                    )
                    .expect("resize works");

                let mut buffer = surface.buffer_mut().expect("fetch window buffer");
                for (pixel, (x, y)) in receive.try_iter() {
                    let red = (256.0 * pixel.x().clamp(0.0, 0.999)) as u32;
                    let green = (256.0 * pixel.y().clamp(0.0, 0.999)) as u32;
                    let blue = (256.0 * pixel.z().clamp(0.0, 0.999)) as u32;
                    let rgb = blue | (green << 8) | (red << 16);
                    // the y axis is flipped
                    let y = image_height - y - 1;
                    let index = (y * image_width + x) as usize;
                    buffer[index] = rgb;
                }
                buffer.present().expect("render not fail");
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
