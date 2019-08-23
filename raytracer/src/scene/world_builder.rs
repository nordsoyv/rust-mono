use rand::distributions::{Uniform, Distribution};
use crate::hitable::{HitableList, Sphere};
use crate::vec3::Vec3;
use crate::material::{Lambertian, Metal, Dielectric};

#[allow(dead_code)]
pub fn build_test_world() -> HitableList {
  let r = (std::f32::consts::PI / 4.0).cos();
  let mut world = HitableList::new();
  let mat1 = world.add_material(Box::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0))));
  world.add_hitable(
    Box::new(Sphere::new(Vec3::new(-r, 0.0, -2.0),
                         r,
                         mat1)));
  let mat2 = world.add_material(Box::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0))));
  world.add_hitable(
    Box::new(Sphere::new(Vec3::new(r, 0.0, -1.0),
                         r,
                         mat2)));

  return world;
}


#[allow(dead_code)]
pub fn build_world() -> HitableList {
  let mut world = HitableList::new();
  let mat_id = world.add_material(Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))));
  world.add_hitable(
    Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0),
                         0.5,
                         mat_id)));
  let mat_id = world.add_material(Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))));
  world.add_hitable(
    Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0),
                         100.0,
                         mat_id)));
  let mat_id = world.add_material(Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)));
  world.add_hitable(
    Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0),
                         0.5,
                         mat_id)));

  let mat_id = world.add_material(Box::new(Dielectric::new(1.5)));
  world.add_hitable(
    Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0),
                         0.5,
                         mat_id)));

  world.add_hitable(
    Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0),
                         -0.45,
                         mat_id)));

  return world;
}

#[allow(dead_code)]
pub fn build_random_world() -> HitableList {
  let mut world = HitableList::new();
  let mut rng = rand::thread_rng();
  let random = Uniform::from(0.0f32..1.0f32);

  // ground
  let ground_mat = world.add_material(Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))));
  world.add_hitable(
    Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0),
                         1000.0,
                         ground_mat)));
  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = random.sample(&mut rng);
      let center = Vec3::new((a as f32) + 0.9 * random.sample(&mut rng), 0.2, (b as f32) + 0.9 * random.sample(&mut rng));
      if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          let m = world.add_material(Box::new(Lambertian::new(Vec3::new(
            random.sample(&mut rng) * random.sample(&mut rng),
            random.sample(&mut rng) * random.sample(&mut rng),
            random.sample(&mut rng) * random.sample(&mut rng)))));
          let s = Sphere::new(center, 0.2, m);
          world.add_hitable(Box::new(s));
        } else if choose_mat < 0.95 {
          let m = world.add_material(Box::new(Metal::new(Vec3::new(
            0.5 * (1.0 + random.sample(&mut rng)),
            0.5 * (1.0 + random.sample(&mut rng)),
            0.5 * (1.0 + random.sample(&mut rng))),
                                                         0.5 * random.sample(&mut rng))));
          let s = Sphere::new(center, 0.2, m);
          world.add_hitable(Box::new(s));
        } else { // glass
          let m = world.add_material(Box::new(Dielectric::new(1.5)));
          let s = Sphere::new(center, 0.2, m);
          world.add_hitable(Box::new(s));
        }
      }
    }
  }
  let d_mat = world.add_material(Box::new(Dielectric::new(1.5)));
  let d = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, d_mat);
  let l_mat = world.add_material(Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))));
  let l = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, l_mat);
  let m_mat = world.add_material(Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)));
  let m = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, m_mat);

  world.add_hitable(Box::new(d));
  world.add_hitable(Box::new(l));
  world.add_hitable(Box::new(m));

  world
}