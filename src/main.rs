use std::time::Duration;

use minifb::{Key, Scale, Window, WindowOptions};

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
mod render;
pub mod scenes;
pub mod util;
pub mod vec3;

use camera::{Camera, CameraOpts};
use hittable::Hittable;
use scenes::Scene;

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

    // setup the world
    let mut scene = scenes::Random::new();
    // let mut scene = scenes::Chapter::new();

    // various live-controllable
    let mut opts = Opts {
        movement: false,
        freeze: false,
        samples,
        cam: scene.init_camopts(),
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update buffer size if window size changes
        let (width, height) = window.get_size();
        let (width, height) = (width / 2, height / 2); // because scaling
        if buffer.len() != width * height {
            buffer.resize(width * height, 0);
        }

        // update camera aperture
        opts.cam.aspect = width as f32 / height as f32;

        let time = if opts.movement {
            init_time.elapsed()
        } else {
            init_time = std::time::Instant::now();
            Duration::new(0, 0)
        };

        // offer the scene a change to update itself
        scene.animate(time);

        // TODO: a safe way to cache the dyn Hittable refs instead of always
        // rebuilding the world vector each iteration?
        // Alternatively, push this out of the loop, and figure out a safe way
        // to downcast &mut dyn Hittables?
        //
        // e.g: this is wildly unsafe lol:
        // ```
        // world.get_mut(0).map(|s| {
        //     // works because of TraitObject struct layout, and because I
        //     // _know_ that the dyn Hittable at index 0 is indeed a Sphere...
        //     let s = unsafe { &mut *(*s as *mut dyn Hittable as *mut Sphere) };
        //     s.center.x = (time.as_millis() as f32 / 1000.).sin();
        // });
        // ```
        let world = scene
            .get_world()
            .iter()
            .map(|x| x as &dyn Hittable)
            .collect::<Vec<_>>();

        // Alternatively, monomorphize render on Vec<Sphere>...
        // let world = &scene.get_world();

        // create the camera
        let camera = Camera::new(opts.cam);

        if !opts.freeze {
            render::trace_some_rays(
                &mut buffer,
                &world,
                camera,
                render::RenderOpts {
                    width,
                    height,
                    samples: opts.samples,
                },
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
        window.get_keys_pressed(minifb::KeyRepeat::Yes).map(|keys| {
            for key in keys {
                let mut opts_updated = true;
                match key {
                    Key::Space => {
                        opts.movement = !opts.movement;
                        fups = SmoothAvg::new();
                    }
                    Key::W => opts.cam.origin -= opts.cam.direction * 0.1,
                    Key::S => opts.cam.origin += opts.cam.direction * 0.1,
                    Key::Minus => opts.cam.hfov -= 1.0,
                    Key::Equal => opts.cam.hfov += 1.0,
                    Key::Period => opts.samples += 1,
                    Key::Comma => opts.samples -= 1,
                    // e => eprintln!("{:?}", e),
                    _ => opts_updated = false,
                }
                if opts_updated {
                    println!("{:#?}", opts);
                }
            }
        });
        window.get_keys().map(|keys| {
            opts.freeze = false;
            for key in keys {
                match key {
                    Key::F => opts.freeze = true,
                    _ => {}
                }
            }
        });
    }

    Ok(())
}
