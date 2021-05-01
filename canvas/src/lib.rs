use std::convert::TryFrom;

mod vector;

const BACKGROUND_COLOR: u32 = 0x00000000;
const FOREGROUND_COLOR: u32 = 0x00ffffff;

pub struct Canvas {
  width: i32,
  height: i32,
  x_offset: i32,
  y_offset: i32,
  bg_color: u32,
  fg_color: u32,
  margin: i32,
  buffer: Vec<u32>,
}

impl Canvas {
  pub fn new(width: i32, height: i32, margin: i32) -> Canvas {
    let mut c = Canvas {
      width,
      height,
      y_offset: 0,
      x_offset: 0,
      bg_color: BACKGROUND_COLOR,
      fg_color: FOREGROUND_COLOR,
      margin: margin,
      buffer: vec![],
    };
    c.clear();
    c
  }

  pub fn set_bg_color(&mut self, color: u32) {
    self.bg_color = color;
  }

  pub fn set_fg_color(&mut self, color: u32) {
    self.fg_color = color;
  }

  pub fn get_buffer(&self) -> &Vec<u32> {
    &self.buffer
  }

  pub fn clear(&mut self) {
    self.buffer = Vec::new();
    let size = usize::try_from((self.width) * (self.height)).unwrap();
    self.buffer.resize(size, self.bg_color);
  }

  pub fn set_offset(&mut self, x_offset: i32, y_offset: i32) {
    self.x_offset = x_offset;
    self.y_offset = y_offset;
  }

  fn normalize_coords(
    &self,
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
  ) -> (i32, i32, i32, i32) {
    // let (new_start_y, new_end_y) = if start_y > end_y {
    //   (end_y, start_y)
    // } else {
    //   (start_y, end_y)
    // };
    // let (new_start_x, new_end_x) = if start_x > end_x {
    //   (end_x, start_x)
    // } else {
    //   (start_x, end_x)
    // };
    let (new_start_x, new_start_y, new_end_x, new_end_y) = if start_x < end_x {
      (start_x, start_y, end_x, end_y)
    } else {
      (end_x, end_y, start_x, start_y)
    };

    let new_new_start_x = if new_start_x < 0 { 0 } else { new_start_x };
    let new_new_start_y = {
      let t = self.height - new_start_y - 1;
      if t < 0 {
        0
      } else {
        t
      }
    };
    let new_new_end_x = if new_end_x >= self.width {
      self.width - 1
    } else {
      new_end_x
    };

    let new_new_end_y = {
      let t = self.height - new_end_y - 1;
      if t >= self.height {
        self.height - 1
      } else if t < 0 {
        0
      } else {
        t
      }
    };
    (
      new_new_start_x,
      new_new_start_y,
      new_new_end_x,
      new_new_end_y,
    )
  }

  fn apply_offset(&self, coord_x: i32, coord_y: i32) -> (i32, i32) {
    (coord_x + self.x_offset, coord_y + self.y_offset)
  }

  pub fn draw_line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
    let (start_x, start_y) = self.apply_offset(start_x, start_y);
    let (end_x, end_y) = self.apply_offset(end_x, end_y);

    let (new_start_x, new_start_y, new_end_x, new_end_y) =
      self.normalize_coords(start_x, start_y, end_x, end_y);
    if new_start_x == new_end_x {
      self.draw_vertical_line(new_start_x, new_start_y, new_end_x, new_end_y);
      return;
    }
    if new_start_y == new_end_y {
      self.draw_horizontal_line(new_start_x, new_start_y, new_end_x, new_end_y);
      return;
    }
    self.draw_other_line(new_start_x, new_start_y, new_end_x, new_end_y);
  }

  fn draw_vertical_line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
    assert_eq!(start_x, end_x);
    let (start_y, end_y) = if start_y > end_y {
      (start_y, end_y)
    } else {
      (end_y, start_y)
    };
    let length = start_y - end_y + 1;
    let start_point = ((start_y - self.margin) * self.width) + (start_x + self.margin);
    for pos in 0..length {
      self.buffer[(start_point - (pos * self.width)) as usize] = self.fg_color;
    }
  }

  fn draw_horizontal_line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
    assert_eq!(start_y, end_y);
    let length = end_x - start_x + 1;
    let start_point = ((start_y - self.margin) * self.width) + (start_x + self.margin);

    for pos in 0..length {
      self.buffer[(start_point + pos) as usize] = self.fg_color;
    }
  }

  fn draw_other_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
    if (y1 - y0).abs() < (x1 - x0).abs() {
      if x0 > x1 {
        self.draw_line_low(x1, y1, x0, y0);
      } else {
        self.draw_line_low(x0, y0, x1, y1);
      }
    } else {
      if y0 > y1 {
        self.draw_line_high(x1, y1, x0, y0);
      } else {
        self.draw_line_high(x0, y0, x1, y1);
      }
    }
  }

  fn draw_line_low(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
    let delta_x = x1 - x0;
    let mut delta_y = y1 - y0;
    let mut y_inc = 1;
    if delta_y < 0 {
      y_inc = -1;
      delta_y = -delta_y;
    }
    let mut delta_err = (2 * delta_y) - delta_x;

    let mut y = y0;
    for x in x0..x1 {
      self.buffer[((y * self.width) + x) as usize] = self.fg_color;
      if delta_err > 0 {
        y += y_inc;
        delta_err = delta_err + (2 * (delta_y - delta_x));
      } else {
        delta_err = delta_err + 2 * delta_y
      }
    }
  }

  fn draw_line_high(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
    let mut delta_x = x1 - x0;
    let delta_y = y1 - y0;
    let mut x_inc = 1;
    if delta_x < 0 {
      x_inc = -1;
      delta_x = -delta_x;
    }
    let mut delta_err = (2 * delta_x) - delta_y;

    let mut x = x0;
    for y in y0..y1 {
      self.buffer[((y * self.width) + x) as usize] = self.fg_color;
      if delta_err > 0 {
        x += x_inc;
        delta_err = delta_err + (2 * (delta_x - delta_y));
      } else {
        delta_err = delta_err + 2 * delta_x
      }
    }
  }

  pub fn fill_square(&mut self, start_x: i32, start_y: i32, width: i32, height: i32) {
    assert!(start_x >= 0);
    assert!(start_y >= 0);
    assert!(width >= 0);
    assert!(height >= 0);
    let width = if start_x + width > self.width {
      self.width - start_x
    } else {
      width
    };
    let height = if start_y + height > self.height {
      self.height - start_y
    } else {
      height
    };
    let real_start_y = self.height - start_y - 1 - self.y_offset;
    let start_point =
      ((real_start_y - self.margin) * self.width) + (start_x + self.margin) + self.x_offset;

    for y_pos in 0..height {
      let line_start = start_point - (y_pos * self.width);
      for x_pos in 0..width {
        let pos = line_start + x_pos;
        self.buffer[pos as usize] = self.fg_color;
      }
    }
  }

  #[allow(dead_code)]
  fn output_drawn_pixels(&self) -> String {
    let mut w = String::new();
    for y_pos in 0..self.height {
      let line_start = (y_pos * self.width) as usize;
      for x_pos in 0..self.width {
        let pos = line_start + x_pos as usize;
        if self.buffer[pos] == 0 {
          w.push('0');
        } else {
          w.push('1');
        }
      }
      if y_pos != self.height - 1 {
        w.push('\n');
      }
    }
    return w;
  }
}

#[cfg(test)]
mod tests {
  use crate::Canvas;

  #[test]
  fn can_draw_horizontal_line() {
    let mut canvas = Canvas::new(10, 4, 0);
    let fg = 0x00000001;
    canvas.set_fg_color(fg);
    canvas.draw_line(1, 1, 5, 1);
    let result = "0000000000
0000000000
0111110000
0000000000";
    assert_eq!(canvas.output_drawn_pixels(), result);
  }

  #[test]
  fn can_set_fg_color() {
    let mut canvas = Canvas::new(10, 4, 0);
    let bg = 0;
    let fg = 0x00aaaaaa;
    canvas.set_fg_color(fg);
    canvas.draw_line(1, 1, 5, 1);
    #[rustfmt::skip]
      let result = vec![
      bg,bg,bg,bg,bg,bg,bg,bg,bg,bg,
      bg,bg,bg,bg,bg,bg,bg,bg,bg,bg,
      bg,fg,fg,fg,fg,fg,bg,bg,bg,bg,
      bg,bg,bg,bg,bg,bg,bg,bg,bg,bg,
    ];
    assert_eq!(canvas.get_buffer(), &result);
  }

  #[test]
  fn can_draw_vertical_line() {
    let mut canvas = Canvas::new(3, 10, 0);
    canvas.set_fg_color(0x00000001);
    canvas.draw_line(1, 1, 1, 5);
    #[rustfmt::skip]
      let result = "000
000
000
000
010
010
010
010
010
000";
    assert_eq!(canvas.output_drawn_pixels(), result);
  }

  #[test]
  fn can_draw_sqaure() {
    let mut canvas = Canvas::new(10, 10, 0);
    let fg = 0x00000001;
    canvas.set_fg_color(fg);
    canvas.fill_square(2, 2, 4, 5);
    let result = "0000000000
0000000000
0000000000
0011110000
0011110000
0011110000
0011110000
0011110000
0000000000
0000000000"
      .to_string();
    assert_eq!(canvas.output_drawn_pixels(), result);
  }

  #[test]
  fn can_use_offset_with_fill_square() {
    let mut canvas = Canvas::new(10, 10, 0);
    let fg = 0x00000001;
    canvas.set_fg_color(fg);
    canvas.set_offset(2, 2);
    canvas.fill_square(2, 2, 2, 2);
    let result = "0000000000
0000000000
0000000000
0000000000
0000110000
0000110000
0000000000
0000000000
0000000000
0000000000"
      .to_string();
    assert_eq!(canvas.output_drawn_pixels(), result);
  }

  #[test]
  fn can_use_offset_with_horizontal_line() {
    let mut canvas = Canvas::new(10, 5, 0);
    let fg = 0x00000001;
    canvas.set_fg_color(fg);
    canvas.set_offset(1, 1);
    canvas.draw_line(1, 1, 5, 1);
    let result = "0000000000
0000000000
0011111000
0000000000
0000000000";
    assert_eq!(canvas.output_drawn_pixels(), result);
  }

  #[test]
  fn can_draw_square() {
    let mut canvas = Canvas::new(7, 7, 0);
    let bg = 0;
    let fg = 0x00aaaaaa;
    canvas.set_fg_color(fg);
    canvas.fill_square(2, 2, 4, 4);
    #[rustfmt::skip]
    let result = vec![
      bg,bg,bg,bg,bg,bg,bg,
      bg,bg,fg,fg,fg,fg,bg,
      bg,bg,fg,fg,fg,fg,bg,
      bg,bg,fg,fg,fg,fg,bg,
      bg,bg,fg,fg,fg,fg,bg,
      bg,bg,bg,bg,bg,bg,bg,
      bg,bg,bg,bg,bg,bg,bg,
    ];
    assert_eq!(canvas.get_buffer(), &result);
  }
}
