use crate::common::get_random_usize;
use eframe::egui::Color32;

use crate::generators::Generator;
use crate::grids::{CellCoord, Grid};

// use crate::grid::types::{CellCoord, Grid};
pub const CELL_ACTIVE_COLOR: Color32 = Color32::from_rgb(255, 255, 0);
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
  strategy: Strategy,
}

impl Generator for GrowingTreeGenerator {
  fn init(&mut self, maze: &mut Box<dyn Grid>) {
    let start_cell = maze.get_entrance();

    maze.init();
    maze
      .get_mut_cell(start_cell)
      .unwrap()
      .set_part_of_maze(true);
    self.stack.push(start_cell);
  }

  fn generate(&mut self, maze: &mut Box<dyn Grid>) {
    while self.done == false {
      self.generate_step(maze);
    }
  }

  fn generate_step(&mut self, maze: &mut Box<dyn Grid>) {
    if self.stack.len() == 0 {
      self.done = true;
      return;
    }
    let next_cell_index = self.get_next_index();
    let coord = self.stack[next_cell_index];
    if let Some(cell) = maze.get_mut_cell(coord) {
      cell.set_color(Some(CELL_ACTIVE_COLOR));
    }

    let available_dirs = maze.get_allowed_directions(coord);
    if available_dirs.len() == 0 {
      if let Some(cell) = maze.get_mut_cell(coord) {
        cell.set_color(None);
      }
      self.stack.remove(next_cell_index);
      return;
    }
    let random_dir = get_random_usize(available_dirs.len());
    maze.carve(coord, available_dirs[random_dir]);
    let next_cell = maze.get_cell_in_dir(coord, available_dirs[random_dir]);
    self.stack.push(next_cell.unwrap());
  }

  fn done(&self) -> bool {
    return self.done;
  }

  fn name(&self) -> &str {
    "Growing Tree"
  }
}

impl GrowingTreeGenerator {
  pub fn new(strategy: Strategy) -> GrowingTreeGenerator {
    GrowingTreeGenerator {
      done: false,
      stack: vec![],
      strategy,
    }
  }

  fn get_next_index(&mut self) -> usize {
    match self.strategy {
      Strategy::Random => get_random_usize(self.stack.len()),
      Strategy::First => 0,
      Strategy::FirstN(num) => {
        let n = get_random_usize(num as usize);
        if n >= self.stack.len() {
          self.stack.len() - 1
        } else {
          n as usize
        }
      }
      Strategy::Last => self.stack.len() - 1,
      Strategy::LastN(num) => {
        let n = get_random_usize(num as usize);
        let index: i32 = self.stack.len() as i32 - 1 - n as i32;
        if index < 0 {
          0
        } else {
          index as usize
        }
      }
      Strategy::LastAndRandom(num) => {
        let n = get_random_usize(num as usize);
        if n == 0 {
          // pick a random
          get_random_usize(self.stack.len())
        } else {
          self.stack.len() - 1
        }
      }
    }
  }
}
