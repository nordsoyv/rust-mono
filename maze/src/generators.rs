pub mod growing_tree;

use crate::maze::Maze;

pub trait Generator {
  fn init(&mut self, maze: &mut Maze);
  fn generate(&mut self,  maze: &mut Maze);
  fn generate_step(&mut self, maze: &mut Maze);
  fn done(&self) -> bool;
}
