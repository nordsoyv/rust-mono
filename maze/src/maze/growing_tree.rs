use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

use crate::canvas::Canvas;
use crate::cell::Cell;
use crate::common::{Direction, NUM_CELLS, Wall};

pub enum Strategy {
  Last,
  First,
  FirstN(i32),
  Random,
  LastN(i32),
}

pub struct GrowingTreeMaze {
  cells: Vec<Cell>,
  rng: ThreadRng,
  random: Uniform<f32>,
  stack: Vec<(i32, i32)>,
  pub done: bool,
  strategy: Strategy,
}

impl GrowingTreeMaze {
  pub fn new(strategy: Strategy) -> GrowingTreeMaze {
    GrowingTreeMaze {
      rng: rand::thread_rng(),
      random: Uniform::from(0f32..1f32),
      cells: vec![],
      stack: vec![],
      done: false,
      strategy,
    }
  }

  fn get_random(&mut self, max: usize) -> usize {
    let d = self.random.sample(&mut self.rng);
    let scaled = d * max as f32;
    let scaled_int = scaled as usize;
    return scaled_int;
  }

  fn get_cell(&self, x: i32, y: i32) -> &Cell {
    let index = (y * NUM_CELLS) + x;
    return &self.cells[index as usize];
  }

  fn get_mut_cell(&mut self, x: i32, y: i32) -> &mut Cell {
    let index = y * NUM_CELLS + x;
    return &mut self.cells[index as usize];
  }

  fn can_carve(&self, x: i32, y: i32, dir: Direction) -> bool {
    let target_x = match dir {
      Direction::West => x - 1,
      Direction::East => x + 1,
      _ => x
    };
    let target_y = match dir {
      Direction::South => y + 1,
      Direction::North => y - 1,
      _ => y
    };

    if target_x < 0 || target_x >= NUM_CELLS || target_y < 0 || target_y >= NUM_CELLS {
      return false;
    }

    let target_cell = self.get_cell(target_x, target_y);
    if !target_cell.part_of_maze {
      return true;
    }
    return false;
  }

  fn get_cell_in_dir(&self, x: i32, y: i32, dir: Direction) -> (i32, i32) {
    match dir {
      Direction::North => (x, y - 1),
      Direction::South => (x, y + 1),
      Direction::East => (x + 1, y),
      Direction::West => (x - 1, y),
    }
  }

  fn carve(&mut self, x_start: i32, y_start: i32, dir: Direction) {
    let x_end = match dir {
      Direction::West => x_start - 1,
      Direction::East => x_start + 1,
      _ => x_start
    };
    let y_end = match dir {
      Direction::South => y_start + 1,
      Direction::North => y_start - 1,
      _ => y_start
    };
    if x_start < 0 || x_end < 0
      || y_start < 0 || y_end < 0
      || x_start > NUM_CELLS || x_end > NUM_CELLS
      || y_start > NUM_CELLS || y_end > NUM_CELLS {
      return;
    }
    {
      let start_cell = self.get_mut_cell(x_start, y_start);
      start_cell.part_of_maze = true;
      match dir {
        Direction::North => {
          start_cell.top = Wall::None;
        }
        Direction::South => {
          start_cell.bottom = Wall::None;
        }
        Direction::East => {
          start_cell.right = Wall::None;
        }
        Direction::West => {
          start_cell.left = Wall::None;
        }
      }
    }
    {
      let end_cell = self.get_mut_cell(x_end, y_end);
      end_cell.part_of_maze = true;
      match dir {
        Direction::North => {
          end_cell.bottom = Wall::None;
        }
        Direction::South => {
          end_cell.top = Wall::None;
        }
        Direction::East => {
          end_cell.left = Wall::None;
        }
        Direction::West => {
          end_cell.right = Wall::None;
        }
      }
    }
  }

  pub fn init(&mut self) {
    for y in 0..NUM_CELLS {
      for x in 0..NUM_CELLS {
        self.cells.push(Cell::default(x, y));
      }
    }
    self.get_mut_cell(NUM_CELLS / 2, 0).top = Wall::None;
    self.get_mut_cell(NUM_CELLS / 2, NUM_CELLS - 1).bottom = Wall::None;
    self.stack.push((NUM_CELLS / 2, NUM_CELLS / 2));
    self.get_mut_cell(NUM_CELLS / 2, NUM_CELLS / 2).part_of_maze = true;
  }

  pub fn generate(&mut self) {
    while self.done == false {
      self.generate_step();
    }
  }

  fn get_next_index(&mut self) -> usize {
    match self.strategy {
      Strategy::Random => self.get_random(self.stack.len()),
      Strategy::First => 0,
      Strategy::FirstN(num) => {
        let n = self.get_random(num as usize);
        if n >= self.stack.len() {
          self.stack.len()-1
        }else {
          n as usize
        }
      }
      Strategy::Last => self.stack.len() - 1,
      Strategy::LastN(num) => {
        let n = self.get_random(num as usize);
        let index: i32 = self.stack.len() as i32  - 1 - n as i32;
        if index < 0 {
          0
        }else {
          index as usize
        }
      }
    }
//    let next_cell_index = self.get_random(self.stack.len());
//    return next_cell_index;
  }

  pub fn generate_step(&mut self) {
    if self.stack.len() == 0 {
      self.done = true;
      return;
    }
    let next_cell_index = self.get_next_index();

    let (x, y) = self.stack[next_cell_index];

    self.get_mut_cell(x, y).active_cell = true;

    let available_dirs = self.get_allowed_directions(x, y);
    if available_dirs.len() == 0 {
      self.get_mut_cell(x, y).active_cell = false;
      self.stack.remove(next_cell_index);
      return;
    }
    let random_dir = self.get_random(available_dirs.len());
    self.carve(x, y, available_dirs[random_dir]);
    let next_cell = self.get_cell_in_dir(x, y, available_dirs[random_dir]);
    self.stack.push(next_cell);
  }

  fn get_allowed_directions(&self, x: i32, y: i32) -> Vec<Direction> {
    let mut dirs = vec![];
    if self.can_carve(x, y, Direction::North) {
      dirs.push(Direction::North);
    }
    if self.can_carve(x, y, Direction::South) {
      dirs.push(Direction::South);
    }
    if self.can_carve(x, y, Direction::East) {
      dirs.push(Direction::East);
    }
    if self.can_carve(x, y, Direction::West) {
      dirs.push(Direction::West);
    }
    return dirs;
  }

  pub fn draw(&self, canvas: &mut Canvas) {
    for cell in &self.cells {
      self.draw_cell(canvas, *cell);
    }
  }

  fn draw_cell(&self, canvas: &mut Canvas, cell: Cell) {
    cell.draw(canvas);
  }
}
