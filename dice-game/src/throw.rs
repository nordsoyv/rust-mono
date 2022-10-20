use rand::prelude::*;

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct Dice {
  value: usize,
}

impl Dice {
  #[allow(dead_code)]
  pub fn new_value(value: usize) -> Dice {
    Dice { value }
  }
  #[allow(dead_code)]
  pub fn throw() -> Dice {
    Dice {
      value: thread_rng().gen_range(1..=6),
    }
  }

  pub fn re_throw(&mut self) {
    self.value = thread_rng().gen_range(1..=6);
  }
}

#[derive(PartialEq, Debug, Eq, Hash, Copy, Clone)]
pub enum ThrowType {
  Bust,
  Other,
  ThreeEqual,
  FourEqual,
  FiveEqual,
  SixEqual,
  ThreePairs,
  TwoTrios,
  FourAndTwo,
  Straight,
}

pub struct ThrowValue {
  pub throw_type: ThrowType,
  pub value: usize,
  pub remaining_dice: usize,
}

pub struct Throw {
  num_dice: usize,
  dice: Vec<Dice>,
  used_dice: Vec<bool>,
  frequencies: Vec<usize>,
}

impl Throw {
  pub fn new(num_dice: usize) -> Throw {
    Throw {
      num_dice,
      frequencies: vec![0; num_dice],
      used_dice: vec![false; num_dice],
      dice: vec![Dice::default(); num_dice],
    }
  }

  #[allow(dead_code)]
  pub fn set_throw(dice: Vec<Dice>) -> Throw {
    let mut frequencies = vec![0; 6];

    for dice in &dice {
      frequencies[dice.value - 1] += 1;
    }
    let t = Throw {
      num_dice: dice.len(),
      frequencies,
      used_dice: vec![false; dice.len()],
      dice,
    };
    t
  }

  pub fn throw(&mut self) {
    self.used_dice = vec![false; self.num_dice];
    self.frequencies = vec![0; 6];
    for dice in &mut self.dice {
      dice.re_throw();
      self.frequencies[dice.value - 1] += 1;
    }
  }

  pub fn evaluate(&mut self) -> ThrowValue {
    let mut throw_value = ThrowValue {
      throw_type: ThrowType::Bust,
      remaining_dice: self.num_dice,
      value: 0,
    };

    if is_straight(&self.frequencies) {
      throw_value.throw_type = ThrowType::Straight;
      throw_value.value = 1500;
      throw_value.remaining_dice = 0;
      for i in 0..6 {
        self.used_dice[i] = true;
      }
      return throw_value;
    }

    if find_three_pairs(&self.frequencies) {
      throw_value.throw_type = ThrowType::ThreePairs;
      throw_value.value += 1500;
      throw_value.remaining_dice -= 6;
      for i in 0..6 {
        self.used_dice[i] = true;
      }
      return throw_value;
    }

    if find_four_plus_two(&self.frequencies) {
      throw_value.throw_type = ThrowType::FourAndTwo;
      throw_value.value += 1500;
      throw_value.remaining_dice -= 6;
      for i in 0..6 {
        self.used_dice[i] = true;
      }
      return throw_value;
    }
    let mut found_trio = false;
    for (count_index, count) in self.frequencies.iter().enumerate() {
      if *count == 6 {
        // found a hexa?
        throw_value.throw_type = ThrowType::SixEqual;
        throw_value.value += 3000;
        throw_value.remaining_dice -= 6;
        for (i, dice) in self.dice.iter().enumerate() {
          if dice.value == count_index + 1 {
            self.used_dice[i] = true;
          }
        }
      }
      if *count == 5 {
        // found a penta?
        throw_value.throw_type = ThrowType::FiveEqual;
        throw_value.value += 2000;
        throw_value.remaining_dice -= 5;
        for (i, dice) in self.dice.iter().enumerate() {
          if dice.value == count_index + 1 {
            self.used_dice[i] = true;
          }
        }
      }
      if *count == 4 {
        // found a quad
        throw_value.throw_type = ThrowType::FourEqual;
        throw_value.value += 1000;
        throw_value.remaining_dice -= 4;
        for (i, dice) in self.dice.iter().enumerate() {
          if dice.value == count_index + 1 {
            self.used_dice[i] = true;
          }
        }
      }
      if *count == 3 {
        // found a trio
        if found_trio {
          // second one we find
          throw_value.throw_type = ThrowType::TwoTrios;
          throw_value.value = 1500;
          throw_value.remaining_dice = 0;
        } else {
          found_trio = true;
          throw_value.value += (count_index + 1) * 100;
          if count_index + 1 == 1 {
            // special case for trio of ones
            throw_value.value = 300;
          }
          throw_value.remaining_dice -= 3;
          throw_value.throw_type = ThrowType::ThreeEqual;
        }

        for (i, dice) in self.dice.iter().enumerate() {
          if dice.value == count_index + 1 {
            self.used_dice[i] = true;
          }
        }
      }
    }

    for (i, dice) in self.dice.iter().enumerate() {
      if self.used_dice[i] {
        continue;
      }
      if is_one(dice) {
        if throw_value.throw_type == ThrowType::Bust {
          throw_value.throw_type = ThrowType::Other;
        }
        throw_value.value += 100;
        throw_value.remaining_dice -= 1;
        self.used_dice[i] = true;
      }
      if is_five(dice) {
        if throw_value.throw_type == ThrowType::Bust {
          throw_value.throw_type = ThrowType::Other;
        }
        throw_value.value += 50;
        throw_value.remaining_dice -= 1;
        self.used_dice[i] = true;
      }
    }

    return throw_value;
  }
}

fn is_one(dice: &Dice) -> bool {
  dice.value == 1
}
fn is_five(dice: &Dice) -> bool {
  dice.value == 5
}

pub fn is_straight(counts: &Vec<usize>) -> bool {
  for count in counts {
    if *count != 1 {
      return false;
    }
  }
  return true;
}

fn find_three_pairs(counts: &Vec<usize>) -> bool {
  let mut num_2s = 0;
  for count in counts {
    if *count == 2 {
      num_2s += 1;
    }
  }
  return num_2s == 3;
}
fn find_four_plus_two(counts: &Vec<usize>) -> bool {
  let mut found_pair = false;
  let mut found_quad = false;
  for count in counts {
    if *count == 2 {
      found_pair = true;
    }
    if *count == 4 {
      found_quad = true;
    }
  }
  return found_pair && found_quad;
}

#[cfg(test)]
mod test {
  use crate::throw::ThrowType;
  use crate::throw::{Dice, Throw};

  macro_rules! throw {
    ($($x:literal)+) => {
      Throw::set_throw( vec![$(Dice::new_value($x),)+])
    }
  }

  macro_rules! dice {
    ($($x:literal)+) => {
      vec![$(Dice::new_value($x),)+]
    };
  }

  macro_rules! check_throw {
    ($throw:expr ,  $value:literal , $remaining:literal) => {{
      let throw_value = $throw.evaluate();
      assert_eq!($value, throw_value.value);
      assert_eq!($remaining, throw_value.remaining_dice);
    }};
    ($throw:expr , $throwType:expr ,  $value:literal , $remaining:literal) => {{
      let throw_value = $throw.evaluate();
      assert_eq!($throwType, throw_value.throw_type);
      assert_eq!($value, throw_value.value);
      assert_eq!($remaining, throw_value.remaining_dice);
    }};
  }

  #[test]
  fn eval_single_dice_throw() {
    check_throw!(throw!(1), ThrowType::Other, 100, 0);
    check_throw!(throw!(2), ThrowType::Bust, 0, 1);
    check_throw!(throw!(3), ThrowType::Bust, 0, 1);
    check_throw!(throw!(4), ThrowType::Bust, 0, 1);
    check_throw!(throw!(5), ThrowType::Other, 50, 0);
    check_throw!(throw!(6), ThrowType::Bust, 0, 1);
  }

  #[test]
  fn eval_2_throw_throw() {
    check_throw!(throw!(1 2), ThrowType::Other, 100, 1);
    check_throw!(throw!(5 2), ThrowType::Other, 50, 1);
    check_throw!(throw!(5 1), ThrowType::Other, 150, 0);
    check_throw!(throw!(3 2), ThrowType::Bust, 0, 2);
  }
  #[test]
  fn eval_3_throw_throw() {
    check_throw!(throw!(1 1 1), ThrowType::ThreeEqual, 300, 0);
    check_throw!(throw!(2 2 2), ThrowType::ThreeEqual, 200, 0);
    check_throw!(throw!(3 3 3), ThrowType::ThreeEqual, 300, 0);
    check_throw!(throw!(4 4 4), ThrowType::ThreeEqual, 400, 0);
    check_throw!(throw!(5 5 5), ThrowType::ThreeEqual, 500, 0);
    check_throw!(throw!(6 6 6), ThrowType::ThreeEqual, 600, 0);
    check_throw!(throw!(1 5 1), ThrowType::Other, 250, 0);
    check_throw!(throw!(1 6 2), ThrowType::Other, 100, 2);
    check_throw!(throw!(1 6 2), ThrowType::Other, 100, 2);
  }

  #[test]
  fn eval_4_throw_throw() {
    check_throw!(throw!(1 1 1 1), ThrowType::FourEqual, 1000, 0);
    check_throw!(throw!(2 2 2 2), ThrowType::FourEqual, 1000, 0);
    check_throw!(throw!(3 3 3 3), ThrowType::FourEqual, 1000, 0);
    check_throw!(throw!(4 4 4 4), ThrowType::FourEqual, 1000, 0);
    check_throw!(throw!(5 5 5 5), ThrowType::FourEqual, 1000, 0);
    check_throw!(throw!(6 6 6 6), ThrowType::FourEqual, 1000, 0);
    check_throw!(throw!(1 5 1 1), ThrowType::ThreeEqual, 350, 0);
    check_throw!(throw!(1 6 2 4), ThrowType::Other, 100, 3);
    check_throw!(throw!(1 6 6 6), ThrowType::ThreeEqual, 700, 0);
    check_throw!(throw!(6 6 6 1), ThrowType::ThreeEqual, 700, 0);
    check_throw!(throw!(6 6 6 5), ThrowType::ThreeEqual, 650, 0);
  }
  #[test]
  fn eval_5_throw_throw() {
    check_throw!(throw!(1 1 1 1 1), ThrowType::FiveEqual, 2000, 0);
    check_throw!(throw!(2 2 2 2 2), ThrowType::FiveEqual, 2000, 0);
    check_throw!(throw!(3 3 3 3 3), ThrowType::FiveEqual, 2000, 0);
    check_throw!(throw!(4 4 4 4 4), ThrowType::FiveEqual, 2000, 0);
    check_throw!(throw!(5 5 5 5 5), ThrowType::FiveEqual, 2000, 0);
    check_throw!(throw!(6 6 6 6 6), ThrowType::FiveEqual, 2000, 0);
    check_throw!(throw!(6 6 6 6 3), ThrowType::FourEqual, 1000, 1);
    check_throw!(throw!(6 6 6 1 6), ThrowType::FourEqual, 1100, 0);
    check_throw!(throw!(1 5 1 1 1), ThrowType::FourEqual, 1050, 0);
    check_throw!(throw!(1 6 2 4 3), ThrowType::Other, 100, 4);
    check_throw!(throw!(6 6 6 5 1), ThrowType::ThreeEqual, 750, 0);
    check_throw!(throw!(2 4 2 3 6), ThrowType::Bust, 0, 5);
  }
  #[test]
  fn eval_6_throw_throw() {
    check_throw!(throw!(1 1 1 1 1 1), ThrowType::SixEqual, 3000, 0);
    check_throw!(throw!(2 2 2 2 2 2), ThrowType::SixEqual, 3000, 0);
    check_throw!(throw!(3 3 3 3 3 3), ThrowType::SixEqual, 3000, 0);
    check_throw!(throw!(4 4 4 4 4 4), ThrowType::SixEqual, 3000, 0);
    check_throw!(throw!(5 5 5 5 5 5), ThrowType::SixEqual, 3000, 0);
    check_throw!(throw!(6 6 6 6 6 6), ThrowType::SixEqual, 3000, 0);
    check_throw!(throw!(1 6 2 4 2 6), ThrowType::Other, 100, 5);
    check_throw!(throw!(1 6 6 6 6 6), ThrowType::FiveEqual, 2100, 0);
    check_throw!(throw!(6 6 6 1 6 3), ThrowType::FourEqual, 1100, 1);
    check_throw!(throw!(6 6 6 5 3 1), ThrowType::ThreeEqual, 750, 1);
    check_throw!(throw!(1 2 3 4 5 6), ThrowType::Straight, 1500, 0);
    check_throw!(throw!(6 5 4 3 2 1), ThrowType::Straight, 1500, 0);
    check_throw!(throw!(1 5 1 1 5 5), ThrowType::TwoTrios, 1500, 0);
    check_throw!(throw!(3 4 3 4 3 4), ThrowType::TwoTrios, 1500, 0);
    check_throw!(throw!(2 2 2 3 3 3), ThrowType::TwoTrios, 1500, 0);
    check_throw!(throw!(5 5 5 3 3 3), ThrowType::TwoTrios, 1500, 0);
    check_throw!(throw!(2 2 3 3 4 4), ThrowType::ThreePairs, 1500, 0); // tre par
    check_throw!(throw!(1 1 3 3 5 5), ThrowType::ThreePairs, 1500, 0); // tre par
    check_throw!(throw!(2 2 2 2 4 4), ThrowType::FourAndTwo, 1500, 0); // 4+2
  }

  #[test]
  fn macro_tests_throw() {
    let t = throw![1 2 3 4 5 6];
    assert_eq!(t.dice.len(), 6);
  }

  #[test]
  fn macro_tests_single() {
    let d = dice![1];
    assert_eq!(d.len(), 1);
    assert_eq!(d, vec![Dice::new_value(1)]);
  }
  #[test]
  fn macro_tests_duo() {
    let d = dice![1 2];
    assert_eq!(d.len(), 2);
    assert_eq!(d, vec![Dice::new_value(1), Dice::new_value(2)]);
  }
  #[test]
  fn macro_tests_many() {
    let d = dice![1 2 3 4 5 6];
    assert_eq!(d.len(), 6);
    assert_eq!(
      d,
      vec![
        Dice::new_value(1),
        Dice::new_value(2),
        Dice::new_value(3),
        Dice::new_value(4),
        Dice::new_value(5),
        Dice::new_value(6)
      ]
    );
  }
}
