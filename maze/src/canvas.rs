use std::convert::TryFrom;
use crate::common::MARGIN;

pub const BACKGROUND_COLOR: u32 = 0x00ffffff;
pub const FOREGROUND_COLOR: u32 = 0xff000000;


pub struct Canvas {
  pub width: i32,
  pub height: i32,
  pub offset:i32,
  pub buffer: Vec<u32>,
}


impl Canvas {
  pub fn clear(&mut self) {
    self.buffer = Vec::new();
    let size = usize::try_from((self.width + 1) * (self.height + 1)).unwrap();
    self.buffer.resize(size, BACKGROUND_COLOR);
  }

  pub fn set_offset(&mut self, offset : i32){
    self.offset = offset;
  }

  fn normalize_coords(&self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> (i32, i32, i32, i32) {
    let (start_y, end_y) = if start_y > end_y {
      (end_y, start_y)
    } else {
      (start_y, end_y)
    };
    let (start_x, end_x) = if start_x > end_x {
      (end_x, start_x)
    } else {
      (start_x, end_x)
    };
    let start_x = if start_x < 0 { 0 } else { start_x };
    let start_y = {
      let t = self.height - start_y - 1;
      if t < 0 { 0 } else { t }
    };
    let end_x = if end_x >= self.width { self.width - 1 } else { end_x };

    let end_y = {
      let t = self.height - end_y - 1;
      if t >= self.height { self.height - 1 } else { t }
    };

    (start_x, start_y, end_x, end_y)
  }

  pub fn draw_vertical_line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
    assert_eq!(start_x, end_x);
    let (start_x, start_y, _end_x, end_y) = self.normalize_coords(start_x, start_y, end_x, end_y);

    let length = start_y - end_y;
    let start_point = ((start_y - MARGIN - self.offset) * self.width) + (start_x + MARGIN);
    for pos in 0..length {
      self.buffer[(start_point - (pos * self.width)) as usize] = FOREGROUND_COLOR;
    }
  }

  pub fn draw_horizontal_line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
    assert_eq!(start_y, end_y);
    let (start_x, start_y, end_x, _end_y) = self.normalize_coords(start_x, start_y, end_x, end_y);

    let length = end_x - start_x;
    let start_point = ((start_y - MARGIN -self.offset) * self.width) + (start_x + MARGIN);
    for pos in 0..length {
      self.buffer[(start_point + pos) as usize] = FOREGROUND_COLOR;
    }
  }

  pub fn fill_square(&mut self, start_x: i32, start_y: i32, width: i32, height: i32, color: u32) {
    assert!(start_x >= 0);
    assert!(start_y >= 0);
    assert!(width >= 0);
    assert!(height >= 0);

    let real_start_y = self.height - start_y - 1 - self.offset;
    let start_point = ((real_start_y - MARGIN) * self.width) + (start_x + MARGIN);

    for x_pos in 0..width {
      for y_pos in 0..height {
        let pos = start_point - (y_pos * self.width) + x_pos;
        self.buffer[pos as usize] = color;
      }
    }
  }
}