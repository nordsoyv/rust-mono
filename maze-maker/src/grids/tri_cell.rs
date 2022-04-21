use eframe::egui::{Color32, Pos2};

use crate::common::is_even;
use crate::grids::{Cell, CellCoord};

#[derive(Clone, Copy, Debug)]
pub struct TriangleCell {
  pub left: Option<CellCoord>,
  pub right: Option<CellCoord>,
  pub top: Option<CellCoord>,
  pub bottom: Option<CellCoord>,
  pub coord: CellCoord,
  pub part_of_maze: bool,
  pub color: Option<Color32>,
  pub distance: i32,
}

impl TriangleCell {
  pub fn default(x: i32, y: i32) -> TriangleCell {
    TriangleCell {
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

  pub fn draw_background(&self, cell_size: f32, margin: f32) -> (Pos2, f32, Color32) {
    if self.color.is_none() {
      return (Pos2::ZERO, 0.0, Color32::TRANSPARENT);
    }

    let x_coord = self.coord.x_pos as f32;
    let y_coord = self.coord.y_pos as f32;
    let half_width = cell_size / 2.0;
    let height = cell_size * (3.0f32.sqrt()) / 2.0;
    let half_height = height / 2.0;

    let cx = half_width + (x_coord * half_width);
    let cy = half_height + (y_coord * height);
    let center = Pos2::new(cx + margin, cy + margin);

    (center, cell_size * 0.4, self.color.unwrap())
  }

  pub fn draw(&self, cell_size: f32, margin: f32) -> Vec<(Pos2, Pos2)> {
    let mut points = vec![];
    let x_coord = self.coord.x_pos as f32;
    let y_coord = self.coord.y_pos as f32;
    let up_right = is_even(self.coord.x_pos + self.coord.y_pos);
    let width = cell_size;
    let half_width = width / 2.0;
    let height = cell_size * (3.0f32.sqrt()) / 2.0;
    let half_height = height / 2.0;

    let cx = half_width + (x_coord * half_width);
    let cy = half_height + (y_coord * height);
    let west_x = cx - half_width;
    let mid_x = cx;
    let east_x = cx + half_width;
    let apex_y;
    let base_y;
    if up_right {
      apex_y = cy - half_height;
      base_y = cy + half_height;
    } else {
      apex_y = cy + half_height;
      base_y = cy - half_height;
    }

    if self.left.is_none() {
      let p1 = Pos2::new(west_x, base_y);
      let p2 = Pos2::new(mid_x, apex_y);
      points.push((p1, p2));
    }
    if self.right.is_none() {
      let p1 = Pos2::new(east_x, base_y);
      let p2 = Pos2::new(mid_x, apex_y);
      points.push((p1, p2));
    }
    if self.top.is_none() && self.bottom.is_none() {
      let p1 = Pos2::new(east_x, base_y);
      let p2 = Pos2::new(west_x, base_y);
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

impl Cell for TriangleCell {
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
