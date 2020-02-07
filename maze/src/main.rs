mod canvas;
mod cell;
mod common;
mod maze;
mod generators;

use std::convert::TryFrom;
use minifb::{Key, Window, WindowOptions};
use crate::canvas::Canvas;
use crate::common::{WIDTH, HEIGHT };
use crate::maze::Maze;
use crate::generators::growing_tree::{GrowingTreeGenerator, Strategy};
use crate::generators::Generator;


fn main() {
  let mut window = Window::new(
    "Test - ESC to exit",
    usize::try_from(WIDTH).unwrap(),
    usize::try_from(HEIGHT).unwrap(),
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });

  let mut maze = Maze::new(50,50);
  let mut generator: Box<Generator>  = Box::new(GrowingTreeGenerator::new(Strategy::Last)) ;
  generator.init(&mut maze);
  while window.is_open() && !window.is_key_down(Key::Escape) {
    {
      let mut canvas = Canvas {
        width: WIDTH,
        height: HEIGHT,
        buffer: vec![],
      };
      canvas.clear();
      if !generator.done() {
        generator.generate_step(&mut maze);
        generator.generate_step(&mut maze);
        generator.generate_step(&mut maze);
        generator.generate_step(&mut maze);
        generator.generate_step(&mut maze);
      }
      maze.draw(&mut canvas);
      // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
      window.update_with_buffer(&canvas.buffer).unwrap();
    } // buffer lock ends here
  }
}

