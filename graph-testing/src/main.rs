mod miner;

// use crate::miner;
use macroquad::hash;
use macroquad::ui::root_ui;
use macroquad::{prelude::*, ui::widgets::Window};

use crate::miner::Miner;
#[macroquad::main("MyGame")]
async fn main() {
  //let mut value_creator = ValueCreator::new();
  let mut miner = Miner::new().await;
  loop {
    clear_background(DARKBLUE);
    if is_key_pressed(KeyCode::Escape) {
      miniquad::window::order_quit();
    }
    miner.update();
    miner.draw();
    next_frame().await
  }
}

enum ValueCreatorKind {
  Constant,
  Sin,
}
struct ValueCreator {
  kind: ValueCreatorKind,
}

impl ValueCreator {
  fn new() -> Self {
    Self {
      kind: ValueCreatorKind::Constant,
    }
  }

  fn draw(&mut self) {
    Window::new(hash!(), vec2(20.0, 20.0), vec2(200.0, 100.0))
      .label("Emitter")
      .ui(&mut root_ui(), |ui| {
        let mut n = match self.kind {
          ValueCreatorKind::Constant => 0,
          ValueCreatorKind::Sin => 1,
        };
        ui.combo_box(hash!(), "Kind", &["Constant", "Sin"], &mut n);
        match n {
          0 => self.kind = ValueCreatorKind::Constant,
          1 => self.kind = ValueCreatorKind::Sin,
          _ => unreachable!(),
        }
      });
  }
}
