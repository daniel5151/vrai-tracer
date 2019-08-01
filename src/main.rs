use std::time::Duration;

use minifb::{Key, Scale, Window, WindowOptions};

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
mod render;
pub mod util;
pub mod vec3;

use camera::Camera;
use hittable::{Hittable, Sphere};
use vec3::Vec3;

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
    fov: f32,
    samples: usize,
    camera_origin: Vec3,
    camera_direction: Vec3,
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
        fov: 45.0,
        samples,
        camera_direction: (Vec3::new(-2.0, 2., 1.) - Vec3::new(0., 0., -1.)).normalize(),
        camera_origin: Vec3::new(-2.0, 2., 1.),
    };

    // setup the world
    let spheres = vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(material::Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.25)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(material::Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Box::new(material::Dielectric::new(1.5)),
        )),
    ];

    let world: Vec<&dyn Hittable> = spheres
        .iter()
        .map(|x| x.as_ref() as &dyn Hittable)
        .collect();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update buffer size if window size changes
        let (width, height) = window.get_size();
        let (width, height) = (width / 2, height / 2); // because scaling
        if buffer.len() != width * height {
            buffer.resize(width * height, 0);
        }

        // TODO: actually animate something with time?
        let _time = if opts.movement {
            init_time.elapsed()
        } else {
            init_time = std::time::Instant::now();
            Duration::new(0, 0)
        };

        // create the camera
        let camera = Camera::new(
            opts.camera_origin,
            opts.camera_direction,
            Vec3::new(0., 1., 0.),
            opts.fov,
            width as f32 / height as f32,
        );

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
                    Key::W => opts.camera_origin -= opts.camera_direction * 0.1,
                    Key::S => opts.camera_origin += opts.camera_direction * 0.1,
                    Key::Minus => opts.fov -= 1.0,
                    Key::Equal => opts.fov += 1.0,
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
