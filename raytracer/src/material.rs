use crate::ray::Ray;
use crate::hitable::HitResult;
use crate::vec3::{Vec3, dot, unit_vec};
use rand::distributions::{Uniform, Distribution};

pub struct MaterialResult {
  pub attenuation: Vec3,
  pub scattered: Ray,
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
  let uv = unit_vec(v);
  let dt = dot(&uv, n);
  let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
  if discriminant > 0.0 {
    let refracted = ((uv - *n * dt) * ni_over_nt) - *n * discriminant.sqrt();
    return Some(refracted);
  } else {
    return None;
  }
}

fn random_in_unit_sphere() -> Vec3 {
  let mut rng = rand::thread_rng();
  let random = Uniform::from(0.0f32..1.0f32);

  loop {
    let p = Vec3::new(random.sample(&mut rng), random.sample(&mut rng), random.sample(&mut rng)) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    if p.squared_length() < 1.0 {
      return p;
    }
  }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
  return *v - *n * dot(v, n) * 2.0;
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
  let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  r0 = r0 * r0;
  return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}

pub trait Material: Send {
  fn scatter(&self, r: &Ray, hit_result: &HitResult) -> Option<MaterialResult>;
}

pub struct Lambertian {
  albedo: Vec3,
}

impl Lambertian {
  pub fn new(albedo: Vec3) -> Lambertian {
    Lambertian {
      albedo
    }
  }
}

impl Material for Lambertian {
  fn scatter(&self, _r: &Ray, hit_result: &HitResult) -> Option<MaterialResult> {
    let target = hit_result.p + hit_result.normal + random_in_unit_sphere();
    let scattered = Ray::new(hit_result.p, target - hit_result.p);
    let attenuation = self.albedo;
    Some(MaterialResult {
      attenuation,
      scattered,
    })
  }
}

pub struct Metal {
  albedo: Vec3,
  fuzz: f32,
}

impl Metal {
  pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
    Metal {
      albedo,
      fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
    }
  }
}

impl Material for Metal {
  fn scatter(&self, r: &Ray, hit_result: &HitResult) -> Option<MaterialResult> {
    let reflected = reflect(&r.direction().to_unit(), &hit_result.normal);
    let scattered = Ray::new(hit_result.p, reflected + random_in_unit_sphere() * self.fuzz);
    let attenuation = self.albedo;
    if dot(&scattered.direction(), &hit_result.normal) > 0.0 {
      return Some(MaterialResult {
        attenuation,
        scattered,
      });
    }
    return None;
  }
}


pub struct Dielectric {
  ref_idx: f32,
}

impl Dielectric {
  pub fn new(ref_idx: f32) -> Dielectric {
    Dielectric {
      ref_idx
    }
  }
}

impl Material for Dielectric {
  fn scatter(&self, r: &Ray, hit_result: &HitResult) -> Option<MaterialResult> {
    let outward_normal;
    let reflected = reflect(&r.direction(), &hit_result.normal);
    let ni_over_nt;
    let attenuation = Vec3::new(1.0, 1.0, 1.0);
//    let refracted;
    let scattered;
    let reflect_prop : f32;
    let cosine : f32;
    if dot(&r.direction(), &hit_result.normal) > 0.0 {
      outward_normal = hit_result.normal * -1.0;
      ni_over_nt = self.ref_idx;
      cosine = self.ref_idx* dot(&r.direction(), &hit_result.normal) /r.direction().length();
    } else {
      outward_normal = hit_result.normal;
      ni_over_nt = 1.0 / self.ref_idx;
      cosine = -dot(&r.direction(), &hit_result.normal) /r.direction().length();
    }
    if let Some(refracted) = refract(&r.direction(), &outward_normal, ni_over_nt) {
      scattered = Ray::new(hit_result.p, refracted);
    } else {
      scattered = Ray::new(hit_result.p, reflected);
    }
    Some(MaterialResult {
      attenuation,
      scattered,
    })
  }
}
