use crate::{task::Task, util::read_file};

pub struct Task07A {}

pub struct Task07B {}

impl Task for Task07A {
  fn run(&self) {
    let input = read_file("./res/2023/task07.txt");
    let mut task = TaskA::parse(input.to_string());
    let res = task.solve();
    println!("The result is {}", res);
  }
}

impl Task for Task07B {
  fn run(&self) {
    let input = read_file("./res/2023/task07.txt");
    let mut task = TaskB::parse(input.to_string());
    let res = task.solve();
    println!("The result is {}", res);
  }
}

#[derive(Debug)]
struct TaskA {
  deals: Vec<Deal>,
}

impl TaskA {
  pub fn solve(&mut self) -> usize {
    self
      .deals
      .sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());
    self
      .deals
      .iter()
      .enumerate()
      .map(|(index, deal)| deal.bet * (index + 1))
      .fold(0, |acc, num| acc + num)
  }
  pub fn parse(value: String) -> Self {
    let deals: Vec<Deal> = value
      .lines()
      .map(|line| {
        let (cards, bet) = line.split_once(" ").unwrap();
        let cards: Vec<Card> = cards
          .chars()
          .map(|c| match c {
            '2' => Card::Number(2),
            '3' => Card::Number(3),
            '4' => Card::Number(4),
            '5' => Card::Number(5),
            '6' => Card::Number(6),
            '7' => Card::Number(7),
            '8' => Card::Number(8),
            '9' => Card::Number(9),
            'T' => Card::Number(10),
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Unexptected char when parsing hand {c}"),
          })
          .collect();
        let bet = bet.parse::<usize>().unwrap();

        let mut sorted = cards.clone();
        sorted.sort();

        let hand = if sorted[0] == sorted[4] {
          Hand::FiveOfAKind(cards)
        } else if sorted[0] == sorted[3] || sorted[1] == sorted[4] {
          Hand::FourOfAKind(cards)
        } else if (sorted[0] == sorted[2] && sorted[3] == sorted[4])
          || (sorted[0] == sorted[1] && sorted[2] == sorted[4])
        {
          Hand::FullHouse(cards)
        } else if sorted[0] == sorted[2] || sorted[1] == sorted[3] || sorted[2] == sorted[4] {
          Hand::ThreeOfAKind(cards)
        } else if (sorted[0] == sorted[1] && sorted[2] == sorted[3])
          || (sorted[0] == sorted[1] && sorted[3] == sorted[4])
          || (sorted[1] == sorted[2] && sorted[3] == sorted[4])
        {
          Hand::TwoPair(cards)
        } else if sorted[0] == sorted[1]
          || sorted[1] == sorted[2]
          || sorted[2] == sorted[3]
          || sorted[3] == sorted[4]
        {
          Hand::OnePair(cards)
        } else {
          Hand::HighCard(cards)
        };
        Deal { bet, hand }
      })
      .collect();
    return TaskA { deals };
  }
}


#[derive(Debug)]
struct TaskB {
  deals: Vec<Deal>,
}

impl TaskB {
  pub fn solve(&mut self) -> usize {
    self
      .deals
      .sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());
    self
      .deals
      .iter()
      .enumerate()
      .map(|(index, deal)| deal.bet * (index + 1))
      .fold(0, |acc, num| acc + num)
  }
  pub fn parse(value: String) -> Self {
    let deals: Vec<Deal> = value
      .lines()
      .map(|line| {
        dbg!(&line);
        let (cards, bet) = line.split_once(" ").unwrap();
        let cards: Vec<Card> = cards
          .chars()
          .map(|c| match c {
            '2' => Card::Number(2),
            '3' => Card::Number(3),
            '4' => Card::Number(4),
            '5' => Card::Number(5),
            '6' => Card::Number(6),
            '7' => Card::Number(7),
            '8' => Card::Number(8),
            '9' => Card::Number(9),
            'T' => Card::Number(10),
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Unexptected char when parsing hand {c}"),
          })
          .collect();
        let bet = bet.parse::<usize>().unwrap();

        let mut sorted = cards.clone();
        sorted.sort();
        dbg!(&sorted);
        let hand = if sorted[0] == sorted[4] {
          Hand::FiveOfAKind(cards)
        } else if sorted[0] == sorted[3] || sorted[1] == sorted[4] {
          if sorted[0] == Card::Joker {
            Hand::FiveOfAKind(cards)
          } else {
            Hand::FourOfAKind(cards)
          }
        } else if (sorted[0] == sorted[2] && sorted[3] == sorted[4])
          || (sorted[0] == sorted[1] && sorted[2] == sorted[4])
        {
          if sorted[0] == Card::Joker {
            Hand::FiveOfAKind(cards)
          } else {
            Hand::FullHouse(cards)
          }
        } else if sorted[0] == sorted[2] || sorted[1] == sorted[3] || sorted[2] == sorted[4] {
          if sorted[0] == Card::Joker && sorted[1] == Card::Joker && sorted[2] == Card::Joker {
            Hand::FourOfAKind(cards)
          } else if sorted[0] == Card::Joker && sorted[1] == Card::Joker {
            Hand::FiveOfAKind(cards)
          } else if sorted[0] == Card::Joker {
            Hand::FourOfAKind(cards)
          } else {
            Hand::ThreeOfAKind(cards)
          }
        } else if (sorted[0] == sorted[1] && sorted[2] == sorted[3])
          || (sorted[0] == sorted[1] && sorted[3] == sorted[4])
          || (sorted[1] == sorted[2] && sorted[3] == sorted[4])
        {
          if sorted[0] == Card::Joker && sorted[1] == Card::Joker {
            Hand::FourOfAKind(cards)
          } else if sorted[0] == Card::Joker {
            Hand::FullHouse(cards)
          } else {
            Hand::TwoPair(cards)
          }
        } else if sorted[0] == sorted[1]
          || sorted[1] == sorted[2]
          || sorted[2] == sorted[3]
          || sorted[3] == sorted[4]
        {
          if sorted[0] == Card::Joker && sorted[1] == Card::Joker && sorted[2] == Card::Joker {
            Hand::FiveOfAKind(cards)
          } else if sorted[0] == Card::Joker && sorted[1] == Card::Joker {
            Hand::ThreeOfAKind(cards)
          } else if sorted[0] == Card::Joker {
            Hand::ThreeOfAKind(cards)
          } else {
            Hand::OnePair(cards)
          }
        } else {
          if sorted[0] == Card::Joker
            && sorted[1] == Card::Joker
            && sorted[2] == Card::Joker
            && sorted[3] == Card::Joker
          {
            Hand::FiveOfAKind(cards)
          } else if sorted[0] == Card::Joker && sorted[1] == Card::Joker && sorted[2] == Card::Joker
          {
            Hand::FourOfAKind(cards)
          } else if sorted[0] == Card::Joker && sorted[1] == Card::Joker {
            Hand::ThreeOfAKind(cards)
          } else if sorted[0] == Card::Joker {
            Hand::OnePair(cards)
          } else {
            Hand::HighCard(cards)
          }
        };
        Deal { bet, hand }
      })
      .collect();
    return TaskB { deals };
  }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
enum Card {
  Joker,
  Number(u32),
  Jack,
  Queen,
  King,
  Ace,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Hand {
  HighCard(Vec<Card>),
  OnePair(Vec<Card>),
  TwoPair(Vec<Card>),
  ThreeOfAKind(Vec<Card>),
  FullHouse(Vec<Card>),
  FourOfAKind(Vec<Card>),
  FiveOfAKind(Vec<Card>),
}

#[derive(Debug)]
struct Deal {
  hand: Hand,
  bet: usize,
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn task_a_parsing_example() {
    let _task1: TaskA = TaskA::parse(
      "23456 34
23452 23
34374 34
37633 45
94499 34
45444 483
99999 483"
        .to_string(),
    );
  }

  #[test]
  fn task_a_example() {
    let mut task1: TaskA = TaskA::parse(
      "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .to_string(),
    );
    let res = task1.solve();
    assert_eq!(6440, res)
  }

  #[test]
  fn task_b_example() {
    let mut task = TaskB::parse(
      "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .to_string(),
    );
    let res = task.solve();
    assert_eq!(5905, res)
  }

  #[test]
  fn task_a_example2() {
    let mut task1: TaskA = TaskA::parse(
      "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"
        .to_string(),
    );
    let res = task1.solve();
    assert_eq!(6592, res)
  }

  #[test]
  fn task_b_example2() {
    let mut task = TaskB::parse(
      "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"
        .to_string(),
    );

    let res = task.solve();
    assert_eq!(6839, res)
  }
}