mod canvas;
mod cell;
mod common;
mod maze;
mod generators;

use std::convert::TryFrom;
use minifb::{Key, Window, WindowOptions};
use crate::canvas::Canvas;
use crate::common::{WIDTH, HEIGHT, NUM_CELLS};
use crate::maze::Maze;
use crate::generators::growing_tree::{GrowingTreeGenerator, Strategy};
use crate::generators::Generator;

fn save_image(buffer: &Vec<u32>, width: i32, height: i32) {
//  let buffer = shared_buffer.lock().unwrap();
  let mut img_buf: image::RgbImage = image::ImageBuffer::new(width as u32, height as u32);

  let mut buffer_index = 0;
  for (_x, _y, pixel) in img_buf.enumerate_pixels_mut() {
    let color = buffer[buffer_index];
    buffer_index += 1;
    let red = ((color & 0x00ff0000) >> 16) as u8;
    let green = ((color & 0x0000ff00) >> 8) as u8;
    let blue = (color & 0x000000ff) as u8;

    *pixel = image::Rgb([red, green, blue]);
  }
  img_buf.save("image.png").unwrap();
}

fn main() {
  let mut window = Window::new(
    "Test - ESC to exit",
    usize::try_from(WIDTH).unwrap(),
    usize::try_from(HEIGHT).unwrap(),
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });

  let mut maze = Maze::new(NUM_CELLS, NUM_CELLS);
  let mut generator: Box<Generator> = Box::new(GrowingTreeGenerator::new(Strategy::Last));
  generator.init(&mut maze);
  let mut saved = false;
  while window.is_open() && !window.is_key_down(Key::Escape) {
    {
      let mut canvas = Canvas {
        width: WIDTH,
        height: HEIGHT,
        buffer: vec![],
      };
      canvas.clear();
      //canvas.draw_vertical_line( 0,0,0,10 );
      //canvas.draw_horizontal_line(0,0,20,0);
    //  canvas.fill_square(100,100,10,100,0xffffff00);


      if !generator.done() {
        generator.generate_step(&mut maze);
        generator.generate_step(&mut maze);
        generator.generate_step(&mut maze);
        generator.generate_step(&mut maze);
        generator.generate_step(&mut maze);
      }
      maze.draw(&mut canvas);
      if generator.done() {
        if window.is_key_down(Key::R) {
          println!("Creating new maze");
          generator = Box::new(GrowingTreeGenerator::new(Strategy::LastN(10)));
          maze = Maze::new(40, 40);
          generator.init(&mut maze);
        }
        if window.is_key_down(Key::S) && !saved {
          println!("Saving image");
          save_image(&canvas.buffer, WIDTH, HEIGHT);
          saved = true;
          println!("image is saved");
        }
      }

      // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
      window.update_with_buffer(&canvas.buffer).unwrap();
    } // buffer lock ends here
  }
}

