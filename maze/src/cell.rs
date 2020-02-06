use crate::common::{Wall, CELL_WIDTH, CELL_HEIGHT};
use crate::canvas::Canvas;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
  pub left: Wall,
  pub right: Wall,
  pub top: Wall,
  pub bottom: Wall,
  pub x_pos: i32,
  pub y_pos: i32,
  pub part_of_maze : bool,
}

impl Cell {
  pub fn draw(&self, canvas: &mut Canvas) {
    if self.top == Wall::Wall {
      canvas.draw_horizontal_line(self.x_pos * CELL_WIDTH, self.y_pos * CELL_HEIGHT, CELL_WIDTH);
    }
    if self.bottom == Wall::Wall {
      canvas.draw_horizontal_line(self.x_pos * CELL_WIDTH, (self.y_pos + 1) * CELL_HEIGHT, CELL_WIDTH);
    }
    if self.left == Wall::Wall {
      canvas.draw_vertical_line(self.x_pos * CELL_WIDTH, self.y_pos * CELL_HEIGHT, CELL_HEIGHT);
    }
    if self.right == Wall::Wall {
      canvas.draw_vertical_line((self.x_pos + 1) * CELL_WIDTH, self.y_pos * CELL_HEIGHT, CELL_HEIGHT);
    }
  }
}
