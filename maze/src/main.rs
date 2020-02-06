mod canvas;
mod cell;
mod common;
mod maze;

use std::convert::TryFrom;
use minifb::{Key, Window, WindowOptions};
use crate::canvas::Canvas;
use crate::common::{WIDTH, HEIGHT};
use crate::maze::recursive_backtracker::RecursiveBacktrackerMaze;
use crate::maze::growing_tree::{GrowingTreeMaze, Strategy};


fn main() {
  let mut window = Window::new(
    "Test - ESC to exit",
    usize::try_from(WIDTH).unwrap(),
    usize::try_from(HEIGHT).unwrap(),
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });

//  let mut maze = RecursiveBacktrackerMaze::new();
  let mut maze = GrowingTreeMaze::new(Strategy::LastAndRandom(50));
  maze.init();
  maze.generate();
  while window.is_open() && !window.is_key_down(Key::Escape) {
    {
      let mut canvas = Canvas {
        width: WIDTH,
        height: HEIGHT,
        buffer: vec![],
      };
      canvas.clear();
      if !maze.done {
        maze.generate_step();
        maze.generate_step();
        maze.generate_step();
        maze.generate_step();
        maze.generate_step();
      }
      maze.draw(&mut canvas);
      // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
      window.update_with_buffer(&canvas.buffer).unwrap();
    } // buffer lock ends here
  }
}

