use macroquad::prelude::*;
use std::cell::RefCell;

use crate::entity::{
  crafter::{CrafterData, draw_crafter},
  input::{InputData, draw_input},
  miner::{MinerData, draw_miner, update_miner},
  output::{OutputData, draw_output, update_output},
};
pub type EntityId = usize;

pub mod crafter;
pub mod input;
pub mod link;
pub mod miner;
pub mod output;

pub struct EntityManager {
  entities: Vec<RefCell<Entity>>,
}

impl EntityManager {
  pub fn new() -> Self {
    Self { entities: vec![] }
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
  Unknown,
  Miner,
  Output,
  Input,
  Crafter,
}

impl Default for EntityType {
  fn default() -> Self {
    EntityType::Unknown
  }
}

#[derive(Default)]
pub struct Entity {
  kind: EntityType,
  id: EntityId,
  parent_id: Option<EntityId>,
  miner_data: Option<MinerData>,
  crafter_data: Option<CrafterData>,
  output_data: Option<OutputData>,
  input_data: Option<InputData>,
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
      EntityType::Output => update_output(self, em),
      _ => {}
    }
  }

  fn draw(&self, em: &EntityManager) {
    match self.kind {
      EntityType::Miner => draw_miner(self),
      EntityType::Output => draw_output(self, em),
      EntityType::Crafter => draw_crafter(self),
      EntityType::Input => draw_input(self, em),
      EntityType::Unknown => {}
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

pub fn create_crafter(em: &mut EntityManager, crafter_data: CrafterData) -> EntityId {
  let crafter = Entity {
    kind: EntityType::Crafter,
    crafter_data: Some(crafter_data),
    ui_data: Some(UiData::new(400.0, 100.0, 100.0, 200.0)),
    ..Default::default()
  };
  let id = em.add_entity(crafter);
  let input_data = InputData::new(0.0, 50.0);
  let in_id = create_input(em, id, input_data);
  em.get_entity(id)
    .borrow_mut()
    .crafter_data
    .as_mut()
    .unwrap()
    .input = in_id;
  return id;
}

pub fn create_miner(em: &mut EntityManager, miner_data: MinerData) -> EntityId {
  let miner = Entity {
    kind: EntityType::Miner,
    miner_data: Some(miner_data),
    ui_data: Some(UiData::new(100.0, 100.0, 100.0, 200.0)),
    ..Default::default()
  };
  let id = em.add_entity(miner);

  let output_data = OutputData::new(200.0, 50.0);
  let out_id = create_output(em, id, output_data);
  em.get_entity(id)
    .borrow_mut()
    .miner_data
    .as_mut()
    .unwrap()
    .output = out_id;
  return id;
}
pub fn create_output(
  em: &mut EntityManager,
  parent: EntityId,
  output_data: OutputData,
) -> EntityId {
  let out = Entity {
    kind: EntityType::Output,
    output_data: Some(output_data),
    parent_id: Some(parent),
    ..Default::default()
  };
  return em.add_entity(out);
}
pub fn create_input(em: &mut EntityManager, parent: EntityId, input_data: InputData) -> EntityId {
  let out = Entity {
    kind: EntityType::Input,
    input_data: Some(input_data),
    parent_id: Some(parent),
    ..Default::default()
  };
  return em.add_entity(out);
}
