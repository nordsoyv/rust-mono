use macroquad::prelude::*;

use crate::entity::{Background, Entity, EntityId, ProgressBar};

pub fn draw_crafter(entity: &Entity) {
  if let Some(cd) = entity.crafter_data.as_ref()
    && let Some(ui_data) = entity.ui_data.as_ref()
  {
    cd.background.draw(ui_data.x_pos, ui_data.y_pos, "Crafter");
    cd.progress_bar
      .draw_progress(ui_data.x_pos, ui_data.y_pos, cd.progress);
  }
}

pub struct CrafterData {
  progress: f32,
  background: Background,
  progress_bar: ProgressBar,
  pub input: EntityId,
  pub output: EntityId,
}

impl CrafterData {
  pub async fn new() -> Self {
    let crafter = Self {
      progress: 0.0,
      background: Background::new(100.0, 200.0).await,
      progress_bar: ProgressBar::new(50.0, 30.0, 100.0, 10.0),
      output: 0,
      input: 0,
    };

    crafter
  }
}
