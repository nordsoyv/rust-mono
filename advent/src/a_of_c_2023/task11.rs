use crate::{task::Task, util::read_file};

pub struct Task11A {}
pub struct Task11B {}

impl Task for Task11A {
  fn run(&self) {
    let input = read_file("./res/2023/task11.txt");
    let task = Task11::parse(input.to_string());
    let res = task.solve_a();
    println!("The result is {}", res);
  }
}

impl Task for Task11B {
  fn run(&self) {
    println!("Empty task, not implemented yet");
  }
}

#[derive(Debug)]
struct Task11 {
  rows: Vec<Vec<char>>,
  empty_rows: Vec<usize>,
  empty_columns: Vec<usize>,
  stars: Vec<(usize, usize)>,
}

impl Task11 {
  fn parse(input: String) -> Task11 {
    let rows = input
      .lines()
      .map(|l| l.chars().collect::<Vec<char>>())
      .collect::<Vec<Vec<char>>>();
    let mut empty_rows = vec![];
    let mut empty_columns = vec![];
    let mut stars = vec![];
    for (index, row) in rows.iter().enumerate() {
      if row.iter().all(|c| *c == '.') {
        empty_rows.push(index);
      }
    }
    for column_index in 0..rows[0].len() {
      let mut is_empty = true;
      for row_index in 0..rows.len() {
        if rows[row_index][column_index] != '.' {
          is_empty = false;
          break;
        }
      }
      if is_empty {
        empty_columns.push(column_index);
      }
    }

    for (column_index, row) in rows.iter().enumerate() {
      for (row_index, c) in row.iter().enumerate() {
        if *c == '#' {
          stars.push((row_index, column_index));
        }
      }
    }

    Task11 {
      rows,
      empty_columns,
      empty_rows,
      stars,
    }
  }

  pub fn solve_a(&self) -> usize {
    let mut sum = 0;
    let mut num_pairs = 0;
    for star_index in 0..self.stars.len() {
      for second_star_index in star_index+1..self.stars.len() {
        num_pairs += 1;
        let star1 = self.stars[star_index];
        let star2 = self.stars[second_star_index];
        let x_range = if star1.0 < star2.0 {
          star1.0..star2.0
        } else {
          star2.0..star1.0
        };
        let y_range = if star1.1 < star2.1 {
          star1.1..star2.1
        } else {
          star2.1..star1.1
        };
        let mut extra_x = 0;
        let mut extra_y = 0;
        self.empty_columns.iter().for_each(|column| {
          if x_range.contains(column) {
            extra_x += 1
          }
        });
        self.empty_rows.iter().for_each(|row| {
          if y_range.contains(row) {
            extra_y += 1
          }
        });

        let x_dist = x_range.end - x_range.start;
        let y_dist = y_range.end - y_range.start;

        sum += x_dist + y_dist + extra_x + extra_y;
      }
    }
    dbg!(num_pairs);
    sum
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

const TEST_INPUT2: &str = "#...#";



  #[test]
  fn task_a_example() {
    let task = Task11::parse(TEST_INPUT.to_string());
    assert_eq!(374, task.solve_a());
  }
  #[test]
  fn task_a_example2() {
    let task = Task11::parse(TEST_INPUT2.to_string());
    assert_eq!(7, task.solve_a());
  }
}
