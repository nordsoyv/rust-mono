use std::ops::Add;

use eframe::egui::{Color32, Painter, Pos2, Stroke, Vec2};

use crate::grids::{Cell, CellCoord, Direction, Grid};

pub struct CircleCell {
  pub inner_radius: f32,
  pub outer_radius: f32,
  pub start_arc: f32,
  pub end_arc: f32,
}
impl CircleCell {
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
    lines.push((inner_start, outer_start)); // wall
    lines.push((inner_end, outer_end)); // wall

    lines.extend(self.sub_divide_line(inner_start, inner_end, self.inner_radius)); //inner arc
    lines.extend(self.sub_divide_line(outer_start, outer_end, self.outer_radius)); //outer arc
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
}

pub struct CircleGrid {
  cells: Vec<Vec<CircleCell>>,
  pub rings: i32,
  pub cell_radius: i32,
  pub margin: i32,
  entrance: CellCoord,
  exit: CellCoord,
  has_solution: bool,
  dead_ends: Vec<CellCoord>,
}

impl CircleGrid {
  pub fn new(rings: i32, cell_radius: i32, margin: i32) -> CircleGrid {
    let cells: Vec<Vec<CircleCell>> = vec![vec![]];
    CircleGrid {
      rings,
      cell_radius,
      margin,
      entrance: CellCoord::new(-1, -1),
      exit: CellCoord::new(-1, -1),
      has_solution: false,
      cells,
      dead_ends: vec![],
    }
  }
}

impl Grid for CircleGrid {
  fn get_cell(&self, coord: CellCoord) -> Option<&dyn Cell> {
    None
  }

  fn get_mut_cell(&mut self, coord: CellCoord) -> Option<&mut dyn Cell> {
    None
  }

  fn get_cell_in_dir(&self, coord: CellCoord, dir: Direction) -> Option<CellCoord> {
    None
  }

  fn carve(&mut self, coord_start: CellCoord, dir: Direction) {
    todo!()
  }

  fn get_allowed_directions(&self, coord: CellCoord) -> Vec<Direction> {
    todo!()
  }

  fn draw(&self, painter: &Painter) {
    let mut points = vec![];
    let shape = Stroke::new(1.0, Color32::BLACK);

    let mut cells = vec![];
    let mut segments = 4;
    for ring in 0..self.rings {
      let circumference = self.cell_radius as f32 * (ring + 1) as f32 * 2.0 * 3.14;
      let cell_width = circumference / segments as f32;
      if cell_width > self.cell_radius as f32 * 2.0 {
        segments = segments * 2;
      }
      let arc_length = (2.0 * 3.14) / segments as f32;
      for i in 0..segments {
        cells.push(CircleCell {
          inner_radius: (ring * self.cell_radius) as f32,
          outer_radius: ((ring + 1) * self.cell_radius) as f32,
          start_arc: i as f32 * arc_length,
          end_arc: (i + 1) as f32 * arc_length,
        });
      }
    }

    for cell in cells {
      points.extend(cell.draw());
    }

    let center = Vec2::new(
      self.margin as f32 + (self.rings as f32 * self.cell_radius as f32),
      self.margin as f32 + (self.rings as f32 * self.cell_radius as f32),
    );

    points
      .into_iter()
      .for_each(|points| painter.line_segment([points.0.add(center), points.1.add(center)], shape));
  }

  fn draw_background(&self, painter: &Painter) {}

  fn set_cell_size(&mut self, cell_size: i32) {
    todo!()
  }

  fn get_width(&self) -> f32 {
    0.0
  }

  fn init(&mut self) {}

  fn get_size_in_pixels(&self) -> (f32, f32) {
    let size = (self.rings * self.cell_radius * 2) as f32;
    (size, size)
  }

  fn get_num_cells_horizontal(&self) -> i32 {
    self.rings * 2
  }

  fn get_num_cells_vertical(&self) -> i32 {
    self.rings * 2
  }

  fn get_cell_size(&self) -> i32 {
    self.cell_radius as i32
  }

  fn get_margin(&self) -> i32 {
    0
  }

  fn get_entrance(&self) -> CellCoord {
    self.entrance
  }

  fn get_exit(&self) -> CellCoord {
    self.exit
  }

  fn has_solution(&self) -> bool {
    self.has_solution
  }

  fn set_has_solution(&mut self, has_solution: bool) {
    self.has_solution = has_solution;
  }

  fn clear_solution(&mut self) {
    self.has_solution = false;
  }

  fn find_dead_ends(&mut self) {
    todo!()
  }

  fn count_dead_ends(&self) -> usize {
    self.dead_ends.len()
  }

  fn remove_dead_end(&mut self) {
    todo!()
  }
}
