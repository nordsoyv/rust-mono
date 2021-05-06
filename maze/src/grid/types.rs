use crate::common::Direction;
use canvas::Canvas;

pub trait Grid {
  fn get_cell(&self, coord: CellCoord) -> Option<&dyn Cell>;
  fn get_mut_cell(&mut self, coord: CellCoord) -> Option<&mut dyn Cell>;
  fn can_carve(&self, coord: CellCoord, dir: Direction) -> bool;
  fn get_cell_in_dir(&self, coord: CellCoord, dir: Direction) -> Option<CellCoord>;
  fn carve(&mut self, coord_start: CellCoord, dir: Direction);
  fn get_allowed_directions(&self, coord: CellCoord) -> Vec<Direction>;
  fn draw(&self, canvas: &mut Canvas);
  fn set_cell_size(&mut self, cell_size: i32);
  fn get_width(&self) -> i32;
  fn init(&mut self);
  fn get_size_in_pixels(&self) -> (i32, i32);
}

#[derive(Clone, Copy, Debug)]
pub struct CellCoord {
  pub x_pos: i32,
  pub y_pos: i32,
}

impl CellCoord {
  #[allow(dead_code)]
  pub fn new(x: i32, y: i32) -> CellCoord {
    CellCoord { x_pos: x, y_pos: y }
  }
}

pub trait Cell {
  fn get_coord(&self) -> CellCoord;
  fn is_part_of_maze(&self) -> bool;
  fn set_part_of_maze(&mut self, part: bool);
  fn set_color(&mut self, color: Option<u32>);
  fn get_distance(&self) -> i32;
  fn set_distance(&mut self, dist: i32);
  fn get_neighbours(&self) -> Vec<CellCoord>;
  // part_of_maze: bool;
  // color: Option<u32>,
  // distance: i32,
}
