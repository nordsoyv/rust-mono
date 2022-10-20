mod throw;

use crate::throw::{Throw, ThrowType, ThrowValue};
use std::collections::HashMap;

fn count_stats(
  stats_map: &mut HashMap<ThrowType, usize>,
  next_throw_map: &mut HashMap<usize, usize>,
  throw_value: ThrowValue,
) {
  let stats = stats_map.get(&throw_value.throw_type);
  if let Some(count) = stats {
    stats_map.insert(throw_value.throw_type, count + 1);
  } else {
    stats_map.insert(throw_value.throw_type, 1);
  }
  let next_throw = next_throw_map.get(&throw_value.remaining_dice);
  if let Some(count) = next_throw {
    next_throw_map.insert(throw_value.remaining_dice, count + 1);
  } else {
    next_throw_map.insert(throw_value.remaining_dice, 1);
  }
}

fn check_throw_value(num_dice: usize) {
  println!("Checking for {:?} dice", num_dice);
  let mut throw = Throw::new(num_dice);
  let mut throw_stats: HashMap<ThrowType, usize> = HashMap::new();
  let mut next_throw_stats: HashMap<usize, usize> = HashMap::new();
  let mut sum = 0;

  let num_throws = 1000000;
  for _ in 0..num_throws {
    throw.throw();
    let value = throw.evaluate();
    sum += value.value;
    count_stats(&mut throw_stats, &mut next_throw_stats, value);
  }
  let num_busts = throw_stats
    .get(&ThrowType::Bust)
    .expect("Should always bust");
  println!("\tMade {:?} throws", num_throws);
  println!(
    "\tAverage throw value when not busting {:?}",
    sum / (num_throws - num_busts)
  );
  println!(
    "\tChance of getting a new throw  ({:?} %)",
    100.0 - (num_busts * 100) as f32 / num_throws as f32
  );
  // println!("{:?}", throw_stats);
  println!("Next throw");
  for remaning_dice in 0..=num_dice - 1 {
    println!(
      "\tNext throw with {:?} dice : {:?} ({:?})",
      match remaning_dice {
        0 => 6,
        _ => remaning_dice,
      },
      next_throw_stats.get(&remaning_dice).unwrap(),
      *next_throw_stats.get(&remaning_dice).unwrap() as f32 / num_throws as f32
    );
  }
  // println!("{:?}", next_throw_stats);
}

fn main() {
  check_throw_value(1);
  check_throw_value(2);
  check_throw_value(3);
  check_throw_value(4);
  check_throw_value(5);
  check_throw_value(6);
}
