use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  #[allow(dead_code)]
  pub fn new(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
    Camera {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    }
  }

  pub fn default() -> Camera {
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    Camera {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    }
  }

  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    Ray::new(self.origin, self.lower_left_corner + (self.horizontal * u) + (self.vertical * v))
  }
}

