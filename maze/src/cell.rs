use canvas::Canvas;

use crate::save_image;

fn is_odd(num: i32) -> bool {
  return num & 1 != 0;
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

#[derive(Clone, Copy, Debug)]
pub struct SquareCell {
  pub left: Option<CellCoord>,
  pub right: Option<CellCoord>,
  pub top: Option<CellCoord>,
  pub bottom: Option<CellCoord>,
  pub coord: CellCoord,
  pub part_of_maze: bool,
  pub color: Option<u32>,
  pub distance: i32,
}

impl SquareCell {
  pub fn default(x: i32, y: i32) -> SquareCell {
    SquareCell {
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

      // dbg!(dist, red, green, blue);
      let red = red << 16;
      let green = green << 8;

      color = red | green | blue;
    } else if self.color.is_some() {
      color = self.color.unwrap()
    } else {
      return;
    }
    canvas.set_fg_color(color);
    canvas.fill_square(
      self.coord.x_pos * cell_width + cell_inset,
      self.coord.y_pos * cell_height + cell_inset,
      cell_width - cell_inset - cell_inset,
      cell_height - cell_inset - cell_inset,
    );
    if cell_inset > 0 {
      if self.top.is_some() {
        canvas.fill_square(
          ((self.coord.x_pos) * cell_width) + cell_inset,
          ((self.coord.y_pos + 1) * cell_height) - cell_inset,
          cell_width - cell_inset - cell_inset,
          cell_inset,
        );
      }
      if self.bottom.is_some() {
        canvas.fill_square(
          ((self.coord.x_pos) * cell_width) + cell_inset,
          self.coord.y_pos * cell_height,
          cell_width - cell_inset - cell_inset,
          cell_inset,
        );
      }
      if self.left.is_some() {
        canvas.fill_square(
          self.coord.x_pos * cell_width,
          ((self.coord.y_pos) * cell_height) + cell_inset,
          cell_inset,
          cell_height - cell_inset - cell_inset,
        );
      }
      if self.right.is_some() {
        canvas.fill_square(
          ((self.coord.x_pos + 1) * cell_width) - cell_inset,
          ((self.coord.y_pos) * cell_height) + cell_inset,
          cell_inset,
          cell_height - cell_inset - cell_inset,
        );
      }
    }
  }

  pub fn draw(&self, canvas: &mut Canvas, cell_inset: i32, cell_width: i32, cell_height: i32) {
    self.draw_background(canvas, cell_inset, cell_width, cell_height);
    canvas.set_fg_color(0x00000000);
    if self.top.is_none() {
      let y_pos = (self.coord.y_pos + 1) * cell_height;
      canvas.draw_line(
        (self.coord.x_pos * cell_width) + cell_inset,
        (y_pos) - cell_inset,
        ((self.coord.x_pos + 1) * cell_width) - cell_inset,
        (y_pos) - cell_inset,
      );
    }
    if self.bottom.is_none() {
      let y_pos = self.coord.y_pos * cell_height;
      canvas.draw_line(
        (self.coord.x_pos * cell_width) + cell_inset,
        y_pos + cell_inset,
        ((self.coord.x_pos + 1) * cell_width) - cell_inset,
        y_pos + cell_inset,
      );
    }
    if self.left.is_none() {
      let x_pos = self.coord.x_pos * cell_width;
      canvas.draw_line(
        x_pos + cell_inset,
        (self.coord.y_pos * cell_height) + cell_inset,
        x_pos + cell_inset,
        ((self.coord.y_pos + 1) * cell_height) - cell_inset,
      )
    }
    if self.right.is_none() {
      let x_pos = (self.coord.x_pos + 1) * cell_width;
      canvas.draw_line(
        x_pos - cell_inset,
        (self.coord.y_pos * cell_height) + cell_inset,
        x_pos - cell_inset,
        ((self.coord.y_pos + 1) * cell_height) - cell_inset,
      );
    }

    if cell_inset > 0 {
      if self.top.is_some() {
        let y_pos = (self.coord.y_pos + 1) * cell_height;
        canvas.draw_line(
          (self.coord.x_pos * cell_width) + cell_inset,
          y_pos,
          (self.coord.x_pos * cell_width) + cell_inset,
          (y_pos) - cell_inset,
        );

        canvas.draw_line(
          ((self.coord.x_pos + 1) * cell_width) - cell_inset,
          y_pos,
          ((self.coord.x_pos + 1) * cell_width) - cell_inset,
          (y_pos) - cell_inset,
        );
      }
      if self.bottom.is_some() {
        let y_pos = (self.coord.y_pos) * cell_height;
        canvas.draw_line(
          (self.coord.x_pos * cell_width) + cell_inset,
          y_pos,
          (self.coord.x_pos * cell_width) + cell_inset,
          (y_pos) + cell_inset,
        );

        canvas.draw_line(
          ((self.coord.x_pos + 1) * cell_width) - cell_inset,
          y_pos,
          ((self.coord.x_pos + 1) * cell_width) - cell_inset,
          (y_pos) + cell_inset,
        );
      }
      if self.left.is_some() {
        let x_pos = self.coord.x_pos * cell_width;
        canvas.draw_line(
          x_pos,
          (self.coord.y_pos * cell_height) + cell_inset,
          x_pos + cell_inset,
          (self.coord.y_pos * cell_height) + cell_inset,
        );
        canvas.draw_line(
          x_pos,
          ((self.coord.y_pos + 1) * cell_height) - cell_inset,
          x_pos + cell_inset,
          ((self.coord.y_pos + 1) * cell_height) - cell_inset,
        );
      }
      if self.right.is_some() {
        let x_pos = (self.coord.x_pos + 1) * cell_width;
        canvas.draw_line(
          x_pos,
          (self.coord.y_pos * cell_height) + cell_inset,
          x_pos - cell_inset,
          (self.coord.y_pos * cell_height) + cell_inset,
        );

        canvas.draw_line(
          x_pos,
          ((self.coord.y_pos + 1) * cell_height) - cell_inset,
          x_pos - cell_inset,
          ((self.coord.y_pos + 1) * cell_height) - cell_inset,
        );
      }
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct HexCell {
  pub north: Option<CellCoord>,
  pub south: Option<CellCoord>,
  pub north_east: Option<CellCoord>,
  pub north_west: Option<CellCoord>,
  pub south_east: Option<CellCoord>,
  pub south_west: Option<CellCoord>,

  pub coord: CellCoord,
  pub part_of_maze: bool,
  pub color: Option<u32>,
  pub distance: i32,
}

impl HexCell {
  pub fn default(x: i32, y: i32) -> HexCell {
    HexCell {
      north: None,
      north_east: None,
      north_west: None,
      south: None,
      south_east: None,
      south_west: None,
      coord: CellCoord::new(x, y),
      part_of_maze: false,
      color: None,
      distance: -1,
    }
  }
  pub fn get_neighbours(&self) -> Vec<CellCoord> {
    let mut neighbours = vec![];
    if self.north.is_some() {
      neighbours.push(self.north.unwrap());
    }
    if self.north_west.is_some() {
      neighbours.push(self.north_west.unwrap());
    }
    if self.north_east.is_some() {
      neighbours.push(self.north_east.unwrap());
    }
    if self.south.is_some() {
      neighbours.push(self.south.unwrap());
    }
    if self.south_west.is_some() {
      neighbours.push(self.south_west.unwrap());
    }
    if self.south_east.is_some() {
      neighbours.push(self.south_east.unwrap());
    }
    neighbours
  }

  pub fn draw(&self, canvas: &mut Canvas, size: f32) {
    canvas.set_fg_color(0);
    let a_size: f32 = size / 2.0;
    let b_size: f32 = (size * 3.0_f32.sqrt()) / 2.0;
    let width = size * 2.0;
    let height = b_size * 2.0;

    let cx = size + 3.0 * self.coord.x_pos as f32 * a_size;
    let mut cy = b_size + self.coord.y_pos as f32 * height;
    if is_odd(self.coord.x_pos) {
      cy += b_size;
    }

    // f/n = far/near
    // n/s/e/w = north/south/east/west
    let x_fw = (cx - size);
    let x_nw = cx - a_size;
    let x_ne = cx + a_size;
    let x_fe = cx + size;

    let y_n = cy - b_size;
    let y_m = cy;
    let y_s = cy + b_size;

    if self.north.is_none() {
      canvas.draw_line(x_nw as i32, y_n as i32, x_ne as i32, y_n as i32);
    }
    if self.north_east.is_none() {
      canvas.draw_line(x_ne as i32, y_n as i32, x_fe as i32, y_m as i32);
    }
    if self.south_east.is_none() {
      canvas.draw_line(x_fe as i32, y_m as i32, x_ne as i32, y_s as i32);
    }
    if self.south.is_none() {
      canvas.draw_line(x_ne as i32, y_s as i32, x_nw as i32, y_s as i32);
    }
    if self.south_west.is_none() {
      canvas.draw_line(x_fw as i32, y_m as i32, x_nw as i32, y_s as i32);
    }
    if self.north_west.is_none() {
      canvas.draw_line(x_fw as i32, y_m as i32, x_nw as i32, y_n as i32);
    }
  }
}
