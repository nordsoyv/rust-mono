use std::convert::TryFrom;
use crate::common::{BACKGROUND_COLOR, FOREGROUND_COLOR};

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
    let margin = 5 * self.width + 5;
    let top_left = (start_y * self.width) + (start_x) + margin;
    for pos in 0..length {
      self.buffer[(top_left + (pos * self.width)) as usize] = FOREGROUND_COLOR;
    }
  }
  pub fn draw_horizontal_line(&mut self, start_x: i32, start_y: i32, length: i32) {
    assert!(length > 0);
    assert!(start_x >= 0);
    assert!(start_y >= 0);
    let margin = 5 * self.width + 5;
    let top_left = (start_y * self.width) + (start_x) + margin;
    for pos in 0..length {
      self.buffer[(top_left + pos) as usize] = FOREGROUND_COLOR;
    }
  }
}
