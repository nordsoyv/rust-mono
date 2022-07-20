use eframe::egui::{Color32, Painter, Stroke, Vec2};
use std::f32::consts::PI;
use std::ops::Add;

use crate::grids::circle_cell::CircleCell;
use crate::grids::{Cell, CellCoord, Direction, Grid};

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
    const INITIAL_SEGMENTS: i32 = 8;
    let mut segments = INITIAL_SEGMENTS;
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

    self.exit = CellCoord::new(self.num_rings - 1, 0);
    self.entrance = CellCoord::new(0, 0);
    for segment in 0..INITIAL_SEGMENTS {
      let cell1 = &self.rings[0].cells[segment as usize];
      self.carve(cell1.coord, Direction::CCW);
    }

    {
      // cell1.part_of_maze = false;
      let cell1 = &self.rings[0].cells[0];
      self.carve(cell1.coord, Direction::Outward1);
    }
    {
      let cell1 = &self.rings[0].cells[0];
      let cell2_ccord = self
        .get_cell_in_dir(cell1.coord, Direction::Outward1)
        .unwrap();
      let cell2 = self.get_mut_cell(cell2_ccord).unwrap();
      cell2.set_part_of_maze(false);
    }

    self.get_mut_cell_internal(self.exit).unwrap().outward_1 = Some(CellCoord {
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
    let mut deadends = vec![];
    for ring in &self.rings {
      for cell in &ring.cells {
        let num_neighbours = cell.get_neighbours().len();
        if num_neighbours == 1 {
          deadends.push(cell.get_coord());
        }
      }
    }
    self.dead_ends = deadends;
  }

  fn count_dead_ends(&self) -> usize {
    self.dead_ends.len()
  }

  fn remove_dead_end(&mut self) {
    todo!()
  }
}
