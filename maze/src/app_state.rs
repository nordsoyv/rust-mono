use crate::common::MARGIN;
use crate::generators::growing_tree::{GrowingTreeGenerator, Strategy};
use crate::generators::Generator;
use crate::grid::hex_grid::HexGrid;
use crate::grid::types::Grid;
use crate::{HEIGHT, WIDTH};

pub struct AppState {
  pub generator: Box<dyn Generator>,
  pub grid: Box<dyn Grid>,
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
      grid: Box::new(HexGrid::new(10, 10, 20)),
      generate_steps: 10,
      show_dist: false,
      cell_inset: 0,
      difficulty: 10,
      num_cells_height: 10,
      num_cells_width: 10,
      cell_width: 20,
      cell_height: 20,
    }
  }

  pub fn generate_maze(&mut self) {
    if !self.generator.done() {
      for _ in 0..self.generate_steps {
        self.generator.generate_step(&mut self.grid);
      }
    }
  }

  pub fn get_maze_size(&self) -> (i32, i32) {
    let grid_size = self.grid.get_size_in_pixels();
    return (grid_size.0 + (MARGIN * 2), grid_size.1 + (MARGIN * 2));
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
    self.grid = Box::new(HexGrid::new(
      self.num_cells_width,
      self.num_cells_height,
      self.cell_width,
    ));
    self.grid.init();
    self.generator = Box::new(GrowingTreeGenerator::new(Strategy::LastN(self.difficulty)));
    self.generator.init(&mut self.grid);
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
    self.grid.set_cell_size(self.cell_height);
    // self.grid.cell_width = self.cell_width;
  }

  pub fn cell_size_larger(&mut self) {
    self.cell_height += 1;
    self.cell_width += 1;
    if self.get_maze_size().0 > WIDTH || self.get_maze_size().1 > HEIGHT {
      self.cell_height -= 1;
      self.cell_width -= 1;
    }
    self.grid.set_cell_size(self.cell_height);
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
