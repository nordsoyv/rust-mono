use eframe::egui::{Color32, Context, Rect};

use crate::Pos2;

pub trait UiComponent {
  fn draw(&mut self, ctx: &Context);
}

#[derive(Clone, Copy, Debug)]
pub struct CellCoord {
  pub x_pos: f32,
  pub y_pos: f32,
}

impl CellCoord {
  #[allow(dead_code)]
  pub fn new(x: f32, y: f32) -> CellCoord {
    CellCoord { x_pos: x, y_pos: y }
  }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
  North,
  NorthEast,
  NorthWest,
  East,
  South,
  SouthEast,
  SouthWest,
  West,
}

pub trait Cell {
  fn get_coord(&self) -> CellCoord;
  fn is_part_of_maze(&self) -> bool;
  fn set_part_of_maze(&mut self, part: bool);
  fn set_color(&mut self, color: Option<Color32>);
  fn get_distance(&self) -> i32;
  fn set_distance(&mut self, dist: i32);
  fn get_neighbours(&self) -> Vec<CellCoord>;
  // part_of_maze: bool;
  // color: Option<u32>,
  // distance: i32,
}

pub trait Grid {
  fn get_cell(&self, coord: CellCoord) -> Option<&dyn Cell>;
  fn get_mut_cell(&mut self, coord: CellCoord) -> Option<&mut dyn Cell>;
  fn can_carve(&self, coord: CellCoord, dir: Direction) -> bool;
  fn get_cell_in_dir(&self, coord: CellCoord, dir: Direction) -> Option<CellCoord>;
  fn carve(&mut self, coord_start: CellCoord, dir: Direction);
  fn get_allowed_directions(&self, coord: CellCoord) -> Vec<Direction>;
  fn draw(&self) -> Vec<(Pos2, Pos2)>;
  fn draw_background(&self) -> Vec<(Rect, Color32)>;
  fn set_cell_size(&mut self, cell_size: i32);
  fn get_width(&self) -> f32;
  fn init(&mut self);
  fn get_size_in_pixels(&self) -> (f32, f32);
  fn get_num_cells_horizontal(&self) -> i32;
  fn get_num_cells_vertical(&self) -> i32;
  fn get_cell_size(&self) -> i32;
  fn get_margin(&self) -> i32;
}

#[derive(PartialEq, Debug)]
pub enum GridType {
  Square,
  Hex,
}
