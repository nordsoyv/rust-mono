mod context;
mod entity;

use macroquad::prelude::*;

use crate::{context::Context, entity::miner::MinerData};

#[macroquad::main("MyGame")]
async fn main() {
  let mut context = Context::new();
  let miner_data = MinerData::new().await;
  context.entity_manager.create_miner(miner_data);

  loop {
    clear_background(DARKBLUE);
    if is_key_pressed(KeyCode::Escape) {
      miniquad::window::order_quit();
    }
    context.update();
    context.draw();
    next_frame().await
  }
}
