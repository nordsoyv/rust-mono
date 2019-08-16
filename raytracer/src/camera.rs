use crate::vec3::{Vec3, unit_vec, cross};
use crate::ray::Ray;

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
//  pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, v_fov: f32, aspect: f32) -> Camera {
//    let theta = (v_fov * std::f32::consts::PI) / 180.0;
//    let half_height = (theta / 2.0).tan();
//    let half_width = aspect * half_height;
//    let origin = look_from;
//    let w  = (look_from - look_at).to_unit();
//    let u = unit_vec(cross(vup,w));
//    let v = cross(w,u);
////    let lower_left_corner =  Vec3::new(-half_width, -half_height, -1.0);
//    let lower_left_corner =  origin - half_width*u - half_height*v - w;
//
//    Camera {
//      origin,
//      lower_left_corner,
//      horizontal: u*half_width*2.0,
//      vertical: v*half_height*2.0,
//    }
//  }


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
    let w = (lookfrom - lookat).to_unit();
    let u = vup.cross(w).to_unit();
    let v = w.cross(u);
    let lower_left_corner = origin - u * half_width - v * half_height - w;
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
    Ray::new(self.origin, self.lower_left_corner + (self.horizontal * u) + (self.vertical * v))
  }
}

