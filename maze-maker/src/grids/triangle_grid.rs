use eframe::egui::{Color32, Painter, Stroke};

use crate::common::{get_random_float, get_random_usize, is_even};
use crate::grids::tri_cell::TriangleCell;
use crate::grids::Direction::{East, North, South, West};
use crate::grids::{Cell, CellCoord, Direction, Grid};

pub struct TriangleGrid2D {
  cells: Vec<TriangleCell>,
  pub width: f32,
  pub height: f32,
  pub cell_size: i32,
  pub margin: f32,
  entrance: CellCoord,
  exit: CellCoord,
  // pub cell_height: i32,
  has_solution: bool,
  dead_ends: Vec<CellCoord>,
}

impl TriangleGrid2D {
  pub fn new(width: i32, height: i32, cell_size: i32, margin: f32) -> TriangleGrid2D {
    let mut cells = vec![];
    for y in 0..height {
      for x in 0..width {
        cells.push(TriangleCell::default(x, y));
      }
    }
    TriangleGrid2D {
      cells,
      width: width as f32,
      height: height as f32,
      cell_size,
      margin,
      entrance: CellCoord::new(-1, -1),
      exit: CellCoord::new(-1, -1),
      has_solution: false,
      dead_ends: vec![],
    }
  }

  #[allow(dead_code)]
  pub fn reset_cell_dist(&mut self) {
    self.cells.iter_mut().for_each(|c| c.distance = -1);
  }

  #[allow(dead_code)]
  fn get_cell_internal(&self, coord: CellCoord) -> Option<&TriangleCell> {
    let index = coord.y_pos * self.width as i32 + coord.x_pos;
    if (index as usize) < self.cells.len() {
      return Some(&self.cells[index as usize]);
    }
    return None;
  }

  fn get_mut_cell_internal(&mut self, coord: CellCoord) -> Option<&mut TriangleCell> {
    let index = (coord.y_pos * self.width as i32) + coord.x_pos;
    if (index as usize) < self.cells.len() {
      return Some(&mut self.cells[index as usize]);
    }
    return None;
  }
}

impl Grid for TriangleGrid2D {
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
    let up_right = is_even(coord.x_pos + coord.y_pos);
    let target_x = match dir {
      Direction::West => coord.x_pos - 1,
      Direction::East => coord.x_pos + 1,
      _ => coord.x_pos,
    };
    let target_y = if up_right {
      if dir == Direction::South {
        coord.y_pos + 1
      } else if dir == Direction::North {
        -10
      } else {
        coord.y_pos
      }
    } else {
      if dir == Direction::North {
        coord.y_pos - 1
      } else if dir == Direction::South {
        -10
      } else {
        coord.y_pos
      }
    };

    if target_x < 0
      || target_x >= self.width as i32
      || target_y < 0
      || target_y >= self.height as i32
    {
      return None;
    }
    Some(CellCoord::new(target_x, target_y))
  }

  fn carve(&mut self, coord_start: CellCoord, dir: Direction) {
    let end_cell = self.get_cell_in_dir(coord_start, dir);

    if end_cell.is_none() {
      return;
    }
    let end_cell_coords = end_cell.unwrap();
    if let Some(start_cell) = self.get_mut_cell_internal(coord_start) {
      start_cell.part_of_maze = true;
      match dir {
        Direction::North => {
          start_cell.top = Some(end_cell_coords);
        }
        Direction::South => {
          start_cell.bottom = Some(end_cell_coords);
        }
        Direction::East => {
          start_cell.right = Some(end_cell_coords);
        }
        Direction::West => {
          start_cell.left = Some(end_cell_coords);
        }
        _ => {}
      }
    }

    if let Some(end_cell) = self.get_mut_cell_internal(end_cell_coords) {
      end_cell.part_of_maze = true;
      match dir {
        Direction::North => {
          end_cell.bottom = Some(coord_start);
        }
        Direction::South => {
          end_cell.top = Some(coord_start);
        }
        Direction::East => {
          end_cell.left = Some(coord_start);
        }
        Direction::West => {
          end_cell.right = Some(coord_start);
        }
        _ => {}
      }
    }
  }

  fn get_allowed_directions(&self, coord: CellCoord) -> Vec<Direction> {
    let mut dirs = vec![];
    if self.can_carve(coord, Direction::North) {
      dirs.push(Direction::North);
    }
    if self.can_carve(coord, Direction::South) {
      dirs.push(Direction::South);
    }
    if self.can_carve(coord, Direction::East) {
      dirs.push(Direction::East);
    }
    if self.can_carve(coord, Direction::West) {
      dirs.push(Direction::West);
    }
    return dirs;
  }

  fn draw(&self, painter: &Painter) {
    let mut points = vec![];
    let shape = Stroke::new(1.0, Color32::BLACK);
    for cell in &self.cells {
      points.extend(cell.draw((self.cell_size * 2) as f32, self.margin));
    }
    points
      .into_iter()
      .for_each(|points| painter.line_segment([points.0, points.1], shape));
  }

  fn draw_background(&self, painter: &Painter) {
    let mut backgrounds = vec![];
    for cell in &self.cells {
      backgrounds.push(cell.draw_background((self.cell_size * 2) as f32, self.margin as f32));
    }
    backgrounds
      .into_iter()
      .for_each(|(center, radius, color)| painter.circle_filled(center, radius, color));
  }

  fn set_cell_size(&mut self, cell_size: i32) {
    self.cell_size = cell_size;
  }

  fn get_width(&self) -> f32 {
    self.width
  }

  fn init(&mut self) {
    let mut ent_x = (self.width / 2.0) as i32;
    if is_even(ent_x) {
      ent_x += 1;
    }
    let mut exit_x = (self.width / 2.0) as i32;
    if is_even(exit_x) {
      exit_x += 1;
    }
    self.entrance = CellCoord::new(ent_x, 0);

    self.exit = CellCoord::new(exit_x, self.height as i32 - 1);

    self.get_mut_cell_internal(self.entrance).unwrap().bottom = Some(CellCoord {
      x_pos: -1,
      y_pos: -1,
    });
    self.get_mut_cell_internal(self.exit).unwrap().top = Some(CellCoord {
      x_pos: -1,
      y_pos: -1,
    });
  }

  fn get_size_in_pixels(&self) -> (f32, f32) {
    (
      self.width * self.cell_size as f32,
      self.height * self.cell_size as f32,
    )
  }

  fn get_num_cells_horizontal(&self) -> i32 {
    self.width as i32
  }

  fn get_num_cells_vertical(&self) -> i32 {
    self.height as i32
  }

  fn get_cell_size(&self) -> i32 {
    self.cell_size
  }

  fn get_margin(&self) -> i32 {
    self.margin as i32
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
    self.set_has_solution(false);
    for c in &mut self.cells {
      c.color = None;
      c.distance = -1;
    }
  }
  fn find_dead_ends(&mut self) {
    let mut deadends = vec![];
    for c in &self.cells {
      let num_neighbours = c.get_neighbours().len();
      if num_neighbours == 1 {
        deadends.push(c.get_coord());
      }
    }
    self.dead_ends = deadends;
  }
  fn count_dead_ends(&self) -> usize {
    self.dead_ends.len()
  }

  fn remove_dead_end(&mut self) {
    let mut wall_sets = vec![];
    {
      for cell_coord in &self.dead_ends {
        if get_random_float(10.0) < 8.0 {
          continue;
        }
        let mut walled_neighbors = vec![];
        let cell = self.get_cell_internal(*cell_coord).unwrap();
        if cell.top.is_none() {
          walled_neighbors.push(North);
        }
        if cell.bottom.is_none() {
          walled_neighbors.push(South);
        }
        if cell.left.is_none() {
          walled_neighbors.push(West);
        }
        if cell.right.is_none() {
          walled_neighbors.push(East);
        }
        if walled_neighbors.len() > 0 {
          wall_sets.push((cell_coord.clone(), walled_neighbors));
        }
      }
    }
    for (cell_coord, wall_set) in wall_sets {
      let random = get_random_usize(wall_set.len());
      self.carve(cell_coord, wall_set[random]);
    }
  }
}
