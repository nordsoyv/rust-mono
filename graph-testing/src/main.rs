mod context;
mod entity;

use macroquad::prelude::*;

use crate::{
  context::Context,
  entity::{crafter::CrafterData, create_crafter, create_miner, miner::MinerData},
};

#[macroquad::main("MyGame")]
async fn main() {
  let mut context = Context::new();
  let miner_data = MinerData::new().await;

  create_miner(&mut context.entity_manager, miner_data);
  create_crafter(&mut context.entity_manager, CrafterData::new().await);
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
