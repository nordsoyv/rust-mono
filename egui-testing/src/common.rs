pub fn is_odd(num: i32) -> bool {
  return num & 1 != 0;
}

pub fn get_random_float(max: f32) -> f32 {
  let d = rand::random::<f32>();
  return max * d;
}

pub fn get_random_usize(max: usize) -> usize {
  let d = rand::random::<f32>();
  return (d * (max as f32)) as usize;
}

#[cfg(test)]
mod tests {
  use crate::common::get_random_usize;

  #[test]
  fn t1() {
    for n in 0..10000 {
      let r = get_random_usize(10);
      // dbg!(r);
      assert!(r < 10);
    }
  }
}
