use crate::Grid;
pub mod growing_tree;

pub trait Generator {
  fn init(&mut self, maze: &mut Box<dyn Grid>);
  fn generate(&mut self, maze: &mut Box<dyn Grid>);
  fn generate_step(&mut self, maze: &mut Box<dyn Grid>);
  fn done(&self) -> bool;
  fn name(&self) -> &str;
}
