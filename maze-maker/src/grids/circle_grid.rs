use std::f32::consts::PI;
use std::ops::Add;

use eframe::egui::{Color32, Painter, Pos2, Stroke, Vec2};

use crate::grids::{Cell, CellCoord, Direction, Grid};

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
      distance: 0,
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

  fn draw_background(&self) -> (Pos2, f32, Color32) {
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
    todo!()
  }
}

struct Ring {
  cells: Vec<CircleCell>,
}

impl Ring {
  fn len(&self) -> usize {
    self.cells.len()
  }
}

pub struct CircleGrid {
  rings: Vec<Ring>,
  num_rings: i32,
  pub cell_radius: i32,
  pub margin: i32,
  entrance: CellCoord,
  exit: CellCoord,
  has_solution: bool,
  dead_ends: Vec<CellCoord>,
}

impl CircleGrid {
  pub fn new(rings: i32, cell_radius: i32, margin: i32) -> CircleGrid {
    CircleGrid {
      num_rings: rings,
      cell_radius,
      margin,
      entrance: CellCoord::new(-1, -1),
      exit: CellCoord::new(-1, -1),
      has_solution: false,
      dead_ends: vec![],
      rings: vec![],
    }
  }
  fn get_mut_cell_internal(&mut self, coord: CellCoord) -> Option<&mut CircleCell> {
    let ring = coord.x_pos as usize;
    let index = coord.y_pos as usize;
    if ring >= self.num_rings as usize {
      return None;
    }
    let curr_ring = &self.rings[ring as usize];
    if index >= curr_ring.cells.len() {
      return None;
    }
    return Some(&mut self.rings[ring].cells[index]);
  }

  fn get_cell_internal(&self, coord: CellCoord) -> Option<&CircleCell> {
    let ring = coord.x_pos as usize;
    let index = coord.y_pos as usize;
    if ring >= self.num_rings as usize {
      return None;
    }
    let curr_ring = &self.rings[ring as usize];
    if index >= curr_ring.cells.len() {
      return None;
    }
    return Some(&self.rings[ring].cells[index]);
  }

  fn get_ring(&self, index: i32) -> Option<&Ring> {
    if index < 0 {
      return None;
    }
    self.rings.get(index as usize)
  }
}

impl Grid for CircleGrid {
  fn get_cell(&self, coord: CellCoord) -> Option<&dyn Cell> {
    if let Some(c) = self.get_cell_internal(coord) {
      return Some(c);
    }
    None
  }

  fn get_mut_cell(&mut self, coord: CellCoord) -> Option<&mut dyn Cell> {
    if let Some(c) = self.get_mut_cell_internal(coord) {
      return Some(c);
    }
    None
  }

  fn get_cell_in_dir(&self, coord: CellCoord, dir: Direction) -> Option<CellCoord> {
    let mut target_x = coord.x_pos;
    let mut target_y = coord.y_pos;

    let ring = self.get_ring(coord.x_pos);
    if ring.is_none() {
      return None;
    }
    let current_ring = ring.unwrap();
    // +1 on y is CCW
    match dir {
      Direction::CW => {
        target_y = target_y - 1;
        if target_y < 0 {
          target_y = (current_ring.len() - 1) as i32
        }
      }
      Direction::CCW => {
        target_y = target_y + 1;
        if target_y >= current_ring.len() as i32 {
          target_y = 0
        }
      }
      Direction::Inward1 => {
        let inward_ring = self.get_ring(coord.x_pos - 1);
        if inward_ring.is_none() {
          return None;
        }
        target_x = target_x - 1;
        let inward_ring = inward_ring.unwrap();
        if inward_ring.len() != current_ring.len() {
          target_y = target_y / 2;
        }
      }
      Direction::Inward2 => {
        let inward_ring = self.get_ring(coord.x_pos - 1);
        if inward_ring.is_none() {
          return None;
        }
        target_x = target_x - 1;
        let inward_ring = inward_ring.unwrap();
        if inward_ring.len() != current_ring.len() {
          target_y = target_y / 2;
        }
      }
      Direction::Outward1 => {
        let outward_ring = self.get_ring(coord.x_pos + 1);
        if outward_ring.is_none() {
          return None;
        }
        target_x = target_x + 1;
        let outward_ring = outward_ring.unwrap();
        if current_ring.len() != outward_ring.len() {
          target_y = target_y * 2;
        }
      }
      Direction::Outward2 => {
        let outward_ring = self.get_ring(coord.x_pos + 1);
        if outward_ring.is_none() {
          return None;
        }
        target_x = target_x + 1;
        let outward_ring = outward_ring.unwrap();
        if current_ring.len() != outward_ring.len() {
          target_y = (target_y * 2) + 1;
        }
      }
      _ => {
        return None;
      }
    }

    Some(CellCoord::new(target_x, target_y))
  }

  fn carve(&mut self, coord_start: CellCoord, dir: Direction) {
    let coord_end = self.get_cell_in_dir(coord_start, dir);
    if coord_end.is_none() {
      return;
    }
    let coord_end = coord_end.unwrap();
    {
      let mut start_cell = self.get_mut_cell_internal(coord_start).unwrap();
      start_cell.part_of_maze = true;
      match dir {
        Direction::CCW => {
          start_cell.ccw = Some(coord_end);
        }
        Direction::CW => {
          start_cell.cw = Some(coord_end);
        }
        Direction::Inward1 => {
          start_cell.inward = Some(coord_end);
        }
        Direction::Inward2 => {
          start_cell.inward = Some(coord_end);
        }
        Direction::Outward1 => {
          start_cell.outward_1 = Some(coord_end);
        }
        Direction::Outward2 => {
          start_cell.outward_2 = Some(coord_end);
        }
        _ => {
          panic!("unknown direction")
        }
      }
    }
    {
      let mut end_cell = self.get_mut_cell_internal(coord_end).unwrap();
      end_cell.part_of_maze = true;
      end_cell.set_color(Some(Color32::BLUE));
      match dir {
        Direction::CCW => {
          end_cell.cw = Some(coord_start);
        }
        Direction::CW => {
          end_cell.ccw = Some(coord_start);
        }
        Direction::Inward1 => {
          end_cell.outward_1 = Some(coord_start);
        }
        Direction::Inward2 => {
          end_cell.outward_2 = Some(coord_start);
        }
        Direction::Outward1 => {
          end_cell.inward = Some(coord_start);
        }
        Direction::Outward2 => {
          end_cell.inward = Some(coord_start);
        }
        _ => {
          panic!("unknown direction")
        }
      }
    }
  }

  fn get_allowed_directions(&self, coord: CellCoord) -> Vec<Direction> {
    let mut dirs = vec![];
    if self.can_carve(coord, Direction::CW) {
      dirs.push(Direction::CW);
    }
    if self.can_carve(coord, Direction::CCW) {
      dirs.push(Direction::CCW);
    }
    if self.can_carve(coord, Direction::Inward1) {
      dirs.push(Direction::Inward1);
    }
    if self.can_carve(coord, Direction::Inward2) {
      dirs.push(Direction::Inward2);
    }
    if self.can_carve(coord, Direction::Outward1) {
      dirs.push(Direction::Outward1);
    }
    if self.can_carve(coord, Direction::Outward2) {
      dirs.push(Direction::Outward2);
    }
    return dirs;
  }

  fn draw(&self, painter: &Painter) {
    let mut points = vec![];
    let shape = Stroke::new(1.0, Color32::BLACK);

    for ring in &self.rings {
      for cell in &ring.cells {
        points.extend(cell.draw());
      }
    }

    let center = Vec2::new(
      self.margin as f32 + (self.num_rings as f32 * self.cell_radius as f32),
      self.margin as f32 + (self.num_rings as f32 * self.cell_radius as f32),
    );

    points
      .into_iter()
      .for_each(|points| painter.line_segment([points.0.add(center), points.1.add(center)], shape));
  }

  fn draw_background(&self, painter: &Painter) {
    let mut backgrounds = vec![];
    for ring in &self.rings {
      for cell in &ring.cells {
        backgrounds.push(cell.draw_background());
      }
    }
    let maze_center = Vec2::new(
      self.margin as f32 + (self.num_rings as f32 * self.cell_radius as f32),
      self.margin as f32 + (self.num_rings as f32 * self.cell_radius as f32),
    );

    backgrounds
      .into_iter()
      .filter(|p| p.2 != Color32::TRANSPARENT)
      .for_each(|(center, radius, color)| {
        painter.circle_filled(center + maze_center, radius, color)
      });
  }

  fn init(&mut self) {
    self.rings = vec![];
    let mut segments = 4;
    for num_ring in 0..self.num_rings {
      let mut curr_ring = Ring { cells: vec![] };
      let circumference = self.cell_radius as f32 * (num_ring + 1) as f32 * 2.0 * PI;
      let cell_width = circumference / segments as f32;
      if cell_width > self.cell_radius as f32 * 2.0 {
        segments = segments * 2;
      }
      let arc_length = (2.0 * PI) / segments as f32;
      for i in 0..segments {
        curr_ring.cells.push(CircleCell::new(
          (num_ring * self.cell_radius) as f32,
          ((num_ring + 1) * self.cell_radius) as f32,
          i as f32 * arc_length,
          (i + 1) as f32 * arc_length,
          CellCoord::new(num_ring, i),
        ));
      }
      self.rings.push(curr_ring);
    }
    let outer_ring = &mut self.rings[(self.num_rings - 1) as usize];
    for cell in &mut outer_ring.cells {
      cell.outward_1 = Some(CellCoord::new(-10, -10));
      cell.outward_2 = Some(CellCoord::new(-10, -10));
    }

    self.entrance = CellCoord::new(self.num_rings - 1, 0);
    self.exit = CellCoord::new(0, 0);

    self.get_mut_cell_internal(self.entrance).unwrap().outward_1 = Some(CellCoord {
      x_pos: -1,
      y_pos: -1,
    });
  }

  fn get_size_in_pixels(&self) -> (f32, f32) {
    let size = (self.num_rings * self.cell_radius * 2) as f32;
    (size, size)
  }

  fn get_num_cells_horizontal(&self) -> i32 {
    self.num_rings
  }

  fn get_num_cells_vertical(&self) -> i32 {
    self.num_rings
  }

  fn get_cell_size(&self) -> i32 {
    self.cell_radius as i32
  }

  fn get_margin(&self) -> i32 {
    self.margin
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
    // todo!()
  }

  fn count_dead_ends(&self) -> usize {
    self.dead_ends.len()
  }

  fn remove_dead_end(&mut self) {
    todo!()
  }
}
