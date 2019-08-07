use minifb::{Key, Window, WindowOptions};

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
    let mut col = 0;


//    let mut buffer_iter = buffer.iter_mut();
    let mut buffer_pos = 0;
    for j in 0..HEIGHT {
      for i in 0..WIDTH {
        let r = i as f32 / WIDTH as f32;
        let g = j as f32 / HEIGHT as f32;
        let b = 0.2f32;

        let red: u32 = ((r * 255.0) as u8) as u32;
        let green: u32 = ((g * 255.0) as u8) as u32;
        let blue: u32 = ((b * 255.0) as u8) as u32;
        let alpha: u32 = 255 as u32;


        let mut c: u32 = 0;
        c = c | alpha << 24;
        c = c | red << 16;
        c = c | green << 8;
        c = c | blue << 0;

        buffer[buffer_pos] = c;
        buffer_pos += 1;
      }
    }

    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    window.update_with_buffer(&buffer).unwrap();
  }
}
