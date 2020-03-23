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
  pub color: Option<u32>,
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
      color: None,
    }
  }


  fn draw_background(&self, canvas: &mut Canvas) {
    let color = self.color.unwrap();
    canvas.fill_square(
      self.x_pos * CELL_WIDTH + CELL_INSET,
      self.y_pos * CELL_HEIGHT + CELL_INSET,
      CELL_WIDTH - CELL_INSET - CELL_INSET,
      CELL_HEIGHT - CELL_INSET - CELL_INSET,
      color);
    if CELL_INSET > 0 {
      if self.top != Wall::Wall {
        canvas.fill_square(
          ((self.x_pos) * CELL_WIDTH) + CELL_INSET,
          ((self.y_pos + 1) * CELL_HEIGHT) - CELL_INSET,
          CELL_WIDTH - CELL_INSET - CELL_INSET,
          CELL_INSET,
          color);
      }
      if self.bottom != Wall::Wall {
        canvas.fill_square(
          ((self.x_pos) * CELL_WIDTH) + CELL_INSET,
          self.y_pos * CELL_HEIGHT,
          CELL_WIDTH - CELL_INSET - CELL_INSET,
          CELL_INSET,
          color);
      }
      if self.left != Wall::Wall {
        canvas.fill_square(
          self.x_pos * CELL_WIDTH,
          ((self.y_pos) * CELL_HEIGHT) + CELL_INSET,
          CELL_INSET,
          CELL_HEIGHT - CELL_INSET - CELL_INSET,
          color);
      }
      if self.right != Wall::Wall {
        canvas.fill_square(
          ((self.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
          ((self.y_pos) * CELL_HEIGHT) + CELL_INSET,
          CELL_INSET,
          CELL_HEIGHT - CELL_INSET - CELL_INSET,
          color);
      }
    }
  }

  pub fn draw(&self, canvas: &mut Canvas) {
    if self.color.is_some() {
      self.draw_background(canvas);
    }


    if self.top == Wall::Wall {
      let y_pos = (self.y_pos + 1) * CELL_HEIGHT;
      canvas.draw_horizontal_line((self.x_pos * CELL_WIDTH) + CELL_INSET,
                                  (y_pos) - CELL_INSET,
                                  ((self.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                  (y_pos) - CELL_INSET);
    }
    if self.bottom == Wall::Wall {
      let y_pos = self.y_pos * CELL_HEIGHT;
      canvas.draw_horizontal_line((self.x_pos * CELL_WIDTH) + CELL_INSET,
                                  y_pos + CELL_INSET,
                                  ((self.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                  y_pos + CELL_INSET);
    }
    if self.left == Wall::Wall {
      let x_pos = self.x_pos * CELL_WIDTH;
      canvas.draw_vertical_line(x_pos + CELL_INSET,
                                (self.y_pos * CELL_HEIGHT) + CELL_INSET,
                                x_pos + CELL_INSET,
                                ((self.y_pos + 1) * CELL_HEIGHT) - CELL_INSET)
    }
    if self.right == Wall::Wall {
      let x_pos = (self.x_pos + 1) * CELL_WIDTH;
      canvas.draw_vertical_line(x_pos - CELL_INSET,
                                (self.y_pos * CELL_HEIGHT) + CELL_INSET,
                                x_pos - CELL_INSET,
                                ((self.y_pos + 1) * CELL_HEIGHT) - CELL_INSET,
      );
    }

    if CELL_INSET > 0 {
      if self.top != Wall::Wall {
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
      if self.bottom != Wall::Wall {
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
      if self.left != Wall::Wall {
        let x_pos = self.x_pos * CELL_WIDTH;
        canvas.draw_horizontal_line(x_pos,
                                    (self.y_pos * CELL_HEIGHT) + CELL_INSET,
                                    x_pos + CELL_INSET,
                                    (self.y_pos * CELL_HEIGHT) + CELL_INSET);
        canvas.draw_horizontal_line(x_pos,
                                    ((self.y_pos + 1) * CELL_HEIGHT) - CELL_INSET,
                                    x_pos + CELL_INSET,
                                    ((self.y_pos + 1) * CELL_HEIGHT) - CELL_INSET);
      }
      if self.right != Wall::Wall {
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
}
