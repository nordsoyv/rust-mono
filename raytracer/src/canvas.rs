use serde_derive::{Deserialize, Serialize};
use rayon::prelude::*;
use rand::distributions::{Uniform, Distribution};

use crate::camera::Camera;
use crate::hitable::{HitableList, Hitable};
use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct Canvas {
  pub width: usize,
  pub height: usize,
  pub samples: usize,
}

impl Canvas {
  pub fn render(&self, camera: &Camera, world: &HitableList) -> Vec<u32> {
    let random = Uniform::from(0.0f32..1.0f32);
    let f32_width = self.width as f32;
    let f32_height = self.height as f32;
    let f32_samples = self.samples as f32;

    let buffer = (0..self.height)
      .into_par_iter()
      .rev()
      .map(|h| {
        println!("drawing line: {}",h);
        (0..self.width)
          .into_par_iter()
          .map(|w| {
            let mut rng = rand::thread_rng();
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..self.samples {
              let u = (w as f32 + random.sample(&mut rng)) / f32_width;
              let v = (h as f32 + random.sample(&mut rng)) / f32_height;
              let ray = camera.get_ray(u, v);
              let col = get_color(ray, world, 0);
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
    return buffer;
  }
}

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

