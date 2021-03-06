use crate::maze::SquareGrid2D;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use crate::common::{ CELL_ACTIVE_COLOR};
use crate::generators::Generator;
use crate::cell::CellCoord;

#[allow(dead_code)]
pub enum Strategy {
  Last,
  First,
  FirstN(i32),
  Random,
  LastN(i32),
  LastAndRandom(i32),
}

pub struct GrowingTreeGenerator {
  pub done: bool,
  stack: Vec<CellCoord>,
  rng: ThreadRng,
  random: Uniform<f32>,
  strategy: Strategy,
}

impl Generator for GrowingTreeGenerator {
  fn init(&mut self, maze: &mut SquareGrid2D) {
    maze.get_mut_cell( CellCoord {x_pos: maze.width / 2 , y_pos: 0}).bottom = Some(CellCoord{x_pos:-1,y_pos:-1});
    maze.get_mut_cell(  CellCoord {x_pos:maze.width / 2 , y_pos: maze.height - 1}).top = Some(CellCoord{x_pos:-1,y_pos:-1});
    self.stack.push(  CellCoord {x_pos:maze.width / 2 , y_pos: maze.height / 2});
    maze.get_mut_cell(CellCoord {x_pos:maze.width / 2 , y_pos: maze.height / 2}).part_of_maze = true;
  }

  fn name(&self) -> &str {
    "Growing Tree"
  }

  fn generate(&mut self, maze: &mut SquareGrid2D) {
    while self.done == false {
      self.generate_step(maze);
    }
  }

  fn generate_step(&mut self, maze: &mut SquareGrid2D) {
    if self.stack.len() == 0 {
      self.done = true;
      return;
    }
    let next_cell_index = self.get_next_index();
    let coord = self.stack[next_cell_index];
    maze.get_mut_cell(coord).color = Some(CELL_ACTIVE_COLOR);

    let available_dirs = maze.get_allowed_directions(coord);
    if available_dirs.len() == 0 {
      maze.get_mut_cell(coord).color = None;
      self.stack.remove(next_cell_index);
      return;
    }
    let random_dir = self.get_random(available_dirs.len());
    maze.carve(coord ,available_dirs[random_dir]);
    let next_cell = maze.get_cell_in_dir(coord, available_dirs[random_dir]);
    self.stack.push(next_cell);
  }

  fn done(&self) -> bool {
    return self.done;
  }
}

impl GrowingTreeGenerator {
  pub fn new(strategy: Strategy) -> GrowingTreeGenerator {
    GrowingTreeGenerator {
      done: false,
      stack: vec![],
      rng: rand::thread_rng(),
      random: Uniform::from(0f32..1f32),
      strategy,
    }
  }

  fn get_next_index(&mut self) -> usize {
    match self.strategy {
      Strategy::Random => self.get_random(self.stack.len()),
      Strategy::First => 0,
      Strategy::FirstN(num) => {
        let n = self.get_random(num as usize);
        if n >= self.stack.len() {
          self.stack.len() - 1
        } else {
          n as usize
        }
      }
      Strategy::Last => self.stack.len() - 1,
      Strategy::LastN(num) => {
        let n = self.get_random(num as usize);
        let index: i32 = self.stack.len() as i32 - 1 - n as i32;
        if index < 0 {
          0
        } else {
          index as usize
        }
      }
      Strategy::LastAndRandom(num) => {
        let n = self.get_random(num as usize);
        if n == 0 { // pick a random
          self.get_random(self.stack.len())
        } else {
          self.stack.len() - 1
        }
      }
    }
  }

  fn get_random(&mut self, max: usize) -> usize {
    let d = self.random.sample(&mut self.rng);
    let scaled = d * max as f32;
    let scaled_int = scaled as usize;
    return scaled_int;
  }
}