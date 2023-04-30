mod round;
mod throw;

use crate::round::{play_round, Strategy};
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

fn simulate_round(strategy: Strategy) {
  let mut total_score = 0;
  let num_rounds = 1_000_000;
  let mut num_busts = 0;
  for _ in 0..num_rounds {
    let (score, num_throws) = play_round(&strategy);
    // println!("Scored {} in {} throws", score, num_throws);
    total_score += score;
    if score == 0 {
      num_busts += 1;
    }
  }

  println!(
    "Strategy {:?} : \n\tTotal score {} over {} rounds, Avg: {}. Number of busts {}",
    strategy,
    total_score,
    num_rounds,
    total_score / num_rounds,
    num_busts
  );
}

fn main() {
  // check_throw_value(1);
  // check_throw_value(2);
  // check_throw_value(3);
  // check_throw_value(4);
  // check_throw_value(5);
  // check_throw_value(6);
  // simulate_round(Strategy::StopOn1);
  simulate_round(Strategy::StopOn1);
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(250, 1));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(250, 2));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(250, 3));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(250, 4));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(250, 5));

  simulate_round(Strategy::ThrowUntilScoreAndLessDice(350, 1));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(350, 2));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(350, 3));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(350, 4));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(350, 5));

  simulate_round(Strategy::ThrowUntilScoreAndLessDice(450, 1));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(450, 2));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(450, 3));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(450, 4));
  simulate_round(Strategy::ThrowUntilScoreAndLessDice(450, 5));
  // simulate_round(Strategy::StopOn2);
  // simulate_round(Strategy::StopOn3);
  // simulate_round(Strategy::StopOn4);
  // simulate_round(Strategy::StopOn5);
}
