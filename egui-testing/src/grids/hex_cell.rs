use crate::common::is_odd;
use crate::grids::{Cell, CellCoord};
use eframe::egui::{Color32, Pos2};

#[derive(Clone, Copy, Debug)]
pub struct HexCell {
  pub north: Option<CellCoord>,
  pub south: Option<CellCoord>,
  pub north_east: Option<CellCoord>,
  pub north_west: Option<CellCoord>,
  pub south_east: Option<CellCoord>,
  pub south_west: Option<CellCoord>,

  pub coord: CellCoord,
  pub part_of_maze: bool,
  pub color: Option<Color32>,
  pub distance: i32,
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

  pub fn draw_background(
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
