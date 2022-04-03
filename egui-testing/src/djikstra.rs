use crate::grids::{CellCoord, Grid};
use eframe::egui::Color32;

pub struct Djikstra {
  frontier: Vec<CellCoord>,
}

impl Djikstra {
  pub fn new() -> Djikstra {
    Djikstra { frontier: vec![] }
  }

  pub fn run(&mut self, grid: &mut Box<dyn Grid>) {
    let entrance = grid.get_entrance();
    let exit = grid.get_exit();
    let start_cell = grid.get_mut_cell(entrance).unwrap();
    start_cell.set_distance(0);
    let neighbours = start_cell.get_neighbours();
    for n in &neighbours {
      if n.x_pos == -1.0 || n.y_pos == -1.0 {
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
        if n.x_pos == -1.0 || n.y_pos == -1.0 {
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

    let mut active_distance;
    let mut active_cell_coord = exit;
    {
      let active_cell = grid.get_mut_cell(exit).unwrap();
      active_distance = active_cell.get_distance();
      active_cell.set_color(Some(Color32::GOLD));
    }
    let mut counter = 0;
    'outer_loop: loop {
      counter += 1;
      if counter > 10000 {
        dbg!("Djikstra infinite loop!");
        break;
      }
      if active_cell_coord == entrance {
        break;
      }
      let neighbours = {
        let active_cell = grid.get_mut_cell(active_cell_coord).unwrap();
        active_cell.get_neighbours()
      };

      for n in &neighbours {
        if let Some(cell) = grid.get_mut_cell(*n) {
          let distance = cell.get_distance();
          if distance == active_distance - 1 {
            cell.set_color(Some(Color32::GOLD));
            active_distance = distance;
            active_cell_coord = cell.get_coord();
            continue 'outer_loop;
          }
        }
      }
    }
    grid.set_has_solution(true);
  }
}
