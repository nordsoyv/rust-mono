use crate::common::MARGIN;
use crate::generators::growing_tree::{GrowingTreeGenerator, Strategy};
use crate::generators::Generator;
use crate::maze::{HexGrid, SquareGrid2D};
use crate::{HEIGHT, WIDTH};
use std::fs::read_to_string;

pub struct AppState {
  pub generator: Box<dyn Generator>,
  pub grid: HexGrid,
  pub generate_steps: i32,
  cell_inset: i32,
  pub show_dist: bool,
  difficulty: i32,
  pub num_cells_width: i32,
  pub num_cells_height: i32,
  pub cell_width: i32,
  pub cell_height: i32,
}

impl AppState {
  pub fn new() -> AppState {
    AppState {
      generator: Box::new(GrowingTreeGenerator::new(Strategy::LastAndRandom(10))),
      // grid: SquareGrid2D::new(30, 30, 15, 15, 0),
      grid: HexGrid::new(10, 10, 10),
      generate_steps: 10,
      show_dist: false,
      cell_inset: 0,
      difficulty: 10,
      num_cells_height: 30,
      num_cells_width: 30,
      cell_width: 15,
      cell_height: 15,
    }
  }

  pub fn generate_maze(&mut self) {
    return;
    // if !self.generator.done() {
    //   for _ in 0..self.generate_steps {
    //     self.generator.generate_step(&mut self.grid);
    //   }
    // }
  }

  pub fn get_maze_size(&self) -> (i32, i32) {
    (
      (self.cell_width * self.num_cells_width) + (MARGIN * 2),
      (self.cell_height * self.num_cells_height) + (MARGIN * 2),
    )
  }

  pub fn get_title(&self) -> String {
    return format!(
      "Maze type: {} -- Difficulty: {} -- Generation speed: {} ",
      self.generator.name(),
      self.difficulty,
      self.generate_steps
    );
  }

  pub fn generate_new_maze(&mut self) {
    self.grid = HexGrid::new(self.num_cells_width, self.num_cells_height, self.cell_width);
    self.generator = Box::new(GrowingTreeGenerator::new(Strategy::LastN(self.difficulty)));
    // self.generator.init(&mut self.grid);
  }

  // pub fn inset_smaller(&mut self) {
  //   self.grid.cell_inset = self.grid.cell_inset - 1;
  //   if self.grid.cell_inset < 0 {
  //     self.grid.cell_inset = 0;
  //   }
  // }

  // pub fn inset_larger(&mut self) {
  //   self.grid.cell_inset = self.grid.cell_inset + 1;
  //   if self.grid.cell_inset > self.cell_width / 4 {
  //     self.grid.cell_inset = self.cell_width / 4;
  //   }
  // }

  pub fn generate_slower(&mut self) {
    self.generate_steps = self.generate_steps - 1;
    if self.generate_steps < 1 {
      self.generate_steps = 1
    }
  }
  pub fn generate_faster(&mut self) {
    self.generate_steps = self.generate_steps + 1;
    if self.generate_steps > 100 {
      self.generate_steps = 100
    }
  }

  pub fn toggle_show_distance(&mut self) {
    self.show_dist = !self.show_dist;
  }

  pub fn cell_size_smaller(&mut self) {
    self.cell_height -= 1;
    self.cell_width -= 1;
    if self.cell_width < 5 {
      self.cell_width = 5;
      self.cell_height = 5;
    }
    self.grid.cell_size = self.cell_height;
    // self.grid.cell_width = self.cell_width;
  }

  pub fn cell_size_larger(&mut self) {
    self.cell_height += 1;
    self.cell_width += 1;
    if self.get_maze_size().0 > WIDTH || self.get_maze_size().1 > HEIGHT {
      self.cell_height -= 1;
      self.cell_width -= 1;
    }
    self.grid.cell_size = self.cell_height;
    // self.grid.cell_width = self.cell_width;
  }

  pub fn num_cell_height_inc(&mut self) {
    self.num_cells_height += 2;
    if self.get_maze_size().1 > HEIGHT {
      self.num_cells_height -= 2;
    }
    self.generate_new_maze();
  }

  pub fn num_cell_height_dec(&mut self) {
    self.num_cells_height -= 2;
    if self.num_cells_height < 1 {
      self.num_cells_height = 2
    }
    self.generate_new_maze();
  }

  pub fn num_cell_width_inc(&mut self) {
    self.num_cells_width += 2;
    if self.get_maze_size().0 > WIDTH {
      self.num_cells_width -= 2;
    }
    self.generate_new_maze();
  }

  pub fn num_cell_width_dec(&mut self) {
    self.num_cells_width -= 2;
    if self.num_cells_width < 1 {
      self.num_cells_width = 2
    }
    self.generate_new_maze();
  }

  pub fn difficulty_harder(&mut self) {
    self.difficulty -= 1;
    if self.difficulty < 1 {
      self.difficulty = 1
    }
    self.generate_new_maze();
  }

  pub fn difficulty_easier(&mut self) {
    self.difficulty += 1;
    self.generate_new_maze();
  }
}
