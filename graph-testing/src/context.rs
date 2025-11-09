use crate::entity::EntityManager;

pub struct Context {
  pub entity_manager: EntityManager,
}

impl Context {
  pub fn new() -> Self {
    Self {
      entity_manager: EntityManager::new(),
    }
  }

  pub fn update(&mut self) {
    self.entity_manager.update();
  }
  pub fn draw(&self) {
    self.entity_manager.draw()
  }
}
