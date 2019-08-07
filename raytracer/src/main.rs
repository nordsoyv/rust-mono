use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

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
    let mut col = 0;
    for i in buffer.iter_mut() {
      *i = col; // write something more funny here!
      col +=1;
    }

    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    window.update_with_buffer(&buffer).unwrap();
  }
}
