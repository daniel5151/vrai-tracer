use minifb::{Key, Scale, Window, WindowOptions};

pub mod vec3;

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

    for (y, row) in buffer.chunks_exact_mut(WIDTH).enumerate() {
        for (x, px) in row.iter_mut().enumerate() {
            let r = x as f32 / WIDTH as f32;
            let g = (HEIGHT - y) as f32 / HEIGHT as f32;
            let b = 0.2;

            *px = Pixel {
                r: (r * 255.99) as u8,
                g: (g * 255.99) as u8,
                b: (b * 255.99) as u8,
                a: 0,
            }
            .into();
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::thread::sleep(std::time::Duration::from_millis(8)); // ~120 fps

        if let Err(e) = window.update_with_buffer(&buffer) {
            eprintln!("Unable to update window: {}", e);
            return;
        }
    }
}
