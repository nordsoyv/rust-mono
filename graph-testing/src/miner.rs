use std::vec;

use macroquad::prelude::*;

pub struct Miner {
  progress: f32,
  background: Background,
  progress_bar: ProgressBar,
  output: Output,
  x_pos: f32,
  y_pos: f32,
  height: f32,
  width: f32,
}

impl Miner {
  pub async fn new() -> Self {
    Self {
      progress: 0.0,
      background: Background::new(100.0, 200.0).await,
      progress_bar: ProgressBar::new(50.0, 30.0, 100.0, 10.0),
      output: Output::new(200.0, 50.0),
      x_pos: 100.0,
      y_pos: 100.0,
      width: 200.0,
      height: 100.0,
    }
  }

  pub fn draw(&self) {
    self.background.draw(self.x_pos, self.y_pos, "Miner");
    self
      .progress_bar
      .draw_progress(self.x_pos, self.y_pos, self.progress);
    self.output.draw(self.x_pos, self.y_pos);
  }

  pub fn update(&mut self) {
    const PROGRESS_PER_SECOND: f32 = 80.0;

    if self.progress < 1.0 {
      let delta = get_frame_time();
      self.progress += delta * PROGRESS_PER_SECOND / 100.0;
      if self.progress > 1.0 {
        if self.output.has_room() {
          self.output.push_item(Item {
            name: "Iron Ore".to_owned(),
          });
          self.progress = 0.0
        }
      }
    }
  }
}

struct Item {
  name: String,
}

struct Output {
  offset_x: f32,
  offset_y: f32,
  buffer: Vec<Item>,
  is_dragging: bool,
}

impl Output {
  fn new(offset_x: f32, offset_y: f32) -> Self {
    Self {
      offset_x,
      offset_y,
      buffer: vec![],
      is_dragging: false,
    }
  }

  fn update(&mut self, x_pos: f32, y_pos: f32) {
    let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
    let center = Vec2::new(self.offset_x + x_pos, self.offset_y + y_pos); //+ 50.0
    let dist = mouse_pos - center;
    if dist.length() < 10.0 && is_mouse_button_down(MouseButton::Left) {
      self.is_dragging = true;
    }
  }

  fn draw(&self, x_pos: f32, y_pos: f32) {
    let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
    let center = Vec2::new(self.offset_x + x_pos, self.offset_y + y_pos); //+ 50.0
    let dist = mouse_pos - center;

    draw_circle(center.x, center.y, 10.0, ORANGE);
    if dist.length() < 10.0 {
      draw_circle(center.x, center.y, 8.0, WHITE);
      draw_text(
        &self.buffer.len().to_string(),
        center.x + 15.0,
        center.y,
        20.0,
        WHITE,
      );
    }
  }

  fn has_room(&self) -> bool {
    self.buffer.len() < 5
  }

  fn push_item(&mut self, item: Item) {
    self.buffer.push(item);
  }
}

struct ProgressBar {
  offset_x: f32,
  offset_y: f32,
  width: f32,
  height: f32,
}

impl ProgressBar {
  fn new(offset_x: f32, offset_y: f32, width: f32, height: f32) -> Self {
    Self {
      offset_x,
      offset_y,
      width,
      height,
    }
  }
  fn draw_progress(&self, pos_x: f32, pos_y: f32, progress: f32) {
    draw_rectangle(
      pos_x + self.offset_x,
      pos_y + self.offset_y,
      self.width,
      self.height,
      DARKBROWN,
    );
    draw_rectangle(
      pos_x + self.offset_x + 1.0,
      pos_y + self.offset_y + 1.0,
      (self.width - 2.0) * progress,
      self.height - 2.0,
      LIME,
    );
  }
}

struct Background {
  background: Texture2D,
  height: f32,
  width: f32,
}

impl Background {
  async fn new(height: f32, width: f32) -> Self {
    let image = load_texture("./assets/box_200_100.png").await.unwrap();
    Self {
      background: image,
      height,
      width,
    }
  }
  fn draw(&self, x_pos: f32, y_pos: f32, heading: &str) {
    draw_texture(&self.background, x_pos, y_pos, WHITE);
    let font_size = 20;
    let font_scale = 1.0;
    let measure = measure_text(heading, None, font_size, font_scale);
    draw_text(
      heading,
      x_pos + (self.width / 2.0) - (measure.width / 2.0),
      y_pos + 16.0,
      font_size as f32,
      DARKGRAY,
    );
  }
}
