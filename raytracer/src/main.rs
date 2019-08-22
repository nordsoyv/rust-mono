mod vec3;
mod ray;
mod hitable;
mod camera;
mod material;
mod scene;
mod canvas;

use minifb::{Key, Window, WindowOptions};

fn main() {
  let scene = scene::load_scene().unwrap_or_else(|e| {
    panic!("{}", e);
  });

  let mut window = Window::new(
    "Test - ESC to exit",
    scene.canvas.width,
    scene.canvas.height,
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });

  let start = std::time::Instant::now();
  let buffer = scene.render();
  let end = start.elapsed();
  let end_time = (end.as_nanos() as f64) / (1000.0 * 1000.0);
  println!("Time taken to render : {} milliseconds", end_time);

  while window.is_open() && !window.is_key_down(Key::Escape) {
    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    window.update_with_buffer(&buffer).unwrap();
  }
}
