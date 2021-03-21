use std::convert::TryFrom;
use std::process::Command;

use minifb::{Key, Menu, MouseMode, Window, WindowOptions};

use canvas::Canvas;

use crate::app_state::AppState;
use crate::cell::CellCoord;
use crate::common::{CELL_ACTIVE_COLOR, MARGIN};
use crate::djikstra::Djikstra;

mod app_state;
mod cell;
mod common;
mod djikstra;
mod generators;
mod maze;

const MENU_NEW_MAZE: usize = 1;
const MENU_FASTER: usize = 2;
const MENU_SLOWER: usize = 3;
const MENU_DJIKSTRA: usize = 4;
const MENU_SHOW_DIST: usize = 5;
const MENU_INSET_LARGER: usize = 6;
const MENU_INSET_SMALLER: usize = 7;
const MENU_SAVE: usize = 8;
const MENU_PRINT: usize = 9;
const MENU_HARDER: usize = 10;
const MENU_EASIER: usize = 11;
const MENU_NUM_CELLS_INC: usize = 12;
const MENU_NUM_CELLS_DEC: usize = 13;
const MENU_CELL_SIZE_INC: usize = 14;
const MENU_CELL_SIZE_DEC: usize = 15;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;

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

fn get_mouse_pos(window: &Window, app_state: &AppState) -> CellCoord {
  let num_cells = app_state.num_cells;
  return window
    .get_mouse_pos(MouseMode::Discard)
    .map(|(x, y)| {
      let mut x_cell = ((x - MARGIN as f32) / app_state.cell_width as f32) as i32;
      let mut y_cell = ((y - MARGIN as f32) / app_state.cell_height as f32) as i32;
      if x_cell >= num_cells {
        x_cell = num_cells - 1;
      }
      if y_cell >= num_cells {
        y_cell = num_cells - 1;
      }
      if x_cell < 0 {
        x_cell = 0;
      }
      if y_cell < 0 {
        y_cell = 0;
      }
      CellCoord {
        x_pos: x_cell,
        y_pos: num_cells - y_cell - 1,
      }
    })
    .unwrap_or(CellCoord {
      x_pos: -1,
      y_pos: -1,
    });
}

fn create_window(app_state: &AppState) -> Window {
  let mut window = Window::new(
    "Test - ESC to exit",
    usize::try_from(WIDTH).unwrap(),
    usize::try_from(HEIGHT).unwrap(),
    WindowOptions::default(),
  )
  .unwrap_or_else(|e| {
    panic!("{}", e);
  });
  window.limit_update_rate(Some(std::time::Duration::from_millis(16)));
  window.set_background_color(255, 0, 0);

  let mut menu = Menu::new("Main").unwrap();
  menu
    .add_item("New maze", MENU_NEW_MAZE)
    .enabled(true)
    .shortcut(Key::N, 0)
    .build();
  menu
    .add_item("Save", MENU_SAVE)
    .enabled(true)
    .shortcut(Key::L, 0)
    .build();
  menu
    .add_item("Print", MENU_PRINT)
    .enabled(true)
    .shortcut(Key::P, 0)
    .build();
  // menu.add_item("Show distances", MENU_SHOW_DIST).enabled(true).shortcut(Key::D, 0).build();

  // difficulty Q - A
  menu
    .add_item("Harder Maze", MENU_HARDER)
    .enabled(true)
    .shortcut(Key::Q, 0)
    .build();
  menu
    .add_item("Easier Maze", MENU_EASIER)
    .enabled(true)
    .shortcut(Key::A, 0)
    .build();

  // faster slower rendering W - S
  menu
    .add_item("Faster", MENU_FASTER)
    .enabled(true)
    .shortcut(Key::W, 0)
    .build();
  menu
    .add_item("Slower", MENU_SLOWER)
    .enabled(true)
    .shortcut(Key::S, 0)
    .build();

  // more less Cells E - D
  menu
    .add_item("More Cells", MENU_NUM_CELLS_INC)
    .enabled(true)
    .shortcut(Key::E, 0)
    .build();
  menu
    .add_item("Less Cells", MENU_NUM_CELLS_DEC)
    .enabled(true)
    .shortcut(Key::D, 0)
    .build();

  // Cell inset  R - F
  menu
    .add_item("Cell inset larger", MENU_INSET_LARGER)
    .enabled(true)
    .shortcut(Key::R, 0)
    .build();
  menu
    .add_item("Cell inset smaller", MENU_INSET_SMALLER)
    .enabled(true)
    .shortcut(Key::F, 0)
    .build();

  // Cell size  T - G
  menu
    .add_item("Cell size larger", MENU_CELL_SIZE_INC)
    .enabled(true)
    .shortcut(Key::T, 0)
    .build();
  menu
    .add_item("Cell size smaller", MENU_CELL_SIZE_DEC)
    .enabled(true)
    .shortcut(Key::G, 0)
    .build();

  window.add_menu(&menu);
  window.set_title(app_state.get_title().as_str());

  return window;
}

fn main() {
  let mut app_state = AppState::new();

  let mut window = create_window(&app_state);

  app_state.generator.init(&mut app_state.grid);

  while window.is_open() && !window.is_key_down(Key::Escape) {
    let mouse_coord = get_mouse_pos(&window, &app_state);

    let mut canvas = Canvas::new(WIDTH, HEIGHT, HEIGHT - app_state.get_maze_size());

    // {
    //   width: WIDTH,
    //   height: HEIGHT,
    //   buffer: ve
    //c![],
    //   offset: 0,
    // };
    // canvas.clear();
    // canvas.set_offset(HEIGHT - app_state.get_maze_size());

    app_state.generate_maze();
    if app_state.generator.done() {
      if mouse_coord.x_pos != -1 && mouse_coord.y_pos != -1 {
        if app_state.show_dist {
          Djikstra::new().run(mouse_coord, &mut app_state.grid);
        } else {
          if let Some(cell) = app_state.grid.get_mut_cell(mouse_coord) {
            cell.color = Some(CELL_ACTIVE_COLOR);
          }
        }
      }
      app_state.grid.draw(&mut canvas);
      if mouse_coord.x_pos != -1 && mouse_coord.y_pos != -1 {
        if let Some(cell) = app_state.grid.get_mut_cell(mouse_coord) {
          cell.color = None;
        }
      }
    } else {
      app_state.grid.draw(&mut canvas);
    }

    let menu_status = window.is_menu_pressed();
    match menu_status {
      None => {}
      Some(cmd) => match cmd {
        MENU_NEW_MAZE => {
          app_state.generate_new_maze();
        }
        MENU_FASTER => {
          app_state.generate_faster();
          window.set_title(app_state.get_title().as_str());
        }
        MENU_SLOWER => {
          app_state.generate_slower();
          window.set_title(app_state.get_title().as_str());
        }
        MENU_INSET_SMALLER => {
          app_state.inset_smaller();
        }
        MENU_INSET_LARGER => {
          app_state.inset_larger();
        }
        MENU_DJIKSTRA => {
          let mut d = Djikstra::new();
          d.run(mouse_coord, &mut app_state.grid);
        }
        MENU_SHOW_DIST => {
          app_state.toggle_show_distance();
        }
        MENU_SAVE => {
          save_image(&canvas.get_buffer(), WIDTH, HEIGHT);
        }
        MENU_PRINT => {
          if let Some(cell) = app_state.grid.get_mut_cell(mouse_coord) {
            cell.color = None
          }
          canvas.clear();
          app_state.grid.draw(&mut canvas);
          save_image(&canvas.get_buffer(), WIDTH, HEIGHT);
          Command::new("mspaint")
            .args(&["/pt", "image.png"])
            .output()
            .expect("Failed to execute process");
        }
        MENU_HARDER => {
          app_state.difficulty_harder();
          window.set_title(app_state.get_title().as_str());
        }
        MENU_EASIER => {
          app_state.difficulty_easier();
          window.set_title(app_state.get_title().as_str());
        }
        MENU_NUM_CELLS_INC => {
          app_state.num_cell_inc();
        }
        MENU_NUM_CELLS_DEC => {
          app_state.num_cell_dec();
        }
        MENU_CELL_SIZE_INC => {
          app_state.cell_size_larger();
        }
        MENU_CELL_SIZE_DEC => {
          app_state.cell_size_smaller();
        }
        _ => println!("Unhandled menu command"),
      },
    }

    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    window
      .update_with_buffer(&canvas.get_buffer(), WIDTH as usize, HEIGHT as usize)
      .unwrap();
    // buffer lock ends here
  }
}
