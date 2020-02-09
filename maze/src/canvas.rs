use std::convert::TryFrom;
use crate::common::{ MARGIN};

pub const BACKGROUND_COLOR: u32 = 0x00ffffff;
pub const FOREGROUND_COLOR: u32 = 0xff000000;


pub struct Canvas {
  pub width: i32,
  pub height: i32,
  pub buffer: Vec<u32>,
}

impl Canvas {
  pub fn clear(&mut self) {
    self.buffer = Vec::new();
    let size = usize::try_from(self.width * self.height).unwrap();
    self.buffer.resize(size, BACKGROUND_COLOR);
  }

  pub fn draw_vertical_line(&mut self, start_x: i32, start_y: i32, length: i32) {
    assert!(length > 0);
    assert!(start_x >= 0);
    assert!(start_y >= 0);
    let margin = MARGIN * self.width + MARGIN;
    let top_left = (start_y * self.width) + (start_x) + margin;
    for pos in 0..length {
      self.buffer[(top_left + (pos * self.width)) as usize] = FOREGROUND_COLOR;
    }
  }
  pub fn draw_horizontal_line(&mut self, start_x: i32, start_y: i32, length: i32) {
    assert!(length > 0);
    assert!(start_x >= 0);
    assert!(start_y >= 0);
    let margin = MARGIN * self.width + MARGIN;
    let top_left = (start_y * self.width) + (start_x) + margin;
    for pos in 0..length {
      self.buffer[(top_left + pos) as usize] = FOREGROUND_COLOR;
    }
  }

  pub fn fill_square(&mut self, start_x: i32, start_y: i32, width : i32, height : i32, color: u32) {
    assert!(start_x >= 0);
    assert!(start_y >= 0);
    assert!(width >=0);
    assert!(height >=0);

    let margin = MARGIN * self.width + MARGIN;
    let top_left = (start_y * self.width) + (start_x) + margin;

    for x_pos in 0..width {
      for y_pos in 0..height {
        let pos = top_left + (y_pos * self.width) + x_pos ;
        self.buffer[pos as usize] = color;
      }
    }

  }
}
