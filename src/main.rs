use minifb::{Key, Scale, Window, WindowOptions};

pub mod camera;
pub mod ray;
mod render;
pub mod vec3;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

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

fn main() -> Result<(), minifb::Error> {
    let mut window = Window::new(
        TITLE,
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )?;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let global_time = std::time::Instant::now();

    let mut last_frame = std::time::Instant::now();
    let mut fups = SmoothAvg::new();
    std::thread::sleep(std::time::Duration::new(0, 16000000));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        fups.update(1000. / last_frame.elapsed().as_millis() as f32);
        last_frame += last_frame.elapsed();
        window.set_title(format!("{} - {:.2} fups", TITLE, fups.get()).as_str());

        render::trace_some_rays(&mut buffer, WIDTH, HEIGHT, global_time.elapsed());

        window.update_with_buffer(&buffer)?;
    }

    Ok(())
}
