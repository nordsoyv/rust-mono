use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct HitResult {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
}

pub trait Hitable: Sync {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult>;
}

pub struct Sphere {
  center: Vec3,
  radius: f32,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32) -> Sphere {
    Sphere {
      center,
      radius,
    }
  }
}

impl Hitable for Sphere {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
    let oc = ray.origin() - self.center;
    let a = dot(&ray.direction(), &ray.direction());
    let b = dot(&oc, &ray.direction());
    let c = dot(&oc, &oc) - self.radius * self.radius;
    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
      let tmp = (-b - discriminant.sqrt()) / a;
      if tmp < t_max && tmp > t_min {
        let p = ray.point_at_param(tmp);
        let normal = (p - self.center) / self.radius;
        let hit = HitResult {
          t: tmp,
          p,
          normal,
        };
        return Some(hit);
      }
      let tmp = (-b + discriminant.sqrt()) / a;
      if tmp < t_max && tmp > t_min {
        let p = ray.point_at_param(tmp);
        let normal = (p - self.center) / self.radius;
        let hit = HitResult {
          t: tmp,
          p,
          normal,
        };
        return Some(hit);
      }
    }
    None
  }
}

pub struct HitableList {
  hitables: Vec<Box<dyn Hitable>>
}

impl HitableList {
  pub fn new() -> HitableList {
    HitableList {
      hitables: vec![]
    }
  }
  pub fn add(&mut self, h: Box<dyn Hitable>) {
    self.hitables.push(h);
  }
}

impl Hitable for HitableList {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
    let mut best_hit = None;
    let mut closest = t_max;
    for h in &self.hitables {
      match h.hit(ray, t_min, closest) {
        Some(hit_result) => {
          closest = hit_result.t;
          best_hit = Some(hit_result);
        }
        None => {}
      }
    }
    return best_hit;
  }
}