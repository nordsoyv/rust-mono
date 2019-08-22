use crate::vec3::Vec3;
use crate::ray::Ray;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CameraBuilder {
  pub lookfrom: Vec3,
  pub lookat: Vec3,
  pub vup: Vec3,
  pub vfov: f32,
  pub aspect: f32,
}

impl CameraBuilder {
  pub fn build(&self) -> Camera {
    Camera::new(self.lookfrom, self.lookat, self.vup, self.vfov, self.aspect)
  }
}

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  pub fn new(
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    vfov: f32,
    aspect: f32,
//    aperture: f64,
//    focus_dist: f64,
  ) -> Camera {
//    let lens_radius = aperture / 2.0;
    let theta = vfov * std::f32::consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    let origin = lookfrom;
    let dir = lookfrom - lookat;
    let w = dir.to_unit();
    let u = vup.cross(w).to_unit();
    let v = w.cross(u);
    let lower_left_corner = origin - half_width * u - half_height * v - w;
    let horizontal = u * half_width * 2.0;
    let vertical = v * half_height * 2.0;
    Camera {
      origin,
      horizontal,
      vertical,
      lower_left_corner,
//      u,
//      v,
//      lens_radius,
    }
  }

  #[allow(dead_code)]
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
    Ray::new(self.origin, self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin)
  }
}


#[test]
fn to_json() {
  let camera = CameraBuilder {
    lookfrom: Vec3::new(-2.0, 2.0, 1.0),
    lookat: Vec3::new(0.0, 0.0, -1.0),
    vup: Vec3::new(0.0, 1.0, 0.0),
    vfov: 90.0,
    aspect: 400.0 / 200.0,
  };
  let json = r#"{"lookfrom":{"x":-2.0,"y":2.0,"z":1.0},"lookat":{"x":0.0,"y":0.0,"z":-1.0},"vup":{"x":0.0,"y":1.0,"z":0.0},"vfov":90.0,"aspect":2.0}"#.to_string();
  let s = serde_json::to_string(&camera).unwrap();
  assert_eq!(s, json);
}