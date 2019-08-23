mod vec3;
mod ray;
mod hitable;
mod camera;
mod material;
mod scene;
mod canvas;

use minifb::{Key, Window, WindowOptions};
#[cfg(debug_assertions)]
const PATH :&str = "scene_debug.json";
#[cfg(not(debug_assertions))]
const PATH : &str = "scene.json";

fn main() {
  let scene = scene::load_scene(std::path::Path::new(PATH)).unwrap_or_else(|e| {
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
    if window.is_key_down(Key::S){
      println!("S is pressed");
    }
    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    window.update_with_buffer(&buffer).unwrap();

    let ten_millis = std::time::Duration::from_millis(500);
    std::thread::sleep(ten_millis);

  }
}
