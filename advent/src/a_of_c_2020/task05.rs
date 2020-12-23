use crate::task::Task;
use crate::util::read_file;

pub struct Task05A {}

pub struct Task05B {}

impl Task for Task05A {
  fn run(&self) {
    let max = read_file("./res/2020/task05.txt")
      .lines()
      .map(|line| BoardingPass::parse(line).id())
      .max().unwrap();
    println!("Highest id is: {}", max);
  }
}

impl Task for Task05B {
  fn run(&self) {
    let mut seats = [[false; 8]; 127];
    read_file("./res/2020/task05.txt")
      .lines()
      .map(|line| {
        let pass = BoardingPass::parse(line);
        seats[pass.row as usize][pass.col as usize] = true;
        return pass;
      })
      .collect::<Vec<BoardingPass>>();

    for (i, row) in seats.iter().enumerate() {
      for (j, col) in row.iter().enumerate() {
        print!("{},{},{} ", i, j, col);
      }
      println!()
    }
    // manual inspetion of output -> row 76, col 2 -> id = 610
  }
}

#[derive(Debug, PartialEq)]
struct BoardingPass {
  row: u32,
  col: u32,
}

impl BoardingPass {
  pub fn parse(input: &str) -> BoardingPass {
    let row_input = input
      .chars()
      .take(7)
      .map(|c| match c {
        'B' => '1',
        'F' => '0',
        _ => panic!()
      })
      .collect::<String>();
    let col_input = input
      .chars()
      .skip(7)
      .take(3)
      .map(|c| match c {
        'R' => '1',
        'L' => '0',
        _ => panic!()
      })
      .collect::<String>();
    let row = u32::from_str_radix(&row_input, 2).unwrap();
    let col = u32::from_str_radix(&col_input, 2).unwrap();
    BoardingPass {
      row,
      col,
    }
  }

  pub fn id(&self) -> u32 {
    (self.row * 8) + self.col
  }
}

#[cfg(test)]
mod test {
  use crate::a_of_c_2020::task05::BoardingPass;

  #[test]
  fn test_parse() {
    assert_eq!(BoardingPass::parse("BFFFBBFRRR"), BoardingPass { row: 70, col: 7 });
    assert_eq!(BoardingPass::parse("FFFBBBFRRR"), BoardingPass { row: 14, col: 7 });
    assert_eq!(BoardingPass::parse("BBFFBBFRLL"), BoardingPass { row: 102, col: 4 });
  }

  #[test]
  fn test_id() {
    assert_eq!(BoardingPass::parse("BFFFBBFRRR").id(), 567);
    assert_eq!(BoardingPass::parse("FFFBBBFRRR").id(), 119);
    assert_eq!(BoardingPass::parse("BBFFBBFRLL").id(), 820);
  }
}