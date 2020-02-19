use crate::canvas::Canvas;
use crate::cell::Cell;
use crate::common::{Direction, Wall};

pub struct Maze {
  cells: Vec<Cell>,
  pub width: i32,
  pub height: i32,
}

impl Maze {
  pub fn new(width: i32, height: i32) -> Maze {
    let mut cells = vec![];
    for y in 0..height {
      for x in 0..width {
        cells.push(Cell::default(x, y));
      }
    }
    Maze {
      cells,
      width,
      height,
    }
  }

  fn get_cell(&self, x: i32, y: i32) -> &Cell {
    let index = (y * self.height) + x;
    return &self.cells[index as usize];
  }

  pub fn get_mut_cell(&mut self, x: i32, y: i32) -> &mut Cell {
    let index = y * self.height + x;
    return &mut self.cells[index as usize];
  }

  pub fn can_carve(&self, x: i32, y: i32, dir: Direction) -> bool {
    let target_x = match dir {
      Direction::West => x - 1,
      Direction::East => x + 1,
      _ => x
    };
    let target_y = match dir {
      Direction::South => y + 1,
      Direction::North => y - 1,
      _ => y
    };

    if target_x < 0 || target_x >= self.width || target_y < 0 || target_y >= self.height {
      return false;
    }

    let target_cell = self.get_cell(target_x, target_y);
    if !target_cell.part_of_maze {
      return true;
    }
    return false;
  }

  pub fn get_cell_in_dir(&self, x: i32, y: i32, dir: Direction) -> (i32, i32) {
    match dir {
      Direction::North => (x, y - 1),
      Direction::South => (x, y + 1),
      Direction::East => (x + 1, y),
      Direction::West => (x - 1, y),
    }
  }

  pub fn carve(&mut self, x_start: i32, y_start: i32, dir: Direction) {
    let x_end = match dir {
      Direction::West => x_start - 1,
      Direction::East => x_start + 1,
      _ => x_start
    };
    let y_end = match dir {
      Direction::South => y_start + 1,
      Direction::North => y_start - 1,
      _ => y_start
    };
    if x_start < 0 || x_end < 0
      || y_start < 0 || y_end < 0
      || x_start > self.width || x_end > self.width
      || y_start > self.height || y_end > self.height {
      return;
    }
    {
      let start_cell = self.get_mut_cell(x_start, y_start);
      start_cell.part_of_maze = true;
      match dir {
        Direction::North => {
          start_cell.top = Wall::None;
        }
        Direction::South => {
          start_cell.bottom = Wall::None;
        }
        Direction::East => {
          start_cell.right = Wall::None;
        }
        Direction::West => {
          start_cell.left = Wall::None;
        }
      }
    }
    {
      let end_cell = self.get_mut_cell(x_end, y_end);
      end_cell.part_of_maze = true;
      match dir {
        Direction::North => {
          end_cell.bottom = Wall::None;
        }
        Direction::South => {
          end_cell.top = Wall::None;
        }
        Direction::East => {
          end_cell.left = Wall::None;
        }
        Direction::West => {
          end_cell.right = Wall::None;
        }
      }
    }
  }


  pub fn get_allowed_directions(&self, x: i32, y: i32) -> Vec<Direction> {
    let mut dirs = vec![];
    if self.can_carve(x, y, Direction::North) {
      dirs.push(Direction::North);
    }
    if self.can_carve(x, y, Direction::South) {
      dirs.push(Direction::South);
    }
    if self.can_carve(x, y, Direction::East) {
      dirs.push(Direction::East);
    }
    if self.can_carve(x, y, Direction::West) {
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
    cell.draw(canvas);
  }
}
