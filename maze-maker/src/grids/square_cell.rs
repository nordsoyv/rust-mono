use crate::grids::{Cell, CellCoord};
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

  pub fn draw_background(&self, cell_width: f32, cell_height: f32, margin: f32) -> (Rect, Color32) {
    let color: Color32;
    if self.color.is_some() {
      color = self.color.unwrap()
    } else {
      return (Rect::NOTHING, Color32::TRANSPARENT);
    }
    let top_left = Pos2::new(
      (self.coord.x_pos as f32 * cell_width) as f32 + margin,
      (self.coord.y_pos as f32 * cell_height) as f32 + margin,
    );
    let bottom_right = Pos2::new(
      ((self.coord.x_pos as f32 + 1.0) * (cell_width)) + margin,
      ((self.coord.y_pos as f32 + 1.0) * (cell_height)) + margin,
    );
    let rect = Rect::from_min_max(top_left, bottom_right);
    return (rect, color);
  }

  pub fn draw(&self, _cell_inset: f32, cell_size: f32, margin: f32) -> Vec<(Pos2, Pos2)> {
    let mut points = vec![];
    let x_coord = self.coord.x_pos as f32;
    let y_coord = self.coord.y_pos as f32;
    if self.top.is_none() {
      let y_pos = (y_coord + 1.0) * cell_size;
      let p1 = Pos2::new(x_coord * cell_size, y_pos);
      let p2 = Pos2::new((x_coord + 1.0) * cell_size, y_pos);
      points.push((p1, p2));
    }
    if self.bottom.is_none() {
      let y_pos = y_coord * cell_size;
      let p1 = Pos2::new(x_coord * cell_size, y_pos);
      let p2 = Pos2::new((x_coord + 1.0) * cell_size, y_pos);
      points.push((p1, p2));
    }
    if self.left.is_none() {
      let x_pos = x_coord * cell_size;
      let p1 = Pos2::new(x_pos, y_coord * cell_size);
      let p2 = Pos2::new(x_pos, (y_coord + 1.0) * cell_size);
      points.push((p1, p2));
    }
    if self.right.is_none() {
      let x_pos = (x_coord + 1.0) * cell_size;
      let p1 = Pos2::new(x_pos, y_coord * cell_size);
      let p2 = Pos2::new(x_pos, (y_coord + 1.0) * cell_size);
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
