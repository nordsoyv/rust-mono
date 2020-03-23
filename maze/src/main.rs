mod canvas;
mod cell;
mod common;
mod maze;
mod generators;

use std::convert::TryFrom;
use minifb::{Key, Window, WindowOptions, MouseMode, Menu};
use crate::canvas::Canvas;
use crate::common::{WIDTH, HEIGHT, NUM_CELLS, CELL_WIDTH, CELL_HEIGHT, MARGIN, CELL_ACTIVE_COLOR};
use crate::maze::SquareGrid2D;
use crate::generators::growing_tree::{GrowingTreeGenerator, Strategy};
use crate::generators::Generator;
use crate::cell::CellCoord;

const MENU_NEW_MAZE: usize = 1;

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

fn get_mouse_pos(window: &Window) -> CellCoord {
  return window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
    let mut x_cell = ((x - MARGIN as f32) / CELL_WIDTH as f32) as i32;
    let mut y_cell = ((y - MARGIN as f32) / CELL_HEIGHT as f32) as i32;
    if x_cell >= NUM_CELLS {
      x_cell = NUM_CELLS - 1;
    }
    if y_cell >= NUM_CELLS {
      y_cell = NUM_CELLS - 1;
    }
    if x_cell < 0 {
      x_cell = 0;
    }
    if y_cell < 0 {
      y_cell = 0;
    }
    CellCoord {
      x_pos: x_cell,
      y_pos: NUM_CELLS - y_cell - 1,
    }
  }).unwrap_or(CellCoord {
    x_pos: -1,
    y_pos: -1,
  });
}

fn main() {
  let mut window = Window::new(
    "Test - ESC to exit",
    usize::try_from(WIDTH).unwrap(),
    usize::try_from(HEIGHT).unwrap(),
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });

  let mut maze = SquareGrid2D::new(NUM_CELLS, NUM_CELLS);
  let mut generator: Box<dyn Generator> = Box::new(GrowingTreeGenerator::new(Strategy::Last));
  generator.init(&mut maze);
  let mut saved = false;
  let mut menu = Menu::new("Main").unwrap();
  // let new_maze = MenuItem::new("")
  menu.add_item("New maze", MENU_NEW_MAZE).enabled(true).build();
  window.add_menu(&menu);
  while window.is_open() && !window.is_key_down(Key::Escape) {
    {
      let menu_status = window.is_menu_pressed();
      match menu_status {
        None => {}
        Some(v) => {
          match v {
            MENU_NEW_MAZE => {
              generator = Box::new(GrowingTreeGenerator::new(Strategy::Last));
              maze = SquareGrid2D::new(NUM_CELLS, NUM_CELLS);
              generator.init(&mut maze);
            }
            _ => println!("Unhandled menu command")
          }
        }
      }
      let mouse_coord = get_mouse_pos(&window);
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
      if generator.done() {
        // window.set_cursor_style(CursorStyle::Arrow);
        if mouse_coord.x_pos != -1 && mouse_coord.y_pos != -1 {
          let cell = maze.get_mut_cell(mouse_coord);
          cell.color = Some(CELL_ACTIVE_COLOR);
        }
        maze.draw(&mut canvas);
        if mouse_coord.x_pos != -1 && mouse_coord.y_pos != -1 {
          let cell = maze.get_mut_cell(mouse_coord);
          cell.color = None;
        }

        if window.is_key_down(Key::R) {
          println!("Creating new maze");
          generator = Box::new(GrowingTreeGenerator::new(Strategy::LastN(10)));
          maze = SquareGrid2D::new(40, 40);
          generator.init(&mut maze);
        }
        if window.is_key_down(Key::S) && !saved {
          println!("Saving image");
          save_image(&canvas.buffer, WIDTH, HEIGHT);
          saved = true;
          println!("image is saved");
        }
      } else {
        maze.draw(&mut canvas);
      }

      // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
      window.update_with_buffer(&canvas.buffer).unwrap();
    } // buffer lock ends here
  }
}

