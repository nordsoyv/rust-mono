use std::ops;

#[derive(Debug, Copy, Clone)]
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
  pub fn r(&self) -> f32 {
    self.x
  }

  #[inline]
  pub fn g(&self) -> f32 {
    self.y
  }

  #[inline]
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

impl ops::Sub<Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: Vec3) -> Vec3 {
    Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
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
  let v = Vec3::new(1.0, 2.0, 3.0);
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



}