use std::ops;
use serde_derive::{Deserialize, Serialize};

pub fn random_in_unit_disk() -> Vec3 {
  let mut rng = rand::thread_rng();
  let random = Uniform::from(0.0f32..1.0f32);

  loop {
    let p = Vec3::new(random.sample(&mut rng), random.sample(&mut rng), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
    if p.dot(p) < 1.0 {
      return p;
    }
  }
}

pub fn random_in_unit_sphere() -> Vec3 {
  let mut rng = rand::thread_rng();
  let random = Uniform::from(0.0f32..1.0f32);

  loop {
    let p = Vec3::new(random.sample(&mut rng), random.sample(&mut rng), random.sample(&mut rng)) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    if p.squared_length() < 1.0 {
      return p;
    }
  }
}

#[allow(dead_code)]
pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
  (a.x() * b.x()) + (a.y() * b.y()) + (a.z() * b.z())
}

#[allow(dead_code)]
pub fn unit_vec(v: Vec3) -> Vec3 {
  v.to_unit()
}

#[inline]
#[allow(dead_code)]
pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
  Vec3 {
    x: lhs.y() * rhs.z() - lhs.z() * rhs.y(),
    y: lhs.z() * rhs.x() - lhs.x() * rhs.z(),
    z: lhs.x() * rhs.y() - lhs.y() * rhs.x(),
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Vec3 {
  x: f32,
  y: f32,
  z: f32,
}

impl Vec3 {
  pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 {
      x,
      y,
      z,
    }
  }

  #[inline]
  pub fn x(&self) -> f32 {
    self.x
  }

  #[inline]
  pub fn y(&self) -> f32 {
    self.y
  }

  #[inline]
  pub fn z(&self) -> f32 {
    self.z
  }

  #[inline]
  #[allow(dead_code)]
  pub fn r(&self) -> f32 {
    self.x
  }

  #[inline]
  #[allow(dead_code)]
  pub fn g(&self) -> f32 {
    self.y
  }

  #[inline]
  #[allow(dead_code)]
  pub fn b(&self) -> f32 {
    self.z
  }

  #[inline]
  pub fn length(&self) -> f32 {
    self.squared_length().sqrt()
  }

  #[inline]
  pub fn squared_length(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  #[inline]
  #[allow(dead_code)]
  pub fn make_unit(&mut self) -> &mut Vec3 {
    let l = self.length();
    self.x = self.x / l;
    self.y = self.y / l;
    self.z = self.z / l;
    self
  }

  #[inline]
  pub fn to_unit(&self) -> Vec3 {
    let l = self.length();
    Vec3 {
      x: self.x / l,
      y: self.y / l,
      z: self.z / l,
    }
  }

  #[inline]
  #[allow(dead_code)]
  pub fn dot(&self, rhs: Vec3) -> f32 {
    (self.x * rhs.x()) + (self.y * rhs.y()) + (self.z * rhs.z())
  }

  #[inline]
  #[allow(dead_code)]
  pub fn cross(&self, rhs: Vec3) -> Vec3 {
    Vec3 {
      x: self.y * rhs.z() - self.z * rhs.y(),
      y: self.z * rhs.x() - self.x * rhs.z(),
      z: self.x * rhs.y() - self.y * rhs.x(),
    }
  }

  #[inline]
  pub fn to_u32_col(&self) -> u32 {
    let red: u32 = ((self.x * 255.0) as u8) as u32;
    let green: u32 = ((self.y * 255.0) as u8) as u32;
    let blue: u32 = ((self.z * 255.0) as u8) as u32;
    let alpha: u32 = 255 as u32;


    let mut c: u32 = 0;
    c = c | alpha << 24;
    c = c | red << 16;
    c = c | green << 8;
    c = c | blue << 0;
    c
  }
}

impl ops::Add<Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: Vec3) -> Vec3 {
    Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
  }
}

impl ops::Add<f32> for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: f32) -> Vec3 {
    Vec3::new(self.x() + rhs, self.y() + rhs, self.z() + rhs)
  }
}

impl ops::Add<Vec3> for f32 {
  type Output = Vec3;

  fn add(self, rhs: Vec3) -> Vec3 {
    Vec3::new(self + rhs.x(), self + rhs.y(), self + rhs.z())
  }
}

impl ops::Sub<Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, v: Vec3) -> Vec3 {
    Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
  }
}

impl ops::Sub<f32> for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: f32) -> Vec3 {
    Vec3::new(self.x() - rhs, self.y() - rhs, self.z() - rhs)
  }
}

impl ops::Mul<Vec3> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
  }
}

impl ops::Mul<f32> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: f32) -> Vec3 {
    Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
  }
}

impl ops::Mul<Vec3> for f32 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Vec3 {
    Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
  }
}

impl ops::Div<Vec3> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: Vec3) -> Vec3 {
    Vec3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
  }
}

impl ops::Div<f32> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: f32) -> Vec3 {
    Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
  }
}



#[test]
fn basic() {
  let mut v = Vec3::new(1.0, 2.0, 3.0);
  let v2 = Vec3::new(1.0, 1.0, 1.0);
  assert_eq!(v.x(), 1.0);
  assert_eq!(v.y(), 2.0);
  assert_eq!(v.z(), 3.0);
  assert_eq!(v.r(), 1.0);
  assert_eq!(v.g(), 2.0);
  assert_eq!(v.b(), 3.0);
  assert_eq!(v.length(), 3.7416575);
  assert_eq!(v.squared_length(), 14.0);
  assert_eq!((v + v2).x(), 2.0);
  assert_eq!((v + v2).y(), 3.0);
  assert_eq!((v + v2).z(), 4.0);
  assert_eq!((v + 10.0).x(), 11.0);
  assert_eq!((v + 10.0).y(), 12.0);
  assert_eq!((v + 10.0).z(), 13.0);
  assert_eq!((v - v2).x(), 0.0);
  assert_eq!((v - v2).y(), 1.0);
  assert_eq!((v - v2).z(), 2.0);
  assert_eq!((v - 10.0).x(), -9.0);
  assert_eq!((v - 10.0).y(), -8.0);
  assert_eq!((v - 10.0).z(), -7.0);
  assert_eq!((2.0 * v).x(), 2.0);
  assert_eq!((2.0 + v).x(), 3.0);
  assert_eq!(v.make_unit().x(), 0.26726124);
  assert_eq!(v.to_unit().x(), 0.26726127);
  assert_eq!(v.dot(v2), 1.6035674);
  let crossed = v.cross(v2);
  assert_eq!(crossed.x(), -0.2672612);
  assert_eq!(crossed.y(), 0.5345224);
  assert_eq!(crossed.z(), -0.26726124);
  let col = Vec3::new(10.0, 5.0, 5.0);
  assert_eq!(col.to_u32_col(), 4294376443);
}