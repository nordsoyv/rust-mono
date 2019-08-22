use rand::distributions::{Uniform, Distribution};
use std::sync::Arc;
use crate::hitable::{HitableList, Sphere, Hitable};
use crate::vec3::Vec3;
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


pub fn build_random_world() -> HitableList {
  let mut world = HitableList::new();
  let mut rng = rand::thread_rng();
  let random = Uniform::from(0.0f32..1.0f32);

  // ground
  world.add(
    Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0),
                         1000.0,
                         Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))))));
  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = random.sample(&mut rng);
      let center = Vec3::new((a as f32) + 0.9 * random.sample(&mut rng), 0.2, (b as f32) + 0.9 * random.sample(&mut rng));
      if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          let s = Sphere::new(center,
                              0.2,
                              Arc::new(
                                Lambertian::new(Vec3::new(
                                  random.sample(&mut rng) * random.sample(&mut rng),
                                  random.sample(&mut rng) * random.sample(&mut rng),
                                  random.sample(&mut rng) * random.sample(&mut rng)))));
          world.add(Box::new(s));
        } else if choose_mat < 0.95 {
          let s = Sphere::new(center,
                              0.2,
                              Arc::new(
                                Metal::new(Vec3::new(
                                  0.5 * (1.0 + random.sample(&mut rng)),
                                  0.5 * (1.0 + random.sample(&mut rng)),
                                  0.5 * (1.0 + random.sample(&mut rng))),
                                           0.5 * random.sample(&mut rng))));
          world.add(Box::new(s));
        } else { // glass
          let s = Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5)));
          world.add(Box::new(s));
        }
      }
    }
  }
  let d= Sphere::new(Vec3::new(0.0,1.0,0.0),1.0, Arc::new(Dielectric::new(1.5)));
  let l = Sphere::new(Vec3::new(-4.0,1.0,0.0),1.0, Arc::new(Lambertian::new(Vec3::new(0.4,0.2,0.1 ))));
  let m = Sphere::new(Vec3::new(4.0,1.0,0.0),1.0, Arc::new(Metal::new(Vec3::new(0.7,0.6,0.5), 0.0)));

  world.add(Box::new(d));
  world.add(Box::new(l));
  world.add(Box::new(m));

  world
}