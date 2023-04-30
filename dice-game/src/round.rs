use crate::{Throw, ThrowType};

#[derive(Debug)]
pub enum Strategy {
  ThrowUntilScore(usize),
  StopOn1,
  ThrowUntilScoreStopOn1(usize),
  StopOn2,
  StopOn3,
  StopOn4,
  StopOn5,
  ThrowUntilScoreAndLessDice(usize, usize),
}

pub fn play_round(strategy: &Strategy) -> (usize, usize) {
  let mut dice_to_throw = 6;
  let mut num_throws = 0;
  let mut score = 0;
  loop {
    let mut throw = Throw::new(dice_to_throw);
    throw.throw();
    num_throws += 1;
    let eval = throw.evaluate();
    if eval.throw_type == ThrowType::Bust {
      // println!("BUST");
      return (0, num_throws);
    }
    score += eval.value;
    dice_to_throw = eval.remaining_dice;
    if dice_to_throw == 0 {
      dice_to_throw = 6;
    }
    match strategy {
      Strategy::ThrowUntilScoreAndLessDice(target_score, remaning_dice) => {
        if score > *target_score && *remaning_dice <= dice_to_throw {
          return (score, num_throws);
        }
      }
      Strategy::ThrowUntilScore(safety) => {
        if score > *safety {
          return (score, num_throws);
        }
      }
      Strategy::StopOn1 => {
        if dice_to_throw < 2 {
          // println!("Scored {} in {} throws", score, num_throws);
          return (score, num_throws);
        }
      }
      Strategy::ThrowUntilScoreStopOn1(safety) => {
        if score > *safety && dice_to_throw < 2 {
          return (score, num_throws);
        }
      }
      Strategy::StopOn2 => {
        if dice_to_throw < 3 {
          return (score, num_throws);
        }
      }
      Strategy::StopOn3 => {
        if dice_to_throw < 4 {
          return (score, num_throws);
        }
      }
      Strategy::StopOn4 => {
        if dice_to_throw < 5 {
          return (score, num_throws);
        }
      }
      Strategy::StopOn5 => {
        if dice_to_throw < 6 {
          return (score, num_throws);
        }
      }
    }
  }
}
