use crate::common::{get_random_float, get_random_usize, is_odd};
use crate::grids::hex_cell::HexCell;
use crate::grids::{Cell, CellCoord, Direction, Grid};
use eframe::egui::{Color32, Painter, Stroke};

pub struct HexGrid {
  cells: Vec<HexCell>,
  pub width: f32,
  pub height: f32,
  pub cell_size: f32,
  cell_height: f32,
  cell_width: f32,
  a_size: f32,
  b_size: f32,
  margin: f32,
  entrance: CellCoord,
  exit: CellCoord,
  has_solution: bool,
  dead_ends: Vec<CellCoord>,
}

impl HexGrid {
  pub fn new(width: i32, height: i32, cell_size: i32, margin: f32) -> HexGrid {
    let mut cells = vec![];
    for y in 0..height as i32 {
      for x in 0..width as i32 {
        cells.push(HexCell::default(x, y));
      }
    }
    let a_size: f32 = cell_size as f32 / 2.0;
    let b_size: f32 = (cell_size as f32 * 3.0_f32.sqrt()) / 2.0;
    let cell_width = cell_size as f32 * 2.0;
    let cell_height = b_size * 2.0;
    HexGrid {
      cells,
      width: width as f32,
      height: height as f32,
      cell_size: cell_size as f32,
      cell_height,
      cell_width,
      a_size,
      b_size,
      margin,
      entrance: CellCoord::new(-1, -1),
      exit: CellCoord::new(-1, -1),
      has_solution: false,
      dead_ends: vec![],
    }
  }

  fn get_mut_cell_internal(&mut self, coord: CellCoord) -> Option<&mut HexCell> {
    let index = (coord.y_pos * self.width as i32) + coord.x_pos;
    if (index as usize) < self.cells.len() {
      return Some(&mut self.cells[index as usize]);
    }
    return None;
  }
  fn get_cell_internal(&self, coord: CellCoord) -> Option<&HexCell> {
    let index = (coord.y_pos * self.width as i32) + coord.x_pos;
    if (index as usize) < self.cells.len() {
      return Some(&self.cells[index as usize]);
    }
    return None;
  }
}

impl Grid for HexGrid {
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

  fn can_carve(&self, coord: CellCoord, dir: Direction) -> bool {
    if let Some(cell_coord) = self.get_cell_in_dir(coord, dir) {
      if let Some(cell) = self.get_cell(cell_coord) {
        return !cell.is_part_of_maze();
      }
    }

    return false;
  }

  fn get_cell_in_dir(&self, coord: CellCoord, dir: Direction) -> Option<CellCoord> {
    let mut target_x = coord.x_pos;
    let mut target_y = coord.y_pos;

    match dir {
      Direction::North => {
        target_y += 1;
      }
      Direction::South => {
        target_y -= 1;
      }
      Direction::NorthWest => {
        target_x -= 1;
        if is_odd(coord.x_pos) {
          target_y += 1;
        }
      }
      Direction::NorthEast => {
        target_x += 1;
        if is_odd(coord.x_pos) {
          target_y += 1;
        }
      }
      Direction::SouthWest => {
        target_x -= 1;
        if !is_odd(coord.x_pos) {
          target_y -= 1;
        }
      }
      Direction::SouthEast => {
        target_x += 1;
        if !is_odd(coord.x_pos) {
          target_y -= 1;
        }
      }
      _ => {}
    }

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
    let coord_end = self.get_cell_in_dir(coord_start, dir);
    if coord_end.is_none() {
      return;
    }
    let coord_end = coord_end.unwrap();
    {
      let mut start_cell = self.get_mut_cell_internal(coord_start).unwrap();
      start_cell.part_of_maze = true;
      match dir {
        Direction::North => {
          start_cell.north = Some(coord_end);
        }
        Direction::NorthWest => {
          start_cell.north_west = Some(coord_end);
        }
        Direction::NorthEast => {
          start_cell.north_east = Some(coord_end);
        }
        Direction::South => {
          start_cell.south = Some(coord_end);
        }
        Direction::SouthWest => {
          start_cell.south_west = Some(coord_end);
        }
        Direction::SouthEast => {
          start_cell.south_east = Some(coord_end);
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
        Direction::North => {
          end_cell.south = Some(coord_start);
        }
        Direction::NorthWest => {
          end_cell.south_east = Some(coord_start);
        }
        Direction::NorthEast => {
          end_cell.south_west = Some(coord_start);
        }
        Direction::South => {
          end_cell.north = Some(coord_start);
        }
        Direction::SouthWest => {
          end_cell.north_east = Some(coord_start);
        }
        Direction::SouthEast => {
          end_cell.north_west = Some(coord_start);
        }
        _ => {
          panic!("unknown direction")
        }
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
    if self.can_carve(coord, Direction::NorthEast) {
      dirs.push(Direction::NorthEast);
    }
    if self.can_carve(coord, Direction::NorthWest) {
      dirs.push(Direction::NorthWest);
    }
    if self.can_carve(coord, Direction::SouthEast) {
      dirs.push(Direction::SouthEast);
    }
    if self.can_carve(coord, Direction::SouthWest) {
      dirs.push(Direction::SouthWest);
    }
    return dirs;
  }

  fn draw(&self, painter: &Painter) {
    let mut points = vec![];
    let shape = Stroke::new(1.0, Color32::BLACK);
    for cell in &self.cells {
      points.extend(cell.draw(
        self.cell_height,
        self.cell_size,
        self.a_size,
        self.b_size,
        self.margin,
      ));
    }
    points
      .into_iter()
      .for_each(|points| painter.line_segment([points.0, points.1], shape));
  }

  fn draw_background(&self, painter: &Painter) {
    let mut backgrounds = vec![];
    for cell in &self.cells {
      backgrounds.push(cell.draw_background(
        self.cell_height,
        self.cell_size,
        self.a_size,
        self.b_size,
        self.margin,
      ));
    }
    backgrounds
      .into_iter()
      .for_each(|(center, radius, color)| painter.circle_filled(center, radius, color));
  }

  fn set_cell_size(&mut self, cell_size: i32) {
    self.cell_size = cell_size as f32;
    let a_size: f32 = self.cell_size / 2.0;
    let b_size: f32 = (self.cell_size * 3.0_f32.sqrt()) / 2.0;
    let cell_width = self.cell_size * 2.0;
    let cell_height = b_size * 2.0;
    self.a_size = a_size;
    self.b_size = b_size;
    self.cell_width = cell_width;
    self.cell_height = cell_height;
  }

  fn get_width(&self) -> f32 {
    self.width
  }

  fn init(&mut self) {
    self.entrance = CellCoord::new(self.width as i32 / 2, 0);
    self.exit = CellCoord::new(self.width as i32 / 2, self.height as i32 - 1);

    self.get_mut_cell_internal(self.entrance).unwrap().south = Some(CellCoord {
      x_pos: -1,
      y_pos: -1,
    });
    self.get_mut_cell_internal(self.exit).unwrap().north = Some(CellCoord {
      x_pos: -1,
      y_pos: -1,
    });
  }

  fn get_size_in_pixels(&self) -> (f32, f32) {
    (
      (self.width * self.cell_width) + (self.cell_width / 2.0),
      (self.height * self.cell_height) + (self.cell_height / 2.0),
    )
  }

  fn get_num_cells_horizontal(&self) -> i32 {
    self.width as i32
  }

  fn get_num_cells_vertical(&self) -> i32 {
    self.height as i32
  }

  fn get_cell_size(&self) -> i32 {
    self.cell_size as i32
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
    self.has_solution = false;
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
        if cell.north.is_none() {
          walled_neighbors.push(Direction::North);
        }
        if cell.north_east.is_none() {
          walled_neighbors.push(Direction::NorthEast);
        }
        if cell.north_west.is_none() {
          walled_neighbors.push(Direction::NorthWest);
        }
        if cell.south.is_none() {
          walled_neighbors.push(Direction::South);
        }
        if cell.south_east.is_none() {
          walled_neighbors.push(Direction::SouthEast);
        }
        if cell.south_west.is_none() {
          walled_neighbors.push(Direction::SouthWest);
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
