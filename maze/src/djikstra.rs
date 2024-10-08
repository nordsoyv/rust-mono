use crate::grid::types::{CellCoord, Grid};

pub struct Djikstra {
  frontier: Vec<CellCoord>,
}

impl Djikstra {
  pub fn new() -> Djikstra {
    Djikstra { frontier: vec![] }
  }

  pub fn run(&mut self, start: CellCoord, grid: &mut dyn Grid) {
    {
      // grid.reset_cell_dist();
    }
    let start_cell = grid.get_mut_cell(start).unwrap();
    start_cell.set_distance(0);
    let neighbours = start_cell.get_neighbours();
    for n in &neighbours {
      if n.x_pos == -1 || n.y_pos == -1 {
        // marker for entrance and exit
        continue;
      }
      if let Some(cell) = grid.get_mut_cell(*n) {
        cell.set_distance(1);
      }
      self.frontier.push(*n);
    }
    while !self.frontier.is_empty() {
      let (neighbours, distance) = {
        let active = grid.get_cell(self.frontier.pop().unwrap()).unwrap();
        let neighbours = active.get_neighbours();
        (neighbours, active.get_distance())
      };
      for n in &neighbours {
        if n.x_pos == -1 || n.y_pos == -1 {
          // marker for entrance and exit
          continue;
        }
        if let Some(cell) = grid.get_mut_cell(*n) {
          if cell.get_distance() == -1 {
            cell.set_distance(distance + 1);
            self.frontier.push(*n);
          }
        }
      }
    }
  }
}
