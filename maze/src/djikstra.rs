use crate::cell::CellCoord;
use crate::maze::SquareGrid2D;

pub struct Djikstra {
  frontier: Vec<CellCoord>,
}

impl Djikstra {
  pub fn new() -> Djikstra {
    Djikstra { frontier: vec![] }
  }

  pub fn run(&mut self, start: CellCoord, grid: &mut SquareGrid2D) {
    {
      grid.cells.iter_mut().for_each(|c| c.distance = -1);
    }
    let start_cell = grid.get_mut_cell(start);
    start_cell.distance = 0;
    let neighbours = start_cell.get_neighbours();
    for n in &neighbours {
      if n.x_pos == -1 || n.y_pos == -1 {
        // marker for entrance and exit
        continue;
      }
      let cell = grid.get_mut_cell(*n);
      cell.distance = 1;
      self.frontier.push(*n);
    }
    while !self.frontier.is_empty() {
      let (neighbours, distance) = {
        let active = grid.get_cell(self.frontier.pop().unwrap());
        let neighbours = active.get_neighbours();
        (neighbours, active.distance)
      };
      for n in &neighbours {
        if n.x_pos == -1 || n.y_pos == -1 {
          // marker for entrance and exit
          continue;
        }
        let cell = grid.get_mut_cell(*n);
        if cell.distance == -1 {
          cell.distance = distance + 1;
          self.frontier.push(*n);
        }
      }
    }
  }
}
