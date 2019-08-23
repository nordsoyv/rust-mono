use serde_derive::{Deserialize, Serialize};

use crate::vec3::{Vec3, random_in_unit_disk};
use crate::ray::Ray;

#[derive(Deserialize, Serialize, Debug)]
pub struct CameraBuilder {
  pub look_from: Vec3,
  pub look_at: Vec3,
  pub vup: Vec3,
  pub vfov: f32,
  pub aspect: f32,
  pub aperture : f32,
}

impl CameraBuilder {
  pub fn build(&self) -> Camera {
    Camera::new(self.look_from, self.look_at, self.vup, self.vfov, self.aspect, self.aperture)
  }
}

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub lens_radius : f32,
  pub u:Vec3,
  pub v:Vec3,

}

impl Camera {
  pub fn new(
    look_from: Vec3,
    look_at: Vec3,
    vup: Vec3,
    vfov: f32,
    aspect: f32,
    aperture: f32,
  ) -> Camera {

    let focus_dist = (look_from-look_at).length();
    let lens_radius = aperture / 2.0;
    let theta = vfov * std::f32::consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    let origin = look_from;
    let dir = look_from - look_at;
    let w = dir.to_unit();
    let u = vup.cross(w).to_unit();
    let v = w.cross(u);
    let lower_left_corner = origin - half_width * u * focus_dist - half_height * v * focus_dist - w * focus_dist;
    let horizontal = u * half_width * 2.0 * focus_dist;
    let vertical = v * half_height * 2.0 * focus_dist;
    Camera {
      origin,
      horizontal,
      vertical,
      lower_left_corner,
      u,
      v,
      lens_radius,
    }
  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    let rd = self.lens_radius * random_in_unit_disk();
    let offset = self.u * rd.x() + self.v* rd.y();
    Ray::new(self.origin + offset, self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.origin - offset)
  }
}
