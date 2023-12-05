use crate::{task::Task, util::read_file};

pub struct Task04A {}
pub struct Task04B {}

impl Task for Task04A {
  fn run(&self) {
    let input = read_file("./res/2023/task04a.txt");
    let cards: Vec<Card> = input
      .lines()
      .into_iter()
      .map(|line| Card::from_string(line))
      .collect();
    let sum: i32 = cards.into_iter().map(|c| c.calc_winning_value()).sum();
    println!("The result is {}", sum);
  }
}

impl Task for Task04B {
  fn run(&self) {
    let input = read_file("./res/2023/task04a.txt");
    let cards: Vec<Card> = input
      .lines()
      .into_iter()
      .map(|line| Card::from_string(line))
      .collect();

    let mut copies = vec![1; cards.len()];

    for index in 0..cards.len() {
      let num_copies = copies[index];
      let num_winning = cards[index].find_number_of_winning_numbers();
      for _iters in 0..num_copies {
        for winning in 0..num_winning {
          copies[index + winning + 1] = copies[index + winning + 1] + 1;
        }
      }
    }

    let sum : usize= copies.into_iter().sum();
    println!("The result is {}", sum);
  }
}

#[derive(Debug, Default)]
struct Card {
  winning_numbers: Vec<i32>,
  numbers: Vec<i32>,
}

impl Card {
  fn from_string(input: &str) -> Card {
    let parts: Vec<&str> = input.split(":").collect();
    let number_parts: Vec<&str> = parts[1].split("|").collect();

    let winning_numbers = number_parts[0]
      .trim()
      .split(" ")
      .collect::<Vec<&str>>()
      .into_iter()
      .filter(|number| number.len() > 0)
      .map(|number| number.trim().parse::<i32>().unwrap())
      .collect::<Vec<i32>>();
    let numbers = number_parts[1]
      .trim()
      .split(" ")
      .collect::<Vec<&str>>()
      .into_iter()
      .filter(|number| number.len() > 0)
      .map(|number| number.trim().parse::<i32>().unwrap())
      .collect::<Vec<i32>>();

    Card {
      winning_numbers,
      numbers,
    }
  }

  fn calc_winning_value(&self) -> i32 {
    let num_winning = self
      .numbers
      .clone()
      .into_iter()
      .filter(|number| self.winning_numbers.contains(number))
      .collect::<Vec<i32>>()
      .len();
    if num_winning < 2 {
      return num_winning as i32;
    }
    return 2_i32.pow((num_winning - 1) as u32);
  }

  fn find_number_of_winning_numbers(&self) -> usize {
    self
      .numbers
      .clone()
      .into_iter()
      .filter(|number| self.winning_numbers.contains(number))
      .collect::<Vec<i32>>()
      .len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn task_a_example() {
    let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let cards: Vec<Card> = input
      .lines()
      .into_iter()
      .map(|line| Card::from_string(line))
      .collect();
    assert_eq!(8, cards[0].calc_winning_value());
    assert_eq!(2, cards[1].calc_winning_value());
    assert_eq!(2, cards[2].calc_winning_value());
    assert_eq!(1, cards[3].calc_winning_value());
    assert_eq!(0, cards[4].calc_winning_value());
    assert_eq!(0, cards[5].calc_winning_value());
  }

  #[test]
  fn task_b_example() {
    let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let cards: Vec<Card> = input
      .lines()
      .into_iter()
      .map(|line| Card::from_string(line))
      .collect();

    let mut copies = vec![1; cards.len()];

    for index in 0..cards.len() {
      let num_copies = copies[index];
      let num_winning = cards[index].find_number_of_winning_numbers();
      for _iters in 0..num_copies {
        for winning in 0..num_winning {
          copies[index + winning + 1] = copies[index + winning + 1] + 1;
        }
      }
    }

    let sum : usize= copies.into_iter().sum();
    dbg!(sum);
  }
}
