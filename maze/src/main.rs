use std::convert::TryFrom;
use std::process::Command;

use minifb::{Key, Menu, MouseMode, Window, WindowOptions};

use crate::canvas::Canvas;
use crate::cell::CellCoord;
use crate::common::{CELL_ACTIVE_COLOR, CELL_HEIGHT, CELL_WIDTH, MARGIN, NUM_CELLS};
use crate::djikstra::Djikstra;
use crate::generators::Generator;
use crate::generators::growing_tree::{GrowingTreeGenerator, Strategy};
use crate::maze::SquareGrid2D;

mod canvas;
mod cell;
mod common;
mod maze;
mod generators;
mod djikstra;

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
const MENU_CELL_SIZE_INC: usize= 14;
const MENU_CELL_SIZE_DEC: usize= 15;

const WIDTH:i32 =1000;
const HEIGHT:i32 =1000;


struct AppState {
  generator: Box<dyn Generator>,
  saved: bool,
  grid: SquareGrid2D,
  generate_steps: i32,
  cell_inset: i32,
  show_dist: bool,
  difficulty: i32,
  num_cells: i32,
  cell_width:i32,
  cell_height:i32
}

impl AppState {
  pub fn get_maze_size(&self)-> i32 {
    (self.cell_width * self.num_cells) + ( MARGIN*2)
  }
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

fn get_mouse_pos(window: &Window, num_cells: i32) -> CellCoord {
  return window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
    let mut x_cell = ((x - MARGIN as f32) / CELL_WIDTH as f32) as i32;
    let mut y_cell = ((y - MARGIN as f32) / CELL_HEIGHT as f32) as i32;
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
  }).unwrap_or(CellCoord {
    x_pos: -1,
    y_pos: -1,
  });
}

fn get_title(app_state: &AppState) -> String {
  return format!("Maze type: {} -- Difficulty: {} -- Generation speed: {} ", app_state.generator.name(), app_state.difficulty, app_state.generate_steps);
}

fn generate_new_maze(app_state: &mut AppState) {
  app_state.grid = SquareGrid2D::new(app_state.num_cells, app_state.num_cells, app_state.cell_width, app_state.cell_height, app_state.cell_inset, );
  app_state.generator = Box::new(GrowingTreeGenerator::new(Strategy::LastN(app_state.difficulty)));
  app_state.generator.init(&mut app_state.grid);
}

fn main() {
  let mut window = Window::new(
    "Test - ESC to exit",
    usize::try_from(WIDTH).unwrap(),
    usize::try_from(HEIGHT).unwrap(),
    WindowOptions::default()).unwrap_or_else(|e| {
    panic!("{}", e);
  });
// println!("Width {}", WIDTH);
// println!("H {}", HEIGHT);
  let mut app_state = AppState {
    generator: Box::new(GrowingTreeGenerator::new(Strategy::LastAndRandom(10))),
    saved: false,
    grid: SquareGrid2D::new(30, 30,  15,15,0),
    generate_steps: 1,
    show_dist: false,
    cell_inset: 0,
    difficulty: 10,
    num_cells: 30,
    cell_width:15,
    cell_height:15
  };
  app_state.generator.init(&mut app_state.grid);
  let mut menu = Menu::new("Main").unwrap();
  menu.add_item("New maze", MENU_NEW_MAZE).enabled(true).shortcut(Key::N, 0).build();
  menu.add_item("Save", MENU_SAVE).enabled(true).shortcut(Key::L, 0).build();
  menu.add_item("Print", MENU_PRINT).enabled(true).shortcut(Key::P, 0).build();
  // menu.add_item("Show distances", MENU_SHOW_DIST).enabled(true).shortcut(Key::D, 0).build();

  // difficulty Q - A
  menu.add_item("Harder Maze", MENU_HARDER).enabled(true).shortcut(Key::Q, 0).build();
  menu.add_item("Easier Maze", MENU_EASIER).enabled(true).shortcut(Key::A, 0).build();

  // faster slower rendering W - S
  menu.add_item("Faster", MENU_FASTER).enabled(true).shortcut(Key::W, 0).build();
  menu.add_item("Slower", MENU_SLOWER).enabled(true).shortcut(Key::S, 0).build();

  // more less Cells E - D
  menu.add_item("More Cells", MENU_NUM_CELLS_INC).enabled(true).shortcut(Key::E, 0).build();
  menu.add_item("Less Cells", MENU_NUM_CELLS_DEC).enabled(true).shortcut(Key::D, 0).build();

  // Cell inset  R - F
  menu.add_item("Cell inset larger", MENU_INSET_LARGER).enabled(true).shortcut(Key::R, 0).build();
  menu.add_item("Cell inset smaller", MENU_INSET_SMALLER).enabled(true).shortcut(Key::F, 0).build();

  // Cell size  T - G
  menu.add_item("Cell size larger", MENU_CELL_SIZE_INC).enabled(true).shortcut(Key::T, 0).build();
  menu.add_item("Cell size smaller", MENU_CELL_SIZE_DEC).enabled(true).shortcut(Key::G, 0).build();

  window.add_menu(&menu);
  window.set_title(get_title(&app_state).as_str());
  while window.is_open() && !window.is_key_down(Key::Escape) {
    {
      let mouse_coord = get_mouse_pos(&window, app_state.num_cells);

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
          if app_state.show_dist {
            Djikstra::new().run(mouse_coord, &mut app_state.grid);
          } else {
            let cell = app_state.grid.get_mut_cell(mouse_coord);
            cell.color = Some(CELL_ACTIVE_COLOR);
          }
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


      let menu_status = window.is_menu_pressed();
      match menu_status {
        None => {}
        Some(cmd) => {
          match cmd {
            MENU_NEW_MAZE => {
              generate_new_maze(&mut app_state);
            }
            MENU_FASTER => {
              app_state.generate_steps = app_state.generate_steps + 1;
              if app_state.generate_steps > 100 {
                app_state.generate_steps = 100
              }
              window.set_title(get_title(&app_state).as_str());
            }
            MENU_SLOWER => {
              app_state.generate_steps = app_state.generate_steps - 1;
              if app_state.generate_steps < 1 {
                app_state.generate_steps = 1
              }
              window.set_title(get_title(&app_state).as_str());
            }
            MENU_INSET_SMALLER => {
              app_state.grid.cell_inset = app_state.grid.cell_inset - 1;
              if app_state.grid.cell_inset < 0 {
                app_state.grid.cell_inset = 0;
              }
            }
            MENU_INSET_LARGER => {
              app_state.grid.cell_inset = app_state.grid.cell_inset + 1;
              if app_state.grid.cell_inset > app_state.cell_width / 4 {
                app_state.grid.cell_inset = app_state.cell_width / 4;
              }
            }
            MENU_DJIKSTRA => {
              let mut d = Djikstra::new();
              d.run(mouse_coord, &mut app_state.grid);
            }
            MENU_SHOW_DIST => {
              app_state.show_dist = !app_state.show_dist;
            }
            MENU_SAVE => {
              save_image(&canvas.buffer, WIDTH, HEIGHT);
              app_state.saved = true;
            }
            MENU_PRINT => {
              let cell = app_state.grid.get_mut_cell(mouse_coord);
              cell.color = None;
              canvas.clear();
              app_state.grid.draw(&mut canvas);
              save_image(&canvas.buffer, WIDTH, HEIGHT);
              app_state.saved = true;
              Command::new("mspaint")
                .args(&["/pt", "image.png"])
                .output()
                .expect("Failed to execute process");
            }
            MENU_HARDER => {
              app_state.difficulty -= 1;
              if app_state.difficulty < 1 {
                app_state.difficulty = 1
              }
              generate_new_maze(&mut app_state);
              window.set_title(get_title(&app_state).as_str());
            }
            MENU_EASIER => {
              app_state.difficulty += 1;
              generate_new_maze(&mut app_state);
              window.set_title(get_title(&app_state).as_str());
            }
            MENU_NUM_CELLS_INC => {
              app_state.num_cells += 1;
              if app_state.get_maze_size() > WIDTH {
                app_state.num_cells -= 1;
              }
              generate_new_maze(&mut app_state);
            }
            MENU_NUM_CELLS_DEC => {
              app_state.num_cells -= 1;
              if app_state.num_cells < 1 {
                app_state.num_cells = 1
              }
              generate_new_maze(&mut app_state);
            }
            MENU_CELL_SIZE_INC => {
              app_state.cell_height +=1;
              app_state.cell_width +=1;
              if app_state.get_maze_size() > WIDTH {
                app_state.cell_height -=1;
                app_state.cell_width -=1;
              }
              generate_new_maze(&mut app_state);
            }
            MENU_CELL_SIZE_DEC => {
              app_state.cell_height -=1;
              app_state.cell_width -=1;
              if app_state.cell_width <5 {
                app_state.cell_width =5;
                app_state.cell_height =5;
              }
              generate_new_maze(&mut app_state);
            }
            _ => println!("Unhandled menu command")
          }
        }
      }

      // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
      window.update_with_buffer(&canvas.buffer, WIDTH as usize,HEIGHT as usize).unwrap();
    } // buffer lock ends here
  }
}

