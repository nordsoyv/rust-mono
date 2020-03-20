use crate::common::{Wall, CELL_WIDTH, CELL_HEIGHT, CELL_INSET};
use crate::canvas::Canvas;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
  pub left: Wall,
  pub right: Wall,
  pub top: Wall,
  pub bottom: Wall,
  pub x_pos: i32,
  pub y_pos: i32,
  pub part_of_maze: bool,
  pub active_cell: bool,
}

impl Cell {
  pub fn default(x: i32, y: i32) -> Cell {
    Cell {
      bottom: Wall::Wall,
      left: Wall::Wall,
      top: Wall::Wall,
      right: Wall::Wall,
      x_pos: x,
      y_pos: y,
      part_of_maze: false,
      active_cell: false,
    }
  }

  pub fn draw(&self, canvas: &mut Canvas) {
    if self.active_cell {
      canvas.fill_square(
        self.x_pos * CELL_WIDTH,
        self.y_pos * CELL_HEIGHT,
        CELL_WIDTH,
        CELL_HEIGHT,
        0xffffff00);
    }
    if self.top == Wall::Wall {
      let y_pos = (self.y_pos + 1) * CELL_HEIGHT;
      canvas.draw_horizontal_line((self.x_pos * CELL_WIDTH) + CELL_INSET,
                                  (y_pos) - CELL_INSET,
                                  ((self.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                  (y_pos) - CELL_INSET);
    } else {
      let y_pos = (self.y_pos + 1) * CELL_HEIGHT;
      canvas.draw_vertical_line((self.x_pos * CELL_WIDTH) + CELL_INSET,
                                y_pos,
                                (self.x_pos * CELL_WIDTH) + CELL_INSET,
                                (y_pos) - CELL_INSET);

      canvas.draw_vertical_line(((self.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                y_pos,
                                ((self.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                (y_pos) - CELL_INSET);
    }
    if self.bottom == Wall::Wall {
      let y_pos = ((self.y_pos) * CELL_HEIGHT);
      canvas.draw_horizontal_line((self.x_pos * CELL_WIDTH) + CELL_INSET,
                                  y_pos + CELL_INSET,
                                  ((self.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                  y_pos + CELL_INSET);
    } else {
      let y_pos = (self.y_pos) * CELL_HEIGHT;
      canvas.draw_vertical_line((self.x_pos * CELL_WIDTH) + CELL_INSET,
                                y_pos,
                                (self.x_pos * CELL_WIDTH) + CELL_INSET,
                                (y_pos) + CELL_INSET);

      canvas.draw_vertical_line(((self.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                y_pos,
                                ((self.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                (y_pos) + CELL_INSET);
    }
    if self.left == Wall::Wall {
      let x_pos = (self.x_pos * CELL_WIDTH);
      canvas.draw_vertical_line(x_pos + CELL_INSET,
                                (self.y_pos * CELL_HEIGHT) + CELL_INSET,
                                x_pos + CELL_INSET,
                                ((self.y_pos + 1) * CELL_HEIGHT) - CELL_INSET)
    } else {
      let x_pos = (self.x_pos * CELL_WIDTH);
      canvas.draw_horizontal_line(x_pos,
                                  (self.y_pos * CELL_HEIGHT) + CELL_INSET,
                                  x_pos + CELL_INSET,
                                  (self.y_pos * CELL_HEIGHT) + CELL_INSET);
      canvas.draw_horizontal_line(x_pos,
                                  ((self.y_pos + 1) * CELL_HEIGHT) - CELL_INSET,
                                  x_pos + CELL_INSET,
                                  ((self.y_pos + 1) * CELL_HEIGHT) - CELL_INSET);
    }
    if self.right == Wall::Wall {
      let x_pos = ((self.x_pos + 1) * CELL_WIDTH);
      canvas.draw_vertical_line(x_pos - CELL_INSET,
                                (self.y_pos * CELL_HEIGHT) + CELL_INSET,
                                x_pos - CELL_INSET,
                                ((self.y_pos + 1) * CELL_HEIGHT) - CELL_INSET,
      );
    } else {
      let x_pos = (self.x_pos + 1) * CELL_WIDTH;
      canvas.draw_horizontal_line(x_pos,
                                  (self.y_pos * CELL_HEIGHT) + CELL_INSET,
                                  x_pos - CELL_INSET,
                                  (self.y_pos * CELL_HEIGHT) + CELL_INSET);

      canvas.draw_horizontal_line(x_pos,
                                  ((self.y_pos + 1) * CELL_HEIGHT) - CELL_INSET,
                                  x_pos - CELL_INSET,
                                  ((self.y_pos + 1) * CELL_HEIGHT) - CELL_INSET);
    }
  }
}
