use std::convert::TryFrom;

use minifb::{Key, Menu, MouseMode, Window, WindowOptions};

use crate::canvas::Canvas;
use crate::cell::CellCoord;
use crate::common::{CELL_ACTIVE_COLOR, CELL_HEIGHT, CELL_WIDTH, HEIGHT, MARGIN, NUM_CELLS, WIDTH};
use crate::generators::Generator;
use crate::generators::growing_tree::{GrowingTreeGenerator, Strategy};
use crate::maze::SquareGrid2D;

mod canvas;
mod cell;
mod common;
mod maze;
mod generators;

const MENU_NEW_MAZE: usize = 1;
const MENU_FASTER: usize = 2;
const MENU_SLOWER: usize = 3;

struct AppState {
  generator: Box<dyn Generator>,
  saved: bool,
  grid: SquareGrid2D,
  generate_steps: i32,
}

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

fn get_title(app_state : &AppState)-> String {
  return format!("Maze type: {} -- Generation speed: {}",app_state.generator.name(),  app_state.generate_steps);
}

fn main() {
  let mut window = Window::new(
    "Test - ESC to exit",
    usize::try_from(WIDTH).unwrap(),
    usize::try_from(HEIGHT).unwrap(),
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });

  let mut app_state = AppState {
    generator: Box::new(GrowingTreeGenerator::new(Strategy::Last)),
    saved: false,
    grid: SquareGrid2D::new(NUM_CELLS, NUM_CELLS),
    generate_steps: 1,
  };
  app_state.generator.init(&mut app_state.grid);
  let mut menu = Menu::new("Main").unwrap();
  menu.add_item("New maze", MENU_NEW_MAZE).enabled(true).shortcut(Key::N, 0).build();
  menu.add_item("Faster", MENU_FASTER).enabled(true).shortcut(Key::F, 0).build();
  menu.add_item("Slower", MENU_SLOWER).enabled(true).shortcut(Key::S, 0).build();
  window.add_menu(&menu);
  window.set_title(get_title(&app_state).as_str());
  while window.is_open() && !window.is_key_down(Key::Escape) {
    {
      let menu_status = window.is_menu_pressed();
      match menu_status {
        None => {}
        Some(cmd) => {
          match cmd {
            MENU_NEW_MAZE => {
              app_state.grid = SquareGrid2D::new(NUM_CELLS, NUM_CELLS);
              app_state.generator = Box::new(GrowingTreeGenerator::new(Strategy::Last));
              app_state.generator.init(&mut app_state.grid);
            }
            MENU_FASTER => {
              app_state.generate_steps = app_state.generate_steps + 1;
              window.set_title(get_title(&app_state).as_str());
            }
            MENU_SLOWER => {
              app_state.generate_steps = app_state.generate_steps - 1;
              window.set_title(get_title(&app_state).as_str());
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
      if !app_state.generator.done() {
        for _ in 0..app_state.generate_steps {
          app_state.generator.generate_step(&mut app_state.grid);
        }
      }
      if app_state.generator.done() {
        // window.set_cursor_style(CursorStyle::Arrow);
        if mouse_coord.x_pos != -1 && mouse_coord.y_pos != -1 {
          let cell = app_state.grid.get_mut_cell(mouse_coord);
          cell.color = Some(CELL_ACTIVE_COLOR);
        }
        app_state.grid.draw(&mut canvas);
        if mouse_coord.x_pos != -1 && mouse_coord.y_pos != -1 {
          let cell = app_state.grid.get_mut_cell(mouse_coord);
          cell.color = None;
        }

        if window.is_key_down(Key::S) && !app_state.saved {
          println!("Saving image");
          save_image(&canvas.buffer, WIDTH, HEIGHT);
          app_state.saved = true;
          println!("image is saved");
        }
      } else {
        app_state.grid.draw(&mut canvas);
      }

      // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
      window.update_with_buffer(&canvas.buffer).unwrap();
    } // buffer lock ends here
  }
}

