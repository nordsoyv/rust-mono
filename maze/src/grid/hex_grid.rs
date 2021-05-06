use canvas::Canvas;

use crate::common::Direction;
use crate::grid::types::{Cell, CellCoord, Grid};

#[derive(Clone, Copy, Debug)]
pub struct HexCell {
  north: Option<CellCoord>,
  south: Option<CellCoord>,
  north_east: Option<CellCoord>,
  north_west: Option<CellCoord>,
  south_east: Option<CellCoord>,
  south_west: Option<CellCoord>,

  coord: CellCoord,
  part_of_maze: bool,
  color: Option<u32>,
  distance: i32,
}

impl HexCell {
  pub fn default(x: i32, y: i32) -> HexCell {
    HexCell {
      north: None,
      north_east: None,
      north_west: None,
      south: None,
      south_east: None,
      south_west: None,
      coord: CellCoord::new(x, y),
      part_of_maze: false,
      color: None,
      distance: -1,
    }
  }

  pub fn draw(&self, canvas: &mut Canvas, height: f32, size: f32, a_size: f32, b_size: f32) {
    canvas.set_fg_color(0);
    let cx = size + 3.0 * self.coord.x_pos as f32 * a_size;
    let mut cy = b_size + self.coord.y_pos as f32 * height;
    if is_odd(self.coord.x_pos) {
      cy += b_size;
    }

    // f/n = far/near
    // n/s/e/w = north/south/east/west
    let x_fw = cx - size;
    let x_nw = cx - a_size;
    let x_ne = cx + a_size;
    let x_fe = cx + size;

    let y_n = cy + b_size;
    let y_m = cy;
    let y_s = cy - b_size;

    if self.north.is_none() {
      canvas.draw_line(x_nw as i32, y_n as i32, x_ne as i32, y_n as i32);
    }
    if self.north_east.is_none() {
      canvas.draw_line(x_ne as i32, y_n as i32, x_fe as i32, y_m as i32);
    }
    if self.south_east.is_none() {
      canvas.draw_line(x_fe as i32, y_m as i32, x_ne as i32, y_s as i32);
    }
    if self.south.is_none() {
      canvas.draw_line(x_ne as i32, y_s as i32, x_nw as i32, y_s as i32);
    }
    if self.south_west.is_none() {
      canvas.draw_line(x_fw as i32, y_m as i32, x_nw as i32, y_s as i32);
    }
    if self.north_west.is_none() {
      canvas.draw_line(x_fw as i32, y_m as i32, x_nw as i32, y_n as i32);
    }

    if self.coord.x_pos == 1 && self.coord.y_pos == 0 {
      canvas.fill_square(x_nw as i32, y_s as i32, size as i32, size as i32);
    }
  }
}

impl Cell for HexCell {
  fn get_coord(&self) -> CellCoord {
    self.coord
  }

  fn is_part_of_maze(&self) -> bool {
    self.part_of_maze
  }

  fn set_part_of_maze(&mut self, part: bool) {
    self.part_of_maze = part;
  }

  fn set_color(&mut self, color: Option<u32>) {
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
    if self.north.is_some() {
      neighbours.push(self.north.unwrap());
    }
    if self.north_west.is_some() {
      neighbours.push(self.north_west.unwrap());
    }
    if self.north_east.is_some() {
      neighbours.push(self.north_east.unwrap());
    }
    if self.south.is_some() {
      neighbours.push(self.south.unwrap());
    }
    if self.south_west.is_some() {
      neighbours.push(self.south_west.unwrap());
    }
    if self.south_east.is_some() {
      neighbours.push(self.south_east.unwrap());
    }
    neighbours
  }
}

pub struct HexGrid {
  cells: Vec<HexCell>,
  pub width: i32,
  pub height: i32,
  pub cell_size: f32,
  cell_height: f32,
  cell_width: f32,
  a_size: f32,
  b_size: f32,
}

impl HexGrid {
  pub fn new(width: i32, height: i32, cell_size: i32) -> HexGrid {
    let mut cells = vec![];
    for y in 0..height {
      for x in 0..width {
        cells.push(HexCell::default(x, y));
      }
    }
    let a_size: f32 = cell_size as f32 / 2.0;
    let b_size: f32 = (cell_size as f32 * 3.0_f32.sqrt()) / 2.0;
    let cell_width = cell_size as f32 * 2.0;
    let cell_height = b_size * 2.0;
    HexGrid {
      cells,
      width,
      height,
      cell_size: cell_size as f32,
      cell_height,
      cell_width,
      a_size,
      b_size,
    }
  }

  fn get_mut_cell_internal(&mut self, coord: CellCoord) -> Option<&mut HexCell> {
    let index = (coord.y_pos * self.width) + coord.x_pos;
    if (index as usize) < self.cells.len() {
      return Some(&mut self.cells[index as usize]);
    }
    return None;
  }
  fn get_cell_internal(&self, coord: CellCoord) -> Option<&HexCell> {
    let index = (coord.y_pos * self.width) + coord.x_pos;
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
        target_x -= 1;
        if !is_odd(coord.x_pos) {
          target_y -= 1;
        }
      }
      _ => {}
    }

    if target_x < 0 || target_x >= self.width || target_y < 0 || target_y >= self.height {
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
        _ => {}
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

  fn draw(&self, canvas: &mut Canvas) {
    for cell in &self.cells {
      cell.draw(
        canvas,
        self.cell_height,
        self.cell_size,
        self.a_size,
        self.b_size,
      );
    }
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

  fn get_width(&self) -> i32 {
    self.width
  }

  fn init(&mut self) {
    let entrance = CellCoord::new(self.width / 2, 0);
    let exit = CellCoord::new(self.width / 2, self.height - 1);

    self.get_mut_cell_internal(entrance).unwrap().south = Some(CellCoord {
      x_pos: -1,
      y_pos: -1,
    });
    self.get_mut_cell_internal(exit).unwrap().north = Some(CellCoord {
      x_pos: -1,
      y_pos: -1,
    });
  }

  fn get_size_in_pixels(&self) -> (i32, i32) {
    (
      self.width * self.cell_width as i32,
      self.height * self.cell_height as i32,
    )
  }
}

fn is_odd(num: i32) -> bool {
  return num & 1 != 0;
}
