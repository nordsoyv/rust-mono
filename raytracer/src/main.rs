mod vec3;
mod ray;
mod hitable;
mod camera;


use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use rand::distributions::{Uniform, Distribution};
use rayon::prelude;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitable::{HitableList, Sphere, Hitable};
use crate::camera::Camera;
//use rayon::prelude::IntoParallelIterator;

const WIDTH: usize = 600;
const HEIGHT: usize = 300;
const SAMPLES: usize = 200;

fn lerp_vector(t: f32, start: Vec3, end: Vec3) -> Vec3 {
  return (start * (1.0 - t)) + (end * t);
}

fn get_color(ray: Ray, world: &dyn Hitable) -> Vec3 {
  let hit = world.hit(&ray, 0.0, 100000.0);

  match hit {
    Some(hit_result) => {
      return (hit_result.normal + 1.0) * 0.5;
    }
    None => {}
  }
  let lerp_start = Vec3::new(1.0, 1.0, 1.0);
  let lerp_end = Vec3::new(0.5, 0.7, 1.0);
  let unit_dir = ray.direction().to_unit();
  let t = 0.5 * (unit_dir.y() + 1.0);
  return lerp_vector(t, lerp_start, lerp_end);
}

fn render(width: usize, height: usize) -> Vec<u32> {
  let mut buffer: Vec<u32> = vec![0; width * height];
  let mut buffer_pos = 0;
  let mut rng = rand::thread_rng();
  let random = Uniform::from(-0.5f32..0.5f32);

  let camera = Camera::default();
  let mut world = HitableList::new();
  world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
  world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

  let start = std::time::Instant::now();

  let buffer = (0..height)
    .into_iter()
    .rev()
    .map(|h| {
      (0..width)
        .into_iter()
        .map(|w| {
          let mut color = Vec3::new(0.0, 0.0, 0.0);
          for _ in 0..SAMPLES {
            let u = (w as f32 + random.sample(&mut rng)) / width as f32;
            let v = (h as f32 + random.sample(&mut rng)) / height as f32;
            let ray = camera.get_ray(u, v);

            let col = get_color(ray, &world);
            color = color + col;
          }
          color = color / (SAMPLES as f32);
          color.to_u32_col()
        })
        .collect::<Vec<u32>>()
    })
    .flatten()
    .collect::<Vec<u32>>();

  let end = start.elapsed();
  let end_time = (end.as_nanos() as f64) / (1000.0 * 1000.0);
  println!("Time taken to render : {} milliseconds", end_time);
  return buffer;
}

fn main() {
  let mut window = Window::new(
    "Test - ESC to exit",
    WIDTH,
    HEIGHT,
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });
  let buffer = render(WIDTH, HEIGHT);
  while window.is_open() && !window.is_key_down(Key::Escape) {
    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    window.update_with_buffer(&buffer).unwrap();
  }
}
