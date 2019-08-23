use std::sync::Arc;

use crate::ray::Ray;
use crate::vec3::{dot, Vec3};
use crate::material::Material;

pub struct HitResult {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
  pub material: usize,
}

pub trait Hitable: Sync {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult>;
}

pub struct Sphere {
  center: Vec3,
  radius: f32,
  material:usize,
}

unsafe impl Sync for Sphere {}

impl Sphere {
  pub fn new(center: Vec3, radius: f32, material: usize) -> Sphere {
    Sphere {
      center,
      radius,
      material,
    }
  }

  fn build_hit_result(&self, t: f32, ray: &Ray) -> HitResult {
    let p = ray.point_at_param(t);
    let normal = (p - self.center) / self.radius;
    let hit = HitResult {
      t,
      p,
      normal,
      material: self.material,
    };
    return hit;
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
        return Some(self.build_hit_result(tmp, ray));
      }
      let tmp = (-b + discriminant.sqrt()) / a;
      if tmp < t_max && tmp > t_min {
        return Some(self.build_hit_result(tmp, ray));
      }
    }
    None
  }
}

pub struct HitableList {
  hitables: Vec<Box<dyn Hitable>>,
  materials: Vec<Arc<Box<dyn Material>>>,
}

unsafe impl Sync for HitableList {}

impl HitableList {
  pub fn new() -> HitableList {
    HitableList {
      hitables: vec![],
      materials: vec![],
    }
  }
  pub fn add_hitable(&mut self, h: Box<dyn Hitable>) {
    self.hitables.push(h);
  }
  pub fn add_material(&mut self, h: Box<dyn Material>) -> usize {
    let id = self.materials.len();
    self.materials.push(Arc::new(h));
    return id;
  }

  pub fn get_material(&self, id: usize) -> Arc<Box<dyn Material>>{
    self.materials[id].clone()
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