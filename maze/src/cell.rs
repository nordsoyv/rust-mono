use canvas::Canvas;

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

  fn draw_background(
    &self,
    canvas: &mut Canvas,
    cell_inset: i32,
    cell_width: i32,
    cell_height: i32,
  ) {
    let color;
    if self.distance > 0 {
      let dist = (self.distance % (256 * 3)) as u32;
      let part = dist as f32 / 3.0;
      let blue = part as u32;
      let remain = dist - blue;
      let part = remain as f32 / 2.0;
      let green = part as u32;
      let remain = remain - green;
      let red = remain;

      dbg!(dist, red, green, blue);
      let red = red << 16;
      let green = green << 8;

      color = red | green | blue;
    } else if self.color.is_some() {
      color = self.color.unwrap()
    } else {
      return;
    }
    canvas.fill_square(
      self.coord.x_pos * cell_width + cell_inset,
      self.coord.y_pos * cell_height + cell_inset,
      cell_width - cell_inset - cell_inset,
      cell_height - cell_inset - cell_inset,
      color,
    );
    if cell_inset > 0 {
      if self.top.is_some() {
        canvas.fill_square(
          ((self.coord.x_pos) * cell_width) + cell_inset,
          ((self.coord.y_pos + 1) * cell_height) - cell_inset,
          cell_width - cell_inset - cell_inset,
          cell_inset,
          color,
        );
      }
      if self.bottom.is_some() {
        canvas.fill_square(
          ((self.coord.x_pos) * cell_width) + cell_inset,
          self.coord.y_pos * cell_height,
          cell_width - cell_inset - cell_inset,
          cell_inset,
          color,
        );
      }
      if self.left.is_some() {
        canvas.fill_square(
          self.coord.x_pos * cell_width,
          ((self.coord.y_pos) * cell_height) + cell_inset,
          cell_inset,
          cell_height - cell_inset - cell_inset,
          color,
        );
      }
      if self.right.is_some() {
        canvas.fill_square(
          ((self.coord.x_pos + 1) * cell_width) - cell_inset,
          ((self.coord.y_pos) * cell_height) + cell_inset,
          cell_inset,
          cell_height - cell_inset - cell_inset,
          color,
        );
      }
    }
  }

  pub fn draw(&self, canvas: &mut Canvas, cell_inset: i32, cell_width: i32, cell_height: i32) {
    self.draw_background(canvas, cell_inset, cell_width, cell_height);
    if self.top.is_none() {
      let y_pos = (self.coord.y_pos + 1) * cell_height;
      canvas.draw_horizontal_line(
        (self.coord.x_pos * cell_width) + cell_inset,
        (y_pos) - cell_inset,
        ((self.coord.x_pos + 1) * cell_width) - cell_inset,
        (y_pos) - cell_inset,
      );
    }
    if self.bottom.is_none() {
      let y_pos = self.coord.y_pos * cell_height;
      canvas.draw_horizontal_line(
        (self.coord.x_pos * cell_width) + cell_inset,
        y_pos + cell_inset,
        ((self.coord.x_pos + 1) * cell_width) - cell_inset,
        y_pos + cell_inset,
      );
    }
    if self.left.is_none() {
      let x_pos = self.coord.x_pos * cell_width;
      canvas.draw_vertical_line(
        x_pos + cell_inset,
        (self.coord.y_pos * cell_height) + cell_inset,
        x_pos + cell_inset,
        ((self.coord.y_pos + 1) * cell_height) - cell_inset,
      )
    }
    if self.right.is_none() {
      let x_pos = (self.coord.x_pos + 1) * cell_width;
      canvas.draw_vertical_line(
        x_pos - cell_inset,
        (self.coord.y_pos * cell_height) + cell_inset,
        x_pos - cell_inset,
        ((self.coord.y_pos + 1) * cell_height) - cell_inset,
      );
    }

    if cell_inset > 0 {
      if self.top.is_some() {
        let y_pos = (self.coord.y_pos + 1) * cell_height;
        canvas.draw_vertical_line(
          (self.coord.x_pos * cell_width) + cell_inset,
          y_pos,
          (self.coord.x_pos * cell_width) + cell_inset,
          (y_pos) - cell_inset,
        );

        canvas.draw_vertical_line(
          ((self.coord.x_pos + 1) * cell_width) - cell_inset,
          y_pos,
          ((self.coord.x_pos + 1) * cell_width) - cell_inset,
          (y_pos) - cell_inset,
        );
      }
      if self.bottom.is_some() {
        let y_pos = (self.coord.y_pos) * cell_height;
        canvas.draw_vertical_line(
          (self.coord.x_pos * cell_width) + cell_inset,
          y_pos,
          (self.coord.x_pos * cell_width) + cell_inset,
          (y_pos) + cell_inset,
        );

        canvas.draw_vertical_line(
          ((self.coord.x_pos + 1) * cell_width) - cell_inset,
          y_pos,
          ((self.coord.x_pos + 1) * cell_width) - cell_inset,
          (y_pos) + cell_inset,
        );
      }
      if self.left.is_some() {
        let x_pos = self.coord.x_pos * cell_width;
        canvas.draw_horizontal_line(
          x_pos,
          (self.coord.y_pos * cell_height) + cell_inset,
          x_pos + cell_inset,
          (self.coord.y_pos * cell_height) + cell_inset,
        );
        canvas.draw_horizontal_line(
          x_pos,
          ((self.coord.y_pos + 1) * cell_height) - cell_inset,
          x_pos + cell_inset,
          ((self.coord.y_pos + 1) * cell_height) - cell_inset,
        );
      }
      if self.right.is_some() {
        let x_pos = (self.coord.x_pos + 1) * cell_width;
        canvas.draw_horizontal_line(
          x_pos,
          (self.coord.y_pos * cell_height) + cell_inset,
          x_pos - cell_inset,
          (self.coord.y_pos * cell_height) + cell_inset,
        );

        canvas.draw_horizontal_line(
          x_pos,
          ((self.coord.y_pos + 1) * cell_height) - cell_inset,
          x_pos - cell_inset,
          ((self.coord.y_pos + 1) * cell_height) - cell_inset,
        );
      }
    }
  }
}
