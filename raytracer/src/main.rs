mod vec3;
mod ray;
mod hitable;
mod camera;
mod material;

use minifb::{Key, Window, WindowOptions};
use rand::distributions::{Uniform, Distribution};
use rayon::prelude::*;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitable::{HitableList, Sphere, Hitable};
use crate::camera::Camera;
use crate::material::{Lambertian, Metal, Dielectric};
use std::sync::Arc;


#[cfg(debug_assertions)]
const WIDTH: usize = 200;
#[cfg(not(debug_assertions))]
const WIDTH: usize = 400;

#[cfg(debug_assertions)]
const HEIGHT: usize = 100;
#[cfg(not(debug_assertions))]
const HEIGHT: usize = 200;

#[cfg(debug_assertions)]
const SAMPLES: usize = 100;
#[cfg(not(debug_assertions))]
const SAMPLES: usize = 200;

fn lerp_vector(t: f32, start: Vec3, end: Vec3) -> Vec3 {
  return (start * (1.0 - t)) + (end * t);
}

fn get_color(ray: Ray, world: &dyn Hitable, depth: u32) -> Vec3 {
  if depth > 50 {
    return Vec3::new(0.0, 0.0, 0.0);
  }
  if let Some(rec) = world.hit(&ray, 0.001, std::f32::INFINITY) {
    if let Some(mat_res) = rec.material.scatter(&ray, &rec) {
      return mat_res.attenuation * get_color(mat_res.scattered, world, depth + 1);
    }
  }

  let lerp_start = Vec3::new(1.0, 1.0, 1.0);
  let lerp_end = Vec3::new(0.5, 0.7, 1.0);
  let unit_dir = ray.direction().to_unit();
  let t = 0.5 * (unit_dir.y() + 1.0);
  return lerp_vector(t, lerp_start, lerp_end);
}

fn build_world() -> HitableList {
  let mut world = HitableList::new();
  world.add(
    Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0),
                         0.5,
                         Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))))));
  world.add(
    Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0),
                         100.0,
                         Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))));

  world.add(
    Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0),
                         0.5,
                         Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)))));
  world.add(
    Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0),
                         0.5,
                         Arc::new(Dielectric::new(1.5)))));

  world.add(
    Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0),
                         -0.45,
                         Arc::new(Dielectric::new(1.5)))));

  return world;
}


fn build_test_world() -> HitableList {
  let r = (std::f32::consts::PI / 4.0).cos();
  let mut world = HitableList::new();
  world.add(
    Box::new(Sphere::new(Vec3::new(-r, 0.0, -2.0),
                         r,
                         Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0))))));
  world.add(
    Box::new(Sphere::new(Vec3::new(r, 0.0, -1.0),
                         r,
                         Arc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0))))));

  return world;
}

fn render(width: usize, height: usize, samples: usize) -> Vec<u32> {
  let random = Uniform::from(0.0f32..1.0f32);
  let f32_samples = samples as f32;
  let f32_width = width as f32;
  let f32_height = height as f32;
  let camera = Camera::new(
    Vec3::new(-2.0, 2.0, 1.0),
    Vec3::new(0.0, 0.0, -1.0),
    Vec3::new(0.0, 1.0, 0.0),
    30.0,
    f32_width / f32_height);
  let world = build_world();
  let start = std::time::Instant::now();

  let buffer = (0..height)
    .into_par_iter()
    .rev()
    .map(|h| {
      (0..width)
        .into_par_iter()
        .map(|w| {
          let mut rng = rand::thread_rng();
          let mut color = Vec3::new(0.0, 0.0, 0.0);
          for _ in 0..samples {
            let u = (w as f32 ) / f32_width;
            let v = (h as f32 ) / f32_height;
            let ray = camera.get_ray(u, v);
//            dbg!(&ray);
            let col = get_color(ray, &world, 0);
            color = color + col;
          }
          color = color / f32_samples;
          // simple gamma correct
          color = Vec3::new(color.x().sqrt(), color.y().sqrt(), color.z().sqrt());
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
  let buffer = render(WIDTH, HEIGHT, SAMPLES);
  while window.is_open() && !window.is_key_down(Key::Escape) {
    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    window.update_with_buffer(&buffer).unwrap();
  }
}
