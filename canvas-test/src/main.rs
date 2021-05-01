use std::convert::TryFrom;

use minifb::{Key, Menu, MouseMode, Window, WindowOptions};

use canvas::Canvas;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;

const MENU_CLEAR: usize = 1;

struct AppState {}

impl AppState {
  pub fn new() -> AppState {
    AppState {}
  }
}

fn get_mouse_pos(window: &Window) -> Option<(f32, f32)> {
  return window
    .get_mouse_pos(MouseMode::Discard)
    .map(|(x, y)| {
      let new_y = HEIGHT as f32 - y - 1.0;
      Some((x, new_y))
    })
    .unwrap_or(None);
}

fn create_window(_app_state: &AppState) -> Window {
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
    .add_item("Clear", MENU_CLEAR)
    .enabled(true)
    .shortcut(Key::N, 0)
    .build();

  window.add_menu(&menu);
  window.set_title("Testing");

  return window;
}

fn main() {
  let app_state = AppState::new();
  let mut window = create_window(&app_state);
  let mut canvas = Canvas::new(WIDTH, HEIGHT, 0);
  while window.is_open() && !window.is_key_down(Key::Escape) {
    let mouse_coord = get_mouse_pos(&window);
    // dbg!(mouse_coord);
    canvas.set_bg_color(0x00ffffff);
    canvas.set_fg_color(0x00000000);
    canvas.clear();
    if let Some((x, y)) = mouse_coord {
      canvas.draw_line(200, 200, x as i32, y as i32);
    }

    // if mouse_coord.0 >= 0.0 {
    //   canvas.fill_square(mouse_coord.0 as i32, mouse_coord.1 as i32, 200, 200);
    // }

    // canvas.draw_line(200, 200, mouse_coord.0 as i32, mouse_coord.1 as i32);
    // dbg!(mouse_coord);
    // println!("{:} {:}", mouse_coord.0, mouse_coord.1);
    window
      .update_with_buffer(&canvas.get_buffer(), WIDTH as usize, HEIGHT as usize)
      .unwrap();
  }
}
