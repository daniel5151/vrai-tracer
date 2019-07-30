use std::time::Duration;

use minifb::{Key, Scale, Window, WindowOptions};

pub mod camera;
pub mod ray;
mod render;
pub mod vec3;

const BASE_WIDTH: usize = 256;
const BASE_HEIGHT: usize = 128;
const DEFAULT_SAMPLES: usize = 4; // speedy, but grainy

const TITLE: &str = "Ray Tacing in 1 Weekend";

struct SmoothAvg {
    total: f32,
    i: usize,
    e: [f32; 8],
}

impl SmoothAvg {
    fn new() -> SmoothAvg {
        SmoothAvg {
            total: 0.,
            i: 0,
            e: [0.; 8],
        }
    }

    fn update(&mut self, v: f32) {
        self.total = self.total - self.e[self.i] + v;
        self.e[self.i] = v;
        self.i = (self.i + 1) % 8;
    }

    fn get(&self) -> f32 {
        self.total / 8.
    }
}

struct Opts {
    movement: bool,
    freeze: bool,
}

fn main() -> Result<(), minifb::Error> {
    let args: Vec<String> = std::env::args().collect();
    let samples = match args.get(1) {
        Some(s) => s.parse().expect("bad number of samples"),
        None => DEFAULT_SAMPLES,
    };

    let mut window = Window::new(
        TITLE,
        BASE_WIDTH,
        BASE_HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            resize: true,
            ..WindowOptions::default()
        },
    )?;

    let mut buffer: Vec<u32> = Vec::new();

    let mut init_time = std::time::Instant::now();
    let mut last_frame = init_time;
    let mut fups = SmoothAvg::new();

    let mut opts = Opts {
        movement: false,
        freeze: false,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update buffer size if window size changes
        let (width, height) = window.get_size();
        let (width, height) = (width / 2, height / 2); // because scaling
        if buffer.len() != width * height {
            buffer.resize(width * height, 0);
        }

        // "movement" in the sense that the render function gets passed the time
        // since movement began
        let time = if opts.movement {
            init_time.elapsed()
        } else {
            init_time = std::time::Instant::now();
            Duration::new(0, 0)
        };

        if !opts.freeze {
            render::trace_some_rays(
                &mut buffer,
                render::RenderOpts {
                    width,
                    height,
                    samples,
                },
                time,
            );
        } else {
            // do some busywork to avoid breaking the fups counter
            // try to hit ~144 fups
            std::thread::sleep(Duration::new(0, ((1000. / 144.) * 1000000.) as u32));
        }

        window.update_with_buffer(&buffer)?;

        // Update frame-rate counter
        fups.update(1000. / last_frame.elapsed().as_millis() as f32);
        last_frame += last_frame.elapsed();
        window.set_title(format!("{} - {:.2} fups", TITLE, fups.get()).as_str());

        // Check for various live options

        window.get_keys_pressed(minifb::KeyRepeat::No).map(|keys| {
            for key in keys {
                match key {
                    Key::Space => {
                        opts.movement = !opts.movement;
                        fups = SmoothAvg::new();
                    }
                    _ => {}
                }
            }
        });

        window.get_keys().map(|keys| {
            let mut freeze = false;
            for key in keys {
                match key {
                    Key::F => freeze = true,
                    _ => {}
                }
            }
            opts.freeze = freeze;
        });
    }

    Ok(())
}
