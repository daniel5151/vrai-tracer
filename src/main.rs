#![allow(clippy::many_single_char_names)] // lots of math uses single char names

use std::sync::{Arc, RwLock};
use std::time::Duration;

use minifb::{Key, Window, WindowOptions};

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod render;
pub mod scenes;
pub mod util;
pub mod vec3;

use camera::{Camera, CameraOpts};
use scenes::Scene;

const BASE_WIDTH: usize = 256;
const BASE_HEIGHT: usize = 128;
const DEFAULT_SAMPLES: usize = 4; // speedy, but grainy

const TITLE: &str = "Ray Tacing in 1 Weekend";

#[derive(Debug)]
struct Opts {
    movement: bool,
    freeze: bool,
    samples: usize,
    cam: CameraOpts,
}

fn main() -> Result<(), minifb::Error> {
    let args: Vec<String> = std::env::args().collect();
    let samples = match args.get(1) {
        Some(s) => s.parse().expect("bad number of samples"),
        None => DEFAULT_SAMPLES,
    };

    let (base_width, base_height) = match args.get(2) {
        Some(s) => {
            let res = s
                .split('x')
                .map(|s| s.parse::<usize>().expect("bad resolution"))
                .collect::<Vec<_>>();
            (res[0], res[1])
        }
        None => (BASE_WIDTH, BASE_HEIGHT),
    };

    let mut window = Window::new(
        TITLE,
        base_width,
        base_height,
        WindowOptions {
            // scale: minifb::Scale::X2,
            resize: true,
            ..WindowOptions::default()
        },
    )?;

    let mut buffer: Vec<u32> = vec![0; base_width * base_height];

    let mut init_time = std::time::Instant::now();
    let mut last_frame = init_time;
    let mut fups = util::SmoothAvg::new();

    // setup the world
    let scene = scenes::Random::new();
    // let scene = scenes::Chapter::new();

    // various live-controllable options
    let mut opts = Opts {
        movement: false,
        freeze: false,
        samples,
        cam: scene.get_camera().opts(),
    };

    // The main loop.
    //
    // Through the power of t h r e a d i n g, the render thread is not blocked
    // on the ray tracer thread. Instead, the ray tracer returns a
    // RenderProgress struct which can be used to continuously flush progress
    // to the framebuffer.

    let scene = Arc::new(RwLock::new(scene));
    let mut current_frame = render::RenderProgress::default();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update buffer size if window size changes
        let (width, height) = window.get_size();
        // let (width, height) = (width / 2, height / 2); // because scaling
        if buffer.len() != width * height {
            buffer = vec![0; width * height];
            current_frame.invalidate();
        }

        if current_frame.poll_done() && !opts.freeze {
            // kick off another frame!

            // Update frame-rate counter
            fups.update(1000. / last_frame.elapsed().as_millis() as f32);
            last_frame += last_frame.elapsed();
            window.set_title(format!("{} - {:.2} fups", TITLE, fups.get()).as_str());

            // update camera aperture
            opts.cam.aspect = width as f32 / height as f32;

            // perform any scene updates
            {
                let time = if opts.movement {
                    init_time.elapsed()
                } else {
                    init_time = std::time::Instant::now();
                    Duration::new(0, 0)
                };

                let mut scene = scene.write().unwrap();
                scene.enable_freecam(Camera::new(opts.cam));
                scene.animate(time);
            }

            current_frame = render::trace_some_rays_nonblocking(
                &scene,
                render::RenderOpts {
                    width,
                    height,
                    samples: opts.samples,
                },
            );
        }

        // Update the window's framebuffer
        current_frame.flush_to_buffer(&mut buffer);
        window.update_with_buffer(&buffer)?;

        // Check for various live options
        if let Some(keys) = window.get_keys_pressed(minifb::KeyRepeat::Yes) {
            for key in keys {
                let mut opts_updated = true;
                match key {
                    Key::Space => opts.movement = !opts.movement,
                    Key::W => opts.cam.origin -= opts.cam.direction * 0.1,
                    Key::S => opts.cam.origin += opts.cam.direction * 0.1,
                    Key::Minus => opts.cam.hfov -= 1.0,
                    Key::Equal => opts.cam.hfov += 1.0,
                    Key::Period => opts.samples += 1,
                    Key::Comma => {
                        if opts.samples > 1 {
                            opts.samples -= 1
                        }
                    }
                    // e => eprintln!("{:?}", e),
                    _ => opts_updated = false,
                }
                if opts_updated {
                    println!("{:#?}", opts);
                    buffer = vec![0; width * height];
                    current_frame.invalidate();
                }
            }
        };

        if let Some(keys) = window.get_keys() {
            opts.freeze = false;
            for key in keys {
                #[allow(clippy::single_match)]
                match key {
                    Key::F => {
                        opts.freeze = true;
                        current_frame.invalidate();
                    }
                    _ => {}
                }
            }
        };
    }

    Ok(())
}
