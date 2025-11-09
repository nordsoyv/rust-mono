use macroquad::prelude::*;

use crate::entity::{Background, Entity, EntityId, EntityManager, Item, ProgressBar};

pub fn update_miner(entity: &mut Entity, em: &EntityManager) {
  const PROGRESS_PER_SECOND: f32 = 80.0;
  let miner_data = entity.miner_data.as_mut().unwrap();
  if miner_data.progress < 1.0 {
    let delta = get_frame_time();
    miner_data.progress += delta * PROGRESS_PER_SECOND / 100.0;
    if miner_data.progress > 1.0 {
      let out = em.get_entity(miner_data.output);
      let mut a = out.borrow_mut();
      if let Some(o) = a.output_data.as_mut() {
        if o.has_room() {
          o.push_item(Item {
            name: "Iron Ore".to_owned(),
          });
          miner_data.progress = 0.0
        }
      }
    }
  }
}

pub fn draw_miner(entity: &Entity) {
  if let Some(md) = entity.miner_data.as_ref()
    && let Some(ui_data) = entity.ui_data.as_ref()
  {
    md.background.draw(ui_data.x_pos, ui_data.y_pos, "Miner");
    md.progress_bar
      .draw_progress(ui_data.x_pos, ui_data.y_pos, md.progress);
  }
}

pub struct MinerData {
  progress: f32,
  background: Background,
  progress_bar: ProgressBar,
  pub output: EntityId,
}

impl MinerData {
  pub async fn new() -> Self {
    let miner = Self {
      progress: 0.0,
      background: Background::new(100.0, 200.0).await,
      progress_bar: ProgressBar::new(50.0, 30.0, 100.0, 10.0),
      output: 0,
    };

    miner
  }
}
