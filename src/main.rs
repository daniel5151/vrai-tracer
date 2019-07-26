use minifb::{Key, Scale, Window, WindowOptions};

pub mod ray;
pub mod vec3;

use ray::Ray;
use vec3::Vec3;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Into<u32> for Pixel {
    fn into(self) -> u32 {
        u32::from_le_bytes([self.b, self.g, self.r, self.a])
    }
}

impl Into<Pixel> for Vec3 {
    fn into(mut self) -> Pixel {
        self = self * 255.99;
        Pixel {
            r: self.x as u8,
            g: self.y as u8,
            b: self.z as u8,
            a: 0,
        }
    }
}

fn color(r: &Ray) -> Vec3 {
    let unit_direction = Vec3::new_unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn trace_some_rays(buffer: &mut Vec<u32>) {
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for (y, row) in buffer.chunks_exact_mut(WIDTH).enumerate() {
        for (x, px) in row.iter_mut().enumerate() {
            let y = (HEIGHT - y) as f32;
            let x = x as f32;

            let u = x / WIDTH as f32;
            let v = y / HEIGHT as f32;

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col: Pixel = color(&r).into();

            *px = col.into();
        }
    }
}

fn main() {
    let mut window = match Window::new(
        "Ray Tacing in 1 Weekend (ESC to exit)",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(e) => {
            eprintln!("Unable to create window: {}", e);
            return;
        }
    };

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    trace_some_rays(&mut buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::thread::sleep(std::time::Duration::from_millis(8)); // ~120 fps

        if let Err(e) = window.update_with_buffer(&buffer) {
            eprintln!("Unable to update window: {}", e);
            return;
        }
    }
}
