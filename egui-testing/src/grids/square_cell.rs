use crate::common::{Cell, CellCoord};
use eframe::egui::{Color32, Pos2, Rect};

#[derive(Clone, Copy, Debug)]
pub struct SquareCell {
  pub left: Option<CellCoord>,
  pub right: Option<CellCoord>,
  pub top: Option<CellCoord>,
  pub bottom: Option<CellCoord>,
  pub coord: CellCoord,
  pub part_of_maze: bool,
  pub color: Option<Color32>,
  pub distance: i32,
}

impl SquareCell {
  pub fn default(x: f32, y: f32) -> SquareCell {
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

  pub fn draw_background(&self, cell_width: f32, cell_height: f32, margin: f32) -> (Rect, Color32) {
    let color: Color32;
    if self.distance > 0 {
      let dist = (self.distance % (256 * 3)) as u32;
      let part = dist as f32 / 3.0;
      let blue = part as u32;
      let remain = dist - blue;
      let part = remain as f32 / 2.0;
      let green = part as u32;
      let remain = remain - green;
      let red = remain;

      color = Color32::from_rgb(red as u8, green as u8, blue as u8);
    } else if self.color.is_some() {
      color = self.color.unwrap()
    } else {
      return (Rect::NOTHING, Color32::TRANSPARENT);
    }
    let top_left = Pos2::new(
      (self.coord.x_pos * cell_width) + margin,
      (self.coord.y_pos * cell_height) + margin,
    );
    let bottom_right = Pos2::new(
      ((self.coord.x_pos + 1.0) * (cell_width)) + margin,
      ((self.coord.y_pos + 1.0) * (cell_height)) + margin,
    );
    let rect = Rect::from_min_max(top_left, bottom_right);
    return (rect, color);
  }

  pub fn draw(&self, _cell_inset: f32, cell_size: f32, margin: f32) -> Vec<(Pos2, Pos2)> {
    let mut points = vec![];
    if self.top.is_none() {
      let y_pos = (self.coord.y_pos + 1.0) * cell_size;
      let p1 = Pos2::new(self.coord.x_pos * cell_size, y_pos);
      let p2 = Pos2::new((self.coord.x_pos + 1.0) * cell_size, y_pos);
      points.push((p1, p2));
    }
    if self.bottom.is_none() {
      let y_pos = (self.coord.y_pos) * cell_size;
      let p1 = Pos2::new(self.coord.x_pos * cell_size, y_pos);
      let p2 = Pos2::new((self.coord.x_pos + 1.0) * cell_size, y_pos);
      points.push((p1, p2));
    }
    if self.left.is_none() {
      let x_pos = self.coord.x_pos * cell_size;
      let p1 = Pos2::new(x_pos, self.coord.y_pos * cell_size);
      let p2 = Pos2::new(x_pos, (self.coord.y_pos + 1.0) * cell_size);
      points.push((p1, p2));
    }
    if self.right.is_none() {
      let x_pos = (self.coord.x_pos + 1.0) * cell_size;
      let p1 = Pos2::new(x_pos, self.coord.y_pos * cell_size);
      let p2 = Pos2::new(x_pos, (self.coord.y_pos + 1.0) * cell_size);
      points.push((p1, p2));
    }
    let new_points = points
      .into_iter()
      .map(|(p1, p2)| {
        (
          Pos2::new(p1.x + margin, p1.y + margin),
          Pos2::new(p2.x + margin, p2.y + margin),
        )
      })
      .collect();
    return new_points;
  }
}

impl Cell for SquareCell {
  fn get_coord(&self) -> CellCoord {
    self.coord
  }

  fn is_part_of_maze(&self) -> bool {
    self.part_of_maze
  }

  fn set_part_of_maze(&mut self, part: bool) {
    self.part_of_maze = part
  }

  fn set_color(&mut self, color: Option<Color32>) {
    self.color = color;
  }

  fn get_distance(&self) -> i32 {
    self.distance
  }

  fn set_distance(&mut self, dist: i32) {
    self.distance = dist;
  }

  fn get_neighbours(&self) -> Vec<CellCoord> {
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
}
