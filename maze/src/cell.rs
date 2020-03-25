use crate::canvas::Canvas;
use crate::common::{CELL_HEIGHT, CELL_INSET, CELL_WIDTH};

#[derive(Clone, Copy, Debug)]
pub struct CellCoord {
  pub x_pos: i32,
  pub y_pos: i32,
}

impl CellCoord {
  #[allow(dead_code)]
  pub fn new(x: i32, y: i32) -> CellCoord {
    CellCoord {
      x_pos: x,
      y_pos: y,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
  pub left: Option<CellCoord>,
  pub right: Option<CellCoord>,
  pub top: Option<CellCoord>,
  pub bottom: Option<CellCoord>,
  pub coord: CellCoord,
  pub part_of_maze: bool,
  pub color: Option<u32>,
  pub distance: i32,
}

impl Cell {
  pub fn default(x: i32, y: i32) -> Cell {
    Cell {
      bottom: None,
      left: None,
      top: None,
      right: None,
      coord: CellCoord::new(x, y),
      part_of_maze: false,
      color: None,
      distance: -1,
    }
  }

  pub fn get_neighbours(&self) -> Vec<CellCoord> {
    let mut neighbours = vec![];
    if self.top.is_some() {
      neighbours.push(self.top.unwrap());
    }
    if self.bottom.is_some() {
      neighbours.push(self.bottom.unwrap());
    }
    if self.left.is_some() {
      neighbours.push(self.left.unwrap());
    }
    if self.right.is_some() {
      neighbours.push(self.right.unwrap());
    }
    neighbours
  }

  fn draw_background(&self, canvas: &mut Canvas) {
    let color;
    if self.distance > 0 {
      let d = (self.distance % 256) as u32;
      let d = 256 - d;
      let red = d << 16;
      let green = d << 8;
      let blue = d;
      color = red | green | blue;
    } else if self.color.is_some() {
      color = self.color.unwrap()
    } else {
      return;
    }
    canvas.fill_square(
      self.coord.x_pos * CELL_WIDTH + CELL_INSET,
      self.coord.y_pos * CELL_HEIGHT + CELL_INSET,
      CELL_WIDTH - CELL_INSET - CELL_INSET,
      CELL_HEIGHT - CELL_INSET - CELL_INSET,
      color);
    if CELL_INSET > 0 {
      if self.top.is_some() {
        canvas.fill_square(
          ((self.coord.x_pos) * CELL_WIDTH) + CELL_INSET,
          ((self.coord.y_pos + 1) * CELL_HEIGHT) - CELL_INSET,
          CELL_WIDTH - CELL_INSET - CELL_INSET,
          CELL_INSET,
          color);
      }
      if self.bottom.is_some() {
        canvas.fill_square(
          ((self.coord.x_pos) * CELL_WIDTH) + CELL_INSET,
          self.coord.y_pos
            * CELL_HEIGHT,
          CELL_WIDTH - CELL_INSET - CELL_INSET,
          CELL_INSET,
          color);
      }
      if self.left.is_some() {
        canvas.fill_square(
          self.coord.x_pos * CELL_WIDTH,
          ((self.coord.y_pos) * CELL_HEIGHT) + CELL_INSET,
          CELL_INSET,
          CELL_HEIGHT - CELL_INSET - CELL_INSET,
          color);
      }
      if self.right.is_some() {
        canvas.fill_square(
          ((self.coord.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
          ((self.coord.y_pos) * CELL_HEIGHT) + CELL_INSET,
          CELL_INSET,
          CELL_HEIGHT - CELL_INSET - CELL_INSET,
          color);
      }
    }
  }

  pub fn draw(&self, canvas: &mut Canvas) {
    self.draw_background(canvas);
    if self.top.is_none() {
      let y_pos = (self.coord.y_pos + 1) * CELL_HEIGHT;
      canvas.draw_horizontal_line((self.coord.x_pos * CELL_WIDTH) + CELL_INSET,
                                  (y_pos) - CELL_INSET,
                                  ((self.coord.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                  (y_pos) - CELL_INSET);
    }
    if self.bottom.is_none() {
      let y_pos = self.coord.y_pos * CELL_HEIGHT;
      canvas.draw_horizontal_line((self.coord.x_pos * CELL_WIDTH) + CELL_INSET,
                                  y_pos + CELL_INSET,
                                  ((self.coord.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                  y_pos + CELL_INSET);
    }
    if self.left.is_none() {
      let x_pos = self.coord.x_pos * CELL_WIDTH;
      canvas.draw_vertical_line(x_pos + CELL_INSET,
                                (self.coord.y_pos * CELL_HEIGHT) + CELL_INSET,
                                x_pos + CELL_INSET,
                                ((self.coord.y_pos + 1) * CELL_HEIGHT) - CELL_INSET)
    }
    if self.right.is_none() {
      let x_pos = (self.coord.x_pos + 1) * CELL_WIDTH;
      canvas.draw_vertical_line(x_pos - CELL_INSET,
                                (self.coord.y_pos * CELL_HEIGHT) + CELL_INSET,
                                x_pos - CELL_INSET,
                                ((self.coord.y_pos + 1) * CELL_HEIGHT) - CELL_INSET,
      );
    }

    if CELL_INSET > 0 {
      if self.top.is_some() {
        let y_pos = (self.coord.y_pos + 1) * CELL_HEIGHT;
        canvas.draw_vertical_line((self.coord.x_pos * CELL_WIDTH) + CELL_INSET,
                                  y_pos,
                                  (self.coord.x_pos * CELL_WIDTH) + CELL_INSET,
                                  (y_pos) - CELL_INSET);

        canvas.draw_vertical_line(((self.coord.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                  y_pos,
                                  ((self.coord.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                  (y_pos) - CELL_INSET);
      }
      if self.bottom.is_some() {
        let y_pos = (self.coord.y_pos) * CELL_HEIGHT;
        canvas.draw_vertical_line((self.coord.x_pos * CELL_WIDTH) + CELL_INSET,
                                  y_pos,
                                  (self.coord.x_pos * CELL_WIDTH) + CELL_INSET,
                                  (y_pos) + CELL_INSET);

        canvas.draw_vertical_line(((self.coord.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                  y_pos,
                                  ((self.coord.x_pos + 1) * CELL_WIDTH) - CELL_INSET,
                                  (y_pos) + CELL_INSET);
      }
      if self.left.is_some() {
        let x_pos = self.coord.x_pos * CELL_WIDTH;
        canvas.draw_horizontal_line(x_pos,
                                    (self.coord.y_pos * CELL_HEIGHT) + CELL_INSET,
                                    x_pos + CELL_INSET,
                                    (self.coord.y_pos * CELL_HEIGHT) + CELL_INSET);
        canvas.draw_horizontal_line(x_pos,
                                    ((self.coord.y_pos + 1) * CELL_HEIGHT) - CELL_INSET,
                                    x_pos + CELL_INSET,
                                    ((self.coord.y_pos + 1) * CELL_HEIGHT) - CELL_INSET);
      }
      if self.right.is_some() {
        let x_pos = (self.coord.x_pos + 1) * CELL_WIDTH;
        canvas.draw_horizontal_line(x_pos,
                                    (self.coord.y_pos * CELL_HEIGHT) + CELL_INSET,
                                    x_pos - CELL_INSET,
                                    (self.coord.y_pos * CELL_HEIGHT) + CELL_INSET);

        canvas.draw_horizontal_line(x_pos,
                                    ((self.coord.y_pos + 1) * CELL_HEIGHT) - CELL_INSET,
                                    x_pos - CELL_INSET,
                                    ((self.coord.y_pos + 1) * CELL_HEIGHT) - CELL_INSET);
      }
    }
  }
}
