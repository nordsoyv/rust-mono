use crate::grids::{Cell, CellCoord};
use eframe::egui::{Color32, Pos2, Vec2};

pub struct CircleCell {
  pub inner_radius: f32,
  pub outer_radius: f32,
  pub start_arc: f32,
  pub end_arc: f32,

  pub inward: Option<CellCoord>,
  pub outward_1: Option<CellCoord>,
  pub outward_2: Option<CellCoord>,
  pub cw: Option<CellCoord>,
  pub ccw: Option<CellCoord>,

  pub coord: CellCoord,
  pub part_of_maze: bool,
  pub color: Option<Color32>,
  pub distance: i32,
}

impl CircleCell {
  pub fn new(
    inner_radius: f32,
    outer_radius: f32,
    start_arc: f32,
    end_arc: f32,
    coord: CellCoord,
  ) -> CircleCell {
    CircleCell {
      outward_1: None,
      outward_2: None,
      inward: None,
      ccw: None,
      coord,
      part_of_maze: false,
      color: None,
      cw: None,
      inner_radius,
      outer_radius,
      start_arc,
      end_arc,
      distance: -1,
    }
  }

  pub fn draw(&self) -> Vec<(Pos2, Pos2)> {
    let ax = self.inner_radius * self.start_arc.cos();
    let ay = self.inner_radius * self.start_arc.sin();
    let bx = self.outer_radius * self.start_arc.cos();
    let by = self.outer_radius * self.start_arc.sin();

    let cx = self.inner_radius * self.end_arc.cos();
    let cy = self.inner_radius * self.end_arc.sin();
    let dx = self.outer_radius * self.end_arc.cos();
    let dy = self.outer_radius * self.end_arc.sin();

    let inner_start = Pos2::new(ax, ay);
    let outer_start = Pos2::new(bx, by);

    let inner_end = Pos2::new(cx, cy);
    let outer_end = Pos2::new(dx, dy);

    let mut lines = vec![];
    if self.cw.is_none() {
      lines.push((inner_start, outer_start)); // wall
    }
    if self.inward.is_none() {
      lines.extend(self.sub_divide_line(inner_start, inner_end, self.inner_radius));
      //inner arc
    }
    if let Some(out1) = self.outward_1 {
      if let Some(out2) = self.outward_2 {
        if out1.x_pos == -10 && out1.y_pos == -10 && out2.x_pos == -10 && out2.y_pos == -10 {
          lines.extend(self.sub_divide_line(outer_start, outer_end, self.outer_radius));
          //outer arc
        }
      }
    }
    lines
  }

  fn sub_divide_line(&self, point_a: Pos2, point_b: Pos2, radius: f32) -> Vec<(Pos2, Pos2)> {
    let mut lines = vec![];
    let dist = point_a.distance(point_b);
    if dist > 5.0 {
      let mid_x = (point_a.x + point_b.x) / 2.0;
      let mid_y = (point_a.y + point_b.y) / 2.0;

      let mut mid = Vec2::new(mid_x, mid_y);
      mid = mid.normalized();
      mid = mid * radius;

      let lines_a_to_mid = self.sub_divide_line(point_a, mid.to_pos2(), radius);
      let lines_mid_to_b = self.sub_divide_line(mid.to_pos2(), point_b, radius);
      lines.extend(lines_a_to_mid);
      lines.extend(lines_mid_to_b);
    } else {
      lines.push((point_a, point_b));
    }
    lines
  }

  pub fn draw_background(&self) -> (Pos2, f32, Color32) {
    if self.color.is_none() {
      return (Pos2::ZERO, 0.0, Color32::TRANSPARENT);
    }
    let ax = self.inner_radius * self.start_arc.cos();
    let ay = self.inner_radius * self.start_arc.sin();
    let dx = self.outer_radius * self.end_arc.cos();
    let dy = self.outer_radius * self.end_arc.sin();
    let center = Vec2::new((ax + dx) / 2.0, (ay + dy) / 2.0);
    return (
      center.to_pos2(),
      (self.outer_radius - self.inner_radius) / 4.0,
      self.color.unwrap(),
    );
  }
}

impl Cell for CircleCell {
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
    if self.ccw.is_some() {
      neighbours.push(self.ccw.unwrap());
    }
    if self.cw.is_some() {
      neighbours.push(self.cw.unwrap());
    }
    if self.inward.is_some() {
      neighbours.push(self.inward.unwrap());
    }
    if self.outward_1.is_some() {
      neighbours.push(self.outward_1.unwrap());
    }
    if self.outward_2.is_some() {
      neighbours.push(self.outward_2.unwrap());
    }
    neighbours
      .into_iter()
      .filter(|c| c.x_pos >= 0 && c.y_pos >= 0)
      .collect()
    // neighbours
  }
}
