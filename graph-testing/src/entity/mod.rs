use macroquad::prelude::*;
use std::cell::RefCell;

use crate::entity::{
  miner::{MinerData, draw_miner, update_miner},
  output::{OutputData, draw_output},
};
pub type EntityId = usize;

pub mod miner;
pub mod output;

pub struct EntityManager {
  entities: Vec<RefCell<Entity>>,
}

impl EntityManager {
  pub fn new() -> Self {
    Self { entities: vec![] }
  }
  pub fn create_miner(&mut self, miner_data: MinerData) -> EntityId {
    let miner = Entity {
      id: 0,
      kind: EntityType::Miner,
      miner_data: Some(miner_data),
      output_data: None,
      parent_id: None,
      ui_data: Some(UiData::new(100.0, 100.0, 100.0, 200.0)),
    };
    let id = self.add_entity(miner);

    let output_data = OutputData::new(200.0, 50.0);
    let out_id = self.create_output(id, output_data);
    self
      .get_entity(id)
      .borrow_mut()
      .miner_data
      .as_mut()
      .unwrap()
      .output = out_id;
    return id;
  }
  pub fn create_output(&mut self, parent: EntityId, output_data: OutputData) -> EntityId {
    let out = Entity {
      id: 0,
      kind: EntityType::Output,
      output_data: Some(output_data),
      miner_data: None,
      parent_id: Some(parent),
      ui_data: None,
    };
    return self.add_entity(out);
  }

  pub fn get_entity(&self, id: EntityId) -> &RefCell<Entity> {
    let ent = self.entities.get(id);
    let c = ent.expect("Tried to get entity with wrong id");
    return c;
  }
  pub fn add_entity(&mut self, mut entity: Entity) -> EntityId {
    let id = self.entities.len();
    entity.id = id;
    self.entities.push(RefCell::new(entity));
    return id;
  }
  //pub fn set_id(&self, id: EntityId) {}
  pub fn update(&mut self) {
    self.entities.iter().for_each(|entity| {
      entity.borrow_mut().update(self);
    });
  }
  pub fn draw(&self) {
    self.entities.iter().for_each(|entity| {
      entity.borrow().draw(&self);
    });
  }
}

pub enum EntityType {
  Miner,
  Output,
}

pub struct Entity {
  kind: EntityType,
  id: EntityId,
  parent_id: Option<EntityId>,
  miner_data: Option<MinerData>,
  output_data: Option<OutputData>,
  ui_data: Option<UiData>,
}

impl Entity {
  #[allow(dead_code)]
  fn set_id(&mut self, id: EntityId) {
    self.id = id;
  }

  #[allow(dead_code)]
  fn get_id(&self) -> EntityId {
    self.id
  }

  #[allow(dead_code)]
  fn set_parent_id(&mut self, id: EntityId) {
    self.parent_id = Some(id);
  }

  fn get_parent_id(&self) -> Option<EntityId> {
    self.parent_id
  }
  fn update(&mut self, em: &EntityManager) {
    match self.kind {
      EntityType::Miner => update_miner(self, em),
      EntityType::Output => self.update_output(em),
    }
  }

  fn update_output(&mut self, _: &EntityManager) {}
  fn draw(&self, em: &EntityManager) {
    match self.kind {
      EntityType::Miner => draw_miner(self),
      EntityType::Output => draw_output(self, em),
    }
  }
}

pub struct Item {
  #[allow(dead_code)]
  name: String,
}

pub struct ProgressBar {
  offset_x: f32,
  offset_y: f32,
  width: f32,
  height: f32,
}

impl ProgressBar {
  pub fn new(offset_x: f32, offset_y: f32, width: f32, height: f32) -> Self {
    Self {
      offset_x,
      offset_y,
      width,
      height,
    }
  }
  pub fn draw_progress(&self, pos_x: f32, pos_y: f32, progress: f32) {
    draw_rectangle(
      pos_x + self.offset_x,
      pos_y + self.offset_y,
      self.width,
      self.height,
      DARKBROWN,
    );
    draw_rectangle(
      pos_x + self.offset_x + 1.0,
      pos_y + self.offset_y + 1.0,
      (self.width - 2.0) * progress,
      self.height - 2.0,
      LIME,
    );
  }
}

#[allow(dead_code)]
pub struct Background {
  background: Texture2D,
  height: f32,
  width: f32,
}

impl Background {
  pub async fn new(height: f32, width: f32) -> Self {
    let image = load_texture("./assets/box_200_100.png").await.unwrap();
    Self {
      background: image,
      height,
      width,
    }
  }
  pub fn draw(&self, x_pos: f32, y_pos: f32, heading: &str) {
    draw_texture(&self.background, x_pos, y_pos, WHITE);
    let font_size = 20;
    let font_scale = 1.0;
    let measure = measure_text(heading, None, font_size, font_scale);
    draw_text(
      heading,
      x_pos + (self.width / 2.0) - (measure.width / 2.0),
      y_pos + 16.0,
      font_size as f32,
      DARKGRAY,
    );
  }
}

#[allow(dead_code)]
struct UiData {
  pub x_pos: f32,
  pub y_pos: f32,
  pub height: f32,
  pub width: f32,
}
impl UiData {
  pub fn new(x_pos: f32, y_pos: f32, height: f32, width: f32) -> Self {
    Self {
      x_pos,
      y_pos,
      height,
      width,
    }
  }
}
