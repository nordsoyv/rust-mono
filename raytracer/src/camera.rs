use serde_derive::{Deserialize, Serialize};
use rand::distributions::{Uniform, Distribution};
use crate::vec3::Vec3;
use crate::ray::Ray;

fn random_in_unit_disk() -> Vec3 {
  let mut rng = rand::thread_rng();
  let random = Uniform::from(0.0f32..1.0f32);

  loop {
    let p = Vec3::new(random.sample(&mut rng), random.sample(&mut rng), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
    if p.dot(p) < 1.0 {
      return p;
    }
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CameraBuilder {
  pub lookfrom: Vec3,
  pub lookat: Vec3,
  pub vup: Vec3,
  pub vfov: f32,
  pub aspect: f32,
  pub aperture : f32,
}

impl CameraBuilder {
  pub fn build(&self) -> Camera {
    Camera::new(self.lookfrom, self.lookat, self.vup, self.vfov, self.aspect, self.aperture)
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
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    vfov: f32,
    aspect: f32,
    aperture: f32,
  ) -> Camera {

    let focus_dist = (lookfrom-lookat).length();
    let lens_radius = aperture / 2.0;
    let theta = vfov * std::f32::consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    let origin = lookfrom;
    let dir = lookfrom - lookat;
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

//  #[allow(dead_code)]
//  pub fn default() -> Camera {
//    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
//    let horizontal = Vec3::new(4.0, 0.0, 0.0);
//    let vertical = Vec3::new(0.0, 2.0, 0.0);
//    let origin = Vec3::new(0.0, 0.0, 0.0);
//    Camera {
//      origin,
//      lower_left_corner,
//      horizontal,
//      vertical,
//    }
//  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    let rd = self.lens_radius * random_in_unit_disk();
    let offset = self.u * rd.x() + self.v* rd.y();
    Ray::new(self.origin + offset, self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.origin - offset)
  }
}
