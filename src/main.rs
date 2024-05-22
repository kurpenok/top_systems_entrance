mod drawer;
mod error;

use drawer::GeometryDrawer;
use minifb::{Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const WHITE: usize = 0xffffff;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let geometry = GeometryDrawer::new(WIDTH);

    let mut window = Window::new(
        "TOP Systems entrance",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let _ = geometry.draw_rectangle(&mut buffer, 120, 130, 220, 230, 1, WHITE);

    let _ = geometry.draw_circle(&mut buffer, 320, 180, 50, WHITE);

    let _ = geometry.draw_line(&mut buffer, 420, 220, 470, 120, WHITE);
    let _ = geometry.draw_line(&mut buffer, 520, 220, 470, 120, WHITE);
    let _ = geometry.draw_line(&mut buffer, 420, 220, 520, 220, WHITE);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
