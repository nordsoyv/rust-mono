use eframe::egui::Pos2;

use crate::common::{Cell, CellCoord, Direction, Grid};

#[derive(Clone, Copy, Debug)]
pub struct SquareCell {
  left: Option<CellCoord>,
  right: Option<CellCoord>,
  top: Option<CellCoord>,
  bottom: Option<CellCoord>,
  coord: CellCoord,
  part_of_maze: bool,
  color: Option<u32>,
  pub distance: i32,
}

impl SquareCell {
  pub fn default(x: f32, y: f32) -> SquareCell {
    SquareCell {
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

  // fn draw_background(
  //   &self,
  //   canvas: &mut Canvas,
  //   cell_inset: i32,
  //   cell_width: i32,
  //   cell_height: i32,
  // ) {
  //   let color;
  //   if self.distance > 0 {
  //     let dist = (self.distance % (256 * 3)) as u32;
  //     let part = dist as f32 / 3.0;
  //     let blue = part as u32;
  //     let remain = dist - blue;
  //     let part = remain as f32 / 2.0;
  //     let green = part as u32;
  //     let remain = remain - green;
  //     let red = remain;
  //
  //     // dbg!(dist, red, green, blue);
  //     let red = red << 16;
  //     let green = green << 8;
  //
  //     color = red | green | blue;
  //   } else if self.color.is_some() {
  //     color = self.color.unwrap()
  //   } else {
  //     return;
  //   }
  //   canvas.set_fg_color(color);
  //   canvas.fill_square(
  //     self.coord.x_pos * cell_width + cell_inset,
  //     self.coord.y_pos * cell_height + cell_inset,
  //     cell_width - cell_inset - cell_inset,
  //     cell_height - cell_inset - cell_inset,
  //   );
  //   if cell_inset > 0 {
  //     if self.top.is_some() {
  //       canvas.fill_square(
  //         ((self.coord.x_pos) * cell_width) + cell_inset,
  //         ((self.coord.y_pos + 1) * cell_height) - cell_inset,
  //         cell_width - cell_inset - cell_inset,
  //         cell_inset,
  //       );
  //     }
  //     if self.bottom.is_some() {
  //       canvas.fill_square(
  //         ((self.coord.x_pos) * cell_width) + cell_inset,
  //         self.coord.y_pos * cell_height,
  //         cell_width - cell_inset - cell_inset,
  //         cell_inset,
  //       );
  //     }
  //     if self.left.is_some() {
  //       canvas.fill_square(
  //         self.coord.x_pos * cell_width,
  //         ((self.coord.y_pos) * cell_height) + cell_inset,
  //         cell_inset,
  //         cell_height - cell_inset - cell_inset,
  //       );
  //     }
  //     if self.right.is_some() {
  //       canvas.fill_square(
  //         ((self.coord.x_pos + 1) * cell_width) - cell_inset,
  //         ((self.coord.y_pos) * cell_height) + cell_inset,
  //         cell_inset,
  //         cell_height - cell_inset - cell_inset,
  //       );
  //     }
  //   }
  // }

  pub fn draw(&self, _cell_inset: f32, cell_size: f32, margin: f32) -> Vec<(Pos2, Pos2)> {
    // self.draw_background(canvas, cell_inset, cell_size, cell_size);
    // canvas.set_fg_color(0x00000000);
    // let margin = 5.0;
    let mut points = vec![];
    if self.top.is_none() {
      let y_pos = (self.coord.y_pos) * cell_size;
      let p1 = Pos2::new(self.coord.x_pos * cell_size, y_pos);
      let p2 = Pos2::new((self.coord.x_pos + 1.0) * cell_size, y_pos);
      points.push((p1, p2));
      // dbg!(&points);
      // canvas.draw_line(
      //   (self.coord.x_pos * cell_size) + cell_inset,
      //   (y_pos) - cell_inset,
      //   ((self.coord.x_pos + 1) * cell_size) - cell_inset,
      //   (y_pos) - cell_inset,
      // );
    }
    if self.bottom.is_none() {
      let y_pos = (self.coord.y_pos + 1.0) * cell_size;
      let p1 = Pos2::new(self.coord.x_pos * cell_size, y_pos);
      let p2 = Pos2::new((self.coord.x_pos + 1.0) * cell_size, y_pos);
      points.push((p1, p2));
      // canvas.draw_line(
      //   (self.coord.x_pos * cell_size) + cell_inset,
      //   y_pos + cell_inset,
      //   ((self.coord.x_pos + 1) * cell_size) - cell_inset,
      //   y_pos + cell_inset,
      // );
    }
    if self.left.is_none() {
      let x_pos = self.coord.x_pos * cell_size;
      let p1 = Pos2::new(x_pos, self.coord.y_pos * cell_size);
      let p2 = Pos2::new(x_pos, (self.coord.y_pos + 1.0) * cell_size);
      points.push((p1, p2));

      // canvas.draw_line(
      //   x_pos + cell_inset,
      //   (self.coord.y_pos * cell_size) + cell_inset,
      //   x_pos + cell_inset,
      //   ((self.coord.y_pos + 1) * cell_size) - cell_inset,
      // )
    }
    if self.right.is_none() {
      let x_pos = (self.coord.x_pos + 1.0) * cell_size;
      let p1 = Pos2::new(x_pos, self.coord.y_pos * cell_size);
      let p2 = Pos2::new(x_pos, (self.coord.y_pos + 1.0) * cell_size);
      points.push((p1, p2));

      // canvas.draw_line(
      //   x_pos - cell_inset,
      //   (self.coord.y_pos * cell_size) + cell_inset,
      //   x_pos - cell_inset,
      //   ((self.coord.y_pos + 1) * cell_size) - cell_inset,
      // );
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
    // if cell_inset > 0 {
    //   if self.top.is_some() {
    //     let y_pos = (self.coord.y_pos + 1) * cell_size;
    //     canvas.draw_line(
    //       (self.coord.x_pos * cell_size) + cell_inset,
    //       y_pos,
    //       (self.coord.x_pos * cell_size) + cell_inset,
    //       (y_pos) - cell_inset,
    //     );
    //
    //     canvas.draw_line(
    //       ((self.coord.x_pos + 1) * cell_size) - cell_inset,
    //       y_pos,
    //       ((self.coord.x_pos + 1) * cell_size) - cell_inset,
    //       (y_pos) - cell_inset,
    //     );
    //   }
    //   if self.bottom.is_some() {
    //     let y_pos = (self.coord.y_pos) * cell_size;
    //     canvas.draw_line(
    //       (self.coord.x_pos * cell_size) + cell_inset,
    //       y_pos,
    //       (self.coord.x_pos * cell_size) + cell_inset,
    //       (y_pos) + cell_inset,
    //     );
    //
    //     canvas.draw_line(
    //       ((self.coord.x_pos + 1) * cell_size) - cell_inset,
    //       y_pos,
    //       ((self.coord.x_pos + 1) * cell_size) - cell_inset,
    //       (y_pos) + cell_inset,
    //     );
    //   }
    //   if self.left.is_some() {
    //     let x_pos = self.coord.x_pos * cell_size;
    //     canvas.draw_line(
    //       x_pos,
    //       (self.coord.y_pos * cell_size) + cell_inset,
    //       x_pos + cell_inset,
    //       (self.coord.y_pos * cell_size) + cell_inset,
    //     );
    //     canvas.draw_line(
    //       x_pos,
    //       ((self.coord.y_pos + 1) * cell_size) - cell_inset,
    //       x_pos + cell_inset,
    //       ((self.coord.y_pos + 1) * cell_size) - cell_inset,
    //     );
    //   }
    //   if self.right.is_some() {
    //     let x_pos = (self.coord.x_pos + 1) * cell_size;
    //     canvas.draw_line(
    //       x_pos,
    //       (self.coord.y_pos * cell_size) + cell_inset,
    //       x_pos - cell_inset,
    //       (self.coord.y_pos * cell_size) + cell_inset,
    //     );
    //
    //     canvas.draw_line(
    //       x_pos,
    //       ((self.coord.y_pos + 1) * cell_size) - cell_inset,
    //       x_pos - cell_inset,
    //       ((self.coord.y_pos + 1) * cell_size) - cell_inset,
    //     );
    //   }
    // }
  }
}

impl Cell for SquareCell {
  fn get_coord(&self) -> CellCoord {
    self.coord
  }

  fn is_part_of_maze(&self) -> bool {
    self.part_of_maze
  }

  fn set_part_of_maze(&mut self, part: bool) {
    self.part_of_maze = part
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

pub struct SquareGrid2D {
  cells: Vec<SquareCell>,
  pub width: f32,
  pub height: f32,
  pub cell_inset: i32,
  pub cell_size: i32,
  pub margin: f32,
  // pub cell_height: i32,
}

impl SquareGrid2D {
  pub fn new(
    width: i32,
    height: i32,
    cell_size: i32,
    cell_inset: i32,
    margin: f32,
  ) -> SquareGrid2D {
    let mut cells = vec![];
    for y in 0..height {
      for x in 0..width {
        cells.push(SquareCell::default(x as f32, y as f32));
      }
    }
    SquareGrid2D {
      cells,
      width: width as f32,
      height: height as f32,
      cell_inset,
      cell_size,
      margin,
    }
  }

  #[allow(dead_code)]
  pub fn reset_cell_dist(&mut self) {
    self.cells.iter_mut().for_each(|c| c.distance = -1);
  }

  #[allow(dead_code)]
  fn get_cell_internal(&self, coord: CellCoord) -> Option<&SquareCell> {
    let index = coord.y_pos * self.width + coord.x_pos;
    if (index as usize) < self.cells.len() {
      return Some(&self.cells[index as usize]);
    }
    return None;
  }

  fn get_mut_cell_internal(&mut self, coord: CellCoord) -> Option<&mut SquareCell> {
    let index = (coord.y_pos * self.width) + coord.x_pos;
    if (index as usize) < self.cells.len() {
      return Some(&mut self.cells[index as usize]);
    }
    return None;
  }
}

impl Grid for SquareGrid2D {
  fn get_cell(&self, coord: CellCoord) -> Option<&dyn Cell> {
    let index = coord.y_pos * self.width + coord.x_pos;
    if (index as usize) < self.cells.len() {
      return Some(&self.cells[index as usize]);
    }
    return None;
  }

  fn get_mut_cell(&mut self, coord: CellCoord) -> Option<&mut dyn Cell> {
    let index = (coord.y_pos * self.width as f32) + coord.x_pos;
    if (index as usize) < self.cells.len() {
      return Some(&mut self.cells[index as usize]);
    }
    return None;
  }

  fn can_carve(&self, coord: CellCoord, dir: Direction) -> bool {
    let target_x = match dir {
      Direction::West => coord.x_pos - 1.0,
      Direction::East => coord.x_pos + 1.0,
      _ => coord.x_pos,
    };
    let target_y = match dir {
      Direction::South => coord.y_pos - 1.0,
      Direction::North => coord.y_pos + 1.0,
      _ => coord.y_pos,
    };

    if target_x < 0.0
      || target_x >= self.width as f32
      || target_y < 0.0
      || target_y >= self.height as f32
    {
      return false;
    }

    if let Some(target_cell) = self.get_cell(CellCoord::new(target_x, target_y)) {
      if !target_cell.is_part_of_maze() {
        return true;
      }
    }

    return false;
  }

  fn get_cell_in_dir(&self, coord: CellCoord, dir: Direction) -> Option<CellCoord> {
    match dir {
      Direction::North => Some(CellCoord::new(coord.x_pos, coord.y_pos + 1.0)),
      Direction::South => Some(CellCoord::new(coord.x_pos, coord.y_pos - 1.0)),
      Direction::East => Some(CellCoord::new(coord.x_pos + 1.0, coord.y_pos)),
      Direction::West => Some(CellCoord::new(coord.x_pos - 1.0, coord.y_pos)),
      _ => None,
    }
  }

  fn carve(&mut self, coord_start: CellCoord, dir: Direction) {
    let x_end = match dir {
      Direction::West => coord_start.x_pos - 1.0,
      Direction::East => coord_start.x_pos + 1.0,
      _ => coord_start.x_pos,
    };
    let y_end = match dir {
      Direction::South => coord_start.y_pos - 1.0,
      Direction::North => coord_start.y_pos + 1.0,
      _ => coord_start.y_pos,
    };
    let coord_end = CellCoord {
      x_pos: x_end,
      y_pos: y_end,
    };
    if coord_start.x_pos < 0.0
      || coord_end.x_pos < 0.0
      || coord_start.y_pos < 0.0
      || coord_end.y_pos < 0.0
      || coord_start.x_pos > self.width
      || coord_end.x_pos > self.width
      || coord_start.y_pos > self.height
      || coord_end.y_pos > self.height
    {
      return;
    }

    if let Some(start_cell) = self.get_mut_cell_internal(coord_start) {
      start_cell.part_of_maze = true;
      match dir {
        Direction::North => {
          start_cell.top = Some(coord_end);
        }
        Direction::South => {
          start_cell.bottom = Some(coord_end);
        }
        Direction::East => {
          start_cell.right = Some(coord_end);
        }
        Direction::West => {
          start_cell.left = Some(coord_end);
        }
        _ => {}
      }
    }

    if let Some(end_cell) = self.get_mut_cell_internal(coord_end) {
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

  fn draw(&self) -> Vec<(Pos2, Pos2)> {
    let mut points = vec![];
    for cell in &self.cells {
      points.extend(cell.draw(self.cell_inset as f32, self.cell_size as f32, self.margin));
    }
    return points;
  }

  fn set_cell_size(&mut self, cell_size: i32) {
    self.cell_size = cell_size;
  }

  fn get_width(&self) -> f32 {
    self.width
  }

  fn init(&mut self) {
    let entrance = CellCoord::new(self.width / 2.0, 0.0);
    let exit = CellCoord::new(self.width / 2.0, self.height - 1.0);

    self.get_mut_cell_internal(entrance).unwrap().bottom = Some(CellCoord {
      x_pos: -1.0,
      y_pos: -1.0,
    });
    self.get_mut_cell_internal(exit).unwrap().top = Some(CellCoord {
      x_pos: -1.0,
      y_pos: -1.0,
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
}
