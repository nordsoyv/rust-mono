use minifb::{Key, Window, WindowOptions};

use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::dot;

mod vec3;
mod ray;

const WIDTH: usize = 600;
const HEIGHT: usize = 300;


fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
//  return false;
  let oc = ray.origin() - center;
  let a =  dot(&ray.direction(),&ray.direction());
  let b = 2.0 *dot(&oc,&ray.direction());
  let c = oc.dot(oc) - radius * radius;
  let discriminant = b * b - 4.0 * a * c;
//dbg!(a,b,c,discriminant);
  return discriminant > 0.0;
}

fn lerp_vector(t: f32, start: Vec3, end: Vec3) -> Vec3 {
  return (start * (1.0 - t)) + (end * t);
}

fn get_color(ray: Ray) -> Vec3 {
  if hit_sphere(Vec3::new(0.0,0.0,-1.0),0.5, &ray) {
    return Vec3::new(1.0,0.0,0.0);
  }
  let lerp_start = Vec3::new(1.0, 1.0, 1.0);
  let lerp_end = Vec3::new(0.5, 0.7, 1.0);
  let unit_dir = ray.direction().to_unit();
  let t = 0.5 * (unit_dir.y() + 1.0);
  return lerp_vector(t, lerp_start, lerp_end);
}

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
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);


    for j in (0..HEIGHT).rev() {
      for i in 0..WIDTH {
        let u = i as f32 / WIDTH as f32;
        let v = j as f32 / HEIGHT as f32;
        let ray = Ray::new(origin, lower_left + (horizontal * u) + (vertical * v));
        let col = get_color(ray);
        buffer[buffer_pos] = col.to_u32_col();
        buffer_pos += 1;
      }
    }

    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    window.update_with_buffer(&buffer).unwrap();
  }
}
