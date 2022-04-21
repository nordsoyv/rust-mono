use eframe::egui::{Color32, Painter};

pub mod cell_connection;
pub mod hex_cell;
pub mod hex_grid;
pub mod square_cell;
pub mod square_grid;
pub mod tri_cell;
pub mod triangle_grid;

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum GridType {
  Square,
  Hex,
  Triangle,
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
  fn can_carve(&self, coord: CellCoord, dir: Direction) -> bool {
    if let Some(cell_coord) = self.get_cell_in_dir(coord, dir) {
      if let Some(cell) = self.get_cell(cell_coord) {
        return !cell.is_part_of_maze();
      }
    }

    return false;
  }
  fn get_cell_in_dir(&self, coord: CellCoord, dir: Direction) -> Option<CellCoord>;
  fn carve(&mut self, coord_start: CellCoord, dir: Direction);
  fn get_allowed_directions(&self, coord: CellCoord) -> Vec<Direction>;
  fn draw(&self, painter: &Painter);
  fn draw_background(&self, painter: &Painter);
  fn set_cell_size(&mut self, cell_size: i32);
  fn get_width(&self) -> f32;
  fn init(&mut self);
  fn get_size_in_pixels(&self) -> (f32, f32);
  fn get_num_cells_horizontal(&self) -> i32;
  fn get_num_cells_vertical(&self) -> i32;
  fn get_cell_size(&self) -> i32;
  fn get_margin(&self) -> i32;
  fn get_entrance(&self) -> CellCoord;
  fn get_exit(&self) -> CellCoord;
  fn has_solution(&self) -> bool;
  fn set_has_solution(&mut self, has_solution: bool);
  fn clear_solution(&mut self);
  fn find_dead_ends(&mut self);
  fn count_dead_ends(&self) -> usize;
  fn remove_dead_end(&mut self);
}
