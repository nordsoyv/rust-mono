use crate::hitable::{HitableList, Sphere};
use crate::vec3::Vec3;
use std::sync::Arc;
use crate::material::{Lambertian, Metal, Dielectric};

#[allow(dead_code)]
pub fn build_test_world() -> HitableList {
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


pub fn build_world() -> HitableList {
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
