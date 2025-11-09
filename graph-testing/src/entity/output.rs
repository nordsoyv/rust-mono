use macroquad::prelude::*;

use crate::entity::{Entity, EntityManager, Item};

pub struct OutputData {
  offset_x: f32,
  offset_y: f32,
  buffer: Vec<Item>,
}
impl OutputData {
  pub fn new(offset_x: f32, offset_y: f32) -> Self {
    Self {
      offset_x,
      offset_y,
      buffer: vec![],
    }
  }
  pub fn has_room(&self) -> bool {
    self.buffer.len() < 5
  }

  pub fn push_item(&mut self, item: Item) {
    self.buffer.push(item);
  }
}

pub fn draw_output(entity: &Entity, em: &EntityManager) {
  let parent = em.get_entity(entity.get_parent_id().unwrap());
  if let Some(od) = entity.output_data.as_ref()
    && let Some(ui_data) = parent.borrow().ui_data.as_ref()
  {
    let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
    let center = Vec2::new(od.offset_x + ui_data.x_pos, od.offset_y + ui_data.y_pos); //+ 50.0
    let dist = mouse_pos - center;

    draw_circle(center.x, center.y, 10.0, ORANGE);
    if dist.length() < 10.0 {
      draw_circle(center.x, center.y, 8.0, WHITE);
      draw_text(
        &od.buffer.len().to_string(),
        center.x + 15.0,
        center.y,
        20.0,
        WHITE,
      );
    }
  }
}
