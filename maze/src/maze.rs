use canvas::Canvas;

use crate::cell::{Cell, CellCoord};
use crate::common::Direction;

pub struct SquareGrid2D {
  pub cells: Vec<Cell>,
  pub width: i32,
  pub height: i32,
  pub cell_inset: i32,
  pub cell_width: i32,
  pub cell_height: i32,
}

impl SquareGrid2D {
  pub fn new(
    width: i32,
    height: i32,
    cell_width: i32,
    cell_height: i32,
    cell_inset: i32,
  ) -> SquareGrid2D {
    let mut cells = vec![];
    for y in 0..height {
      for x in 0..width {
        cells.push(Cell::default(x, y));
      }
    }
    SquareGrid2D {
      cells,
      width,
      height,
      cell_inset,
      cell_height,
      cell_width,
    }
  }

  pub fn get_cell(&self, coord: CellCoord) -> Option<&Cell> {
    let index = coord.y_pos * self.height + coord.x_pos;
    if (index as usize) < self.cells.len() {
      return Some(&self.cells[index as usize]);
    }
    return None;
  }

  pub fn get_mut_cell(&mut self, coord: CellCoord) -> Option<&mut Cell> {
    let index = (coord.y_pos * self.height) + coord.x_pos;
    if (index as usize) < self.cells.len() {
      return Some(&mut self.cells[index as usize]);
    }
    return None;
  }

  pub fn can_carve(&self, coord: CellCoord, dir: Direction) -> bool {
    let target_x = match dir {
      Direction::West => coord.x_pos - 1,
      Direction::East => coord.x_pos + 1,
      _ => coord.x_pos,
    };
    let target_y = match dir {
      Direction::South => coord.y_pos - 1,
      Direction::North => coord.y_pos + 1,
      _ => coord.y_pos,
    };

    if target_x < 0 || target_x >= self.width || target_y < 0 || target_y >= self.height {
      return false;
    }

    if let Some(target_cell) = self.get_cell(CellCoord::new(target_x, target_y)) {
      if !target_cell.part_of_maze {
        return true;
      }
    }

    return false;
  }

  pub fn get_cell_in_dir(&self, coord: CellCoord, dir: Direction) -> CellCoord {
    match dir {
      Direction::North => CellCoord::new(coord.x_pos, coord.y_pos + 1),
      Direction::South => CellCoord::new(coord.x_pos, coord.y_pos - 1),
      Direction::East => CellCoord::new(coord.x_pos + 1, coord.y_pos),
      Direction::West => CellCoord::new(coord.x_pos - 1, coord.y_pos),
    }
  }

  pub fn carve(&mut self, coord_start: CellCoord, dir: Direction) {
    let x_end = match dir {
      Direction::West => coord_start.x_pos - 1,
      Direction::East => coord_start.x_pos + 1,
      _ => coord_start.x_pos,
    };
    let y_end = match dir {
      Direction::South => coord_start.y_pos - 1,
      Direction::North => coord_start.y_pos + 1,
      _ => coord_start.y_pos,
    };
    let coord_end = CellCoord {
      x_pos: x_end,
      y_pos: y_end,
    };
    if coord_start.x_pos < 0
      || coord_end.x_pos < 0
      || coord_start.y_pos < 0
      || coord_end.y_pos < 0
      || coord_start.x_pos > self.width
      || coord_end.x_pos > self.width
      || coord_start.y_pos > self.height
      || coord_end.y_pos > self.height
    {
      return;
    }

    if let Some(start_cell) = self.get_mut_cell(coord_start) {
      start_cell.part_of_maze = true;
      match dir {
        Direction::North => {
          start_cell.top = Some(coord_end);
        }
        Direction::South => {
          start_cell.bottom = Some(coord_end);
        }
        Direction::East => {
          start_cell.right = Some(coord_end);
        }
        Direction::West => {
          start_cell.left = Some(coord_end);
        }
      }
    }

    if let Some(end_cell) = self.get_mut_cell(coord_end) {
      end_cell.part_of_maze = true;
      match dir {
        Direction::North => {
          end_cell.bottom = Some(coord_start);
        }
        Direction::South => {
          end_cell.top = Some(coord_start);
        }
        Direction::East => {
          end_cell.left = Some(coord_start);
        }
        Direction::West => {
          end_cell.right = Some(coord_start);
        }
      }
    }
  }

  pub fn get_allowed_directions(&self, coord: CellCoord) -> Vec<Direction> {
    let mut dirs = vec![];
    if self.can_carve(coord, Direction::North) {
      dirs.push(Direction::North);
    }
    if self.can_carve(coord, Direction::South) {
      dirs.push(Direction::South);
    }
    if self.can_carve(coord, Direction::East) {
      dirs.push(Direction::East);
    }
    if self.can_carve(coord, Direction::West) {
      dirs.push(Direction::West);
    }
    return dirs;
  }

  pub fn draw(&self, canvas: &mut Canvas) {
    for cell in &self.cells {
      self.draw_cell(canvas, *cell);
    }
  }

  fn draw_cell(&self, canvas: &mut Canvas, cell: Cell) {
    cell.draw(canvas, self.cell_inset, self.cell_width, self.cell_height);
  }
}
