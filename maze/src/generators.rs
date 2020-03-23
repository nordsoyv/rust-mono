pub mod growing_tree;

use crate::maze::SquareGrid2D;

pub trait Generator {
  fn init(&mut self, maze: &mut SquareGrid2D);
  fn generate(&mut self,  maze: &mut SquareGrid2D);
  fn generate_step(&mut self, maze: &mut SquareGrid2D);
  fn done(&self) -> bool;
}
