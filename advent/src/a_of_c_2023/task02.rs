use crate::{task::Task, util::read_file};

pub struct Task02A {}
pub struct Task02B {}

impl Task for Task02A {
  fn run(&self) {
    let input = read_file("./res/2023/task02a.txt");
    let game_infos = make_game_infos(input);
    let bag = BagContent {
      blue: 14,
      red: 12,
      green: 13,
    };
    let sum = game_infos
      .into_iter()
      .filter(|game| game.is_possible(&bag))
      .fold(0, |sum, number| sum + number.game_id);
    println!("The result is {}", sum);
  }
}

impl Task for Task02B {
  fn run(&self) {
    let input = read_file("./res/2023/task02a.txt");
    let game_infos = make_game_infos(input);
    let sum = game_infos
      .into_iter()
      .map(|g| g.calc_power())
      .fold(0, |sum, number| sum + number);
    println!("The result is {}", sum);
  }
}

fn make_game_infos(input: String) -> Vec<GameInfo> {
  input.lines().map(|line| GameInfo::parse(line)).collect()
}
#[derive(Debug)]
struct GameInfo {
  game_id: i32,
  red: i32,
  green: i32,
  blue: i32,
}

#[derive(Debug)]
struct BagContent {
  red: i32,
  green: i32,
  blue: i32,
}

impl GameInfo {
  // line is "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
  fn parse(line: &str) -> GameInfo {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let parts: Vec<&str> = line.split(":").collect();
    let game_id = parts[0].replace("Game ", "").parse::<i32>().unwrap();
    let draws: Vec<&str> = parts[1].split(";").collect();

    for draw in draws {
      draw.split(",").for_each(|d| {
        let part: Vec<&str> = d.trim().split(" ").collect();
        let count = part[0].parse::<i32>().unwrap();
        match part[1] {
          "red" => {
            if count > red {
              red = count
            }
          }
          "green" => {
            if count > green {
              green = count
            }
          }
          "blue" => {
            if count > blue {
              blue = count
            }
          }
          _ => panic!("unknown color {}", part[1]),
        }
      });
    }
    GameInfo {
      game_id,
      red,
      green,
      blue,
    }
  }

  fn is_possible(&self, bag: &BagContent) -> bool {
    self.blue <= bag.blue && self.green <= bag.green && self.red <= bag.red
  }

  fn calc_power(&self) -> i32 {
    self.blue * self.red * self.green
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn task_a_example() {
    let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
      .to_string();
    let result = make_game_infos(input);
    let bag = BagContent {
      blue: 14,
      red: 12,
      green: 13,
    };
    let sum = result
      .into_iter()
      .filter(|game| game.is_possible(&bag))
      .fold(0, |sum, number| sum + number.game_id);
    assert_eq!(8, sum);
  }
  #[test]
  fn task_b_example() {
    let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
      .to_string();
    let result = make_game_infos(input);
    let sum = result
      .into_iter()
      .map(|g| g.calc_power())
      .fold(0, |sum, number| sum + number);
    assert_eq!(2286, sum);
  }
  #[test]
  fn parse_game5() {
    let input = r"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let game = GameInfo::parse(input);
    assert_eq!(5, game.game_id);
    assert_eq!(6, game.red);
    assert_eq!(3, game.green);
    assert_eq!(2, game.blue);
  }
}
