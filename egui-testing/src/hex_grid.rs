use crate::common::{Cell, CellCoord, Direction};
use crate::Grid;
use eframe::egui::{Color32, Painter, Pos2, Stroke};

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
  color: Option<Color32>,
  distance: i32,
}

impl HexCell {
  pub fn default(x: f32, y: f32) -> HexCell {
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

  pub fn draw(
    &self,
    height: f32,
    size: f32,
    a_size: f32,
    b_size: f32,
    margin: f32,
  ) -> Vec<(Pos2, Pos2)> {
    let mut points = vec![];
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

    let y_n = cy - b_size;
    let y_m = cy;
    let y_s = cy + b_size;

    if self.south.is_none() {
      let p1 = Pos2::new(x_nw, y_n);
      let p2 = Pos2::new(x_ne, y_n);
      points.push((p1, p2));
    }
    if self.south_east.is_none() {
      let p1 = Pos2::new(x_ne, y_n);
      let p2 = Pos2::new(x_fe, y_m);
      points.push((p1, p2));
    }
    if self.north_east.is_none() {
      let p1 = Pos2::new(x_fe, y_m);
      let p2 = Pos2::new(x_ne, y_s);
      points.push((p1, p2));
    }
    if self.north.is_none() {
      let p1 = Pos2::new(x_ne, y_s);
      let p2 = Pos2::new(x_nw, y_s);
      points.push((p1, p2));
    }
    if self.north_west.is_none() {
      let p1 = Pos2::new(x_fw, y_m);
      let p2 = Pos2::new(x_nw, y_s);
      points.push((p1, p2));
    }
    if self.south_west.is_none() {
      let p1 = Pos2::new(x_fw, y_m);
      let p2 = Pos2::new(x_nw, y_n);
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

  fn draw_background(
    &self,
    cell_height: f32,
    cell_size: f32,
    a_size: f32,
    b_size: f32,
    margin: f32,
  ) -> (Pos2, f32, Color32) {
    if self.color.is_none() {
      return (Pos2::ZERO, 0.0, Color32::TRANSPARENT);
    }

    let cx = cell_size + 3.0 * self.coord.x_pos as f32 * a_size;
    let mut cy = b_size + self.coord.y_pos as f32 * cell_height;
    if is_odd(self.coord.x_pos) {
      cy += b_size;
    }
    let center = Pos2::new(cx + margin, cy + margin);

    (center, cell_size, self.color.unwrap())
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
  pub width: f32,
  pub height: f32,
  pub cell_size: f32,
  cell_height: f32,
  cell_width: f32,
  a_size: f32,
  b_size: f32,
  margin: f32,
}

impl HexGrid {
  pub fn new(width: i32, height: i32, cell_size: i32, margin: f32) -> HexGrid {
    let mut cells = vec![];
    for y in 0..height as i32 {
      for x in 0..width as i32 {
        cells.push(HexCell::default(x as f32, y as f32));
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
        target_y += 1.0;
      }
      Direction::South => {
        target_y -= 1.0;
      }
      Direction::NorthWest => {
        target_x -= 1.0;
        if is_odd(coord.x_pos) {
          target_y += 1.0;
        }
      }
      Direction::NorthEast => {
        target_x += 1.0;
        if is_odd(coord.x_pos) {
          target_y += 1.0;
        }
      }
      Direction::SouthWest => {
        target_x -= 1.0;
        if !is_odd(coord.x_pos) {
          target_y -= 1.0;
        }
      }
      Direction::SouthEast => {
        target_x += 1.0;
        if !is_odd(coord.x_pos) {
          target_y -= 1.0;
        }
      }
      _ => {}
    }

    if target_x < 0.0 || target_x >= self.width || target_y < 0.0 || target_y >= self.height {
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
    let entrance = CellCoord::new(self.width / 2.0, 0.0);
    let exit = CellCoord::new(self.width / 2.0, self.height - 1.0);

    self.get_mut_cell_internal(entrance).unwrap().south = Some(CellCoord {
      x_pos: -1.0,
      y_pos: -1.0,
    });
    self.get_mut_cell_internal(exit).unwrap().north = Some(CellCoord {
      x_pos: -1.0,
      y_pos: -1.0,
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
}

fn is_odd(num: f32) -> bool {
  return (num as i32) & 1 != 0;
}
