use minifb::{Key, Scale, Window, WindowOptions};

pub mod ray;
mod render;
pub mod vec3;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn main() -> Result<(), minifb::Error> {
    let mut window = Window::new(
        "Ray Tacing in 1 Weekend (ESC to exit)",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )?;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    render::trace_some_rays(&mut buffer, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::thread::sleep(std::time::Duration::from_millis(8)); // ~120 fps
        window.update_with_buffer(&buffer)?;
    }

    Ok(())
}
