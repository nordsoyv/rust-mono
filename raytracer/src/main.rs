use std::sync::{Arc, Mutex};

use hotwatch::{Event, Hotwatch};
use minifb::{Key, Window, WindowOptions};

use crate::scene::Scene;

mod vec3;
mod ray;
mod hitable;
mod camera;
mod material;
mod scene;
mod canvas;

#[cfg(debug_assertions)]
const PATH: &str = "scene_debug.json";
#[cfg(not(debug_assertions))]
const PATH: &str = "scene.json";

type Buffer = Arc<Mutex<Vec<u32>>>;

pub fn new_buffer() -> Buffer {
  Arc::new(Mutex::new(Vec::new()))
}

fn render(scene: Scene, shared_buffer: Buffer) {
fn save_image(shared_buffer : Arc<Mutex<Vec<u32>>>, width: u32, height: u32){
  let buffer = shared_buffer.lock().unwrap();
  let mut img_buf: image::RgbImage = image::ImageBuffer::new(width, height);

  let mut buffer_index = 0;
  for (_x, _y, pixel) in img_buf.enumerate_pixels_mut() {
    let color = buffer[buffer_index];
    buffer_index += 1;
    let red = ((color & 0x00ff0000) >> 16) as u8;
    let green = ((color & 0x0000ff00) >> 8) as u8;
    let blue = (color & 0x000000ff) as u8;

    *pixel = image::Rgb([ red, green, blue]);
  }
  img_buf.save("image.png").unwrap();
}

fn render(scene: Scene, shared_buffer: Arc<Mutex<Vec<u32>>>) {
  let start = std::time::Instant::now();
  let mut buffer = scene.render();
  let mut inner_buffer = shared_buffer.lock().unwrap();
  inner_buffer.clear();
  inner_buffer.append(&mut buffer);
  let end = start.elapsed();
  let end_time = (end.as_nanos() as f64) / (1000.0 * 1000.0);
  println!("Time taken to render : {} milliseconds", end_time);
}

fn main() {
  let scene: Scene = scene::load_scene(std::path::Path::new(PATH)).unwrap_or_else(|e| {
    panic!("{}", e);
  });

  let width = scene.canvas.width as u32;
  let height = scene.canvas.height as u32;

  let mut window = Window::new(
    "Test - ESC to exit",
    scene.canvas.width,
    scene.canvas.height,
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });

  let shared_buffer = new_buffer();
  let shared_buffer_clone = shared_buffer.clone(); // copy for the file wathcer closure
  render(scene, shared_buffer.clone());

  let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
  hotwatch.watch(PATH, move |event: Event| {
    if let Event::Create(_path) = event {
      println!("scene has changed.");
      let scene = scene::load_scene(std::path::Path::new(PATH)).unwrap_or_else(|e| {
        panic!("{}", e);
      });
      render(scene, shared_buffer_clone.clone());
    }
  }).expect("failed to watch file!");

  while window.is_open() && !window.is_key_down(Key::Escape) {
    if window.is_key_down(Key::S) {
      println!("Saving image");
      save_image(shared_buffer.clone(), width, height);
      println!("image is saved");
    }
    {
      let buffer = shared_buffer.lock().unwrap();

      // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
      window.update_with_buffer(&buffer).unwrap();
    } // buffer lock ends here

    let ten_millis = std::time::Duration::from_millis(500);
    std::thread::sleep(ten_millis);
  }
}
