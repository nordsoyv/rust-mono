use minifb::{Key, Window, WindowOptions};

use crate::vec3::Vec3;

mod vec3;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn main() {
  let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
  let mut window = Window::new(
    "Test - ESC to exit",
    WIDTH,
    HEIGHT,
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });

  while window.is_open() && !window.is_key_down(Key::Escape) {
    let mut buffer_pos = 0;
    for j in 0..HEIGHT {
      for i in 0..WIDTH {
        let col = Vec3::new(i as f32 / WIDTH as f32,
                            j as f32 / HEIGHT as f32,
                            0.2f32);

        buffer[buffer_pos] = col.to_u32_col();
        buffer_pos += 1;
      }
    }

    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    window.update_with_buffer(&buffer).unwrap();
  }
}
