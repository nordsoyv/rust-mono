use crate::vec3::Vec3;

pub struct Ray {
  origin: Vec3,
  direction: Vec3,
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3) -> Ray {
    Ray {
      origin,
      direction,
    }
  }

  pub fn origin(&self) -> Vec3 {
    self.origin
  }

  pub fn direction(&self) -> Vec3 {
    self.direction
  }

  pub fn point_at_param(&self, t: f32) -> Vec3 {
    self.origin + (self.direction * t)
  }
}

#[test]
fn basic_test() {
  let ray = Ray::new(Vec3::new(0.0,0.0,0.0),Vec3::new(1.0,0.0,0.0) );
  let point = ray.point_at_param(1.0);
  assert_eq!(point.x(), 1.0);
  assert_eq!(point.y(), 0.0);
  assert_eq!(point.z(), 0.0);
}