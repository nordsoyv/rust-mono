use crate::entity::{EntityId, Item};

pub struct LinkData {
  input: EntityId,
  output: EntityId,
  buffer: Vec<Item>,
}

impl LinkData {
  pub fn new(input: EntityId, output: EntityId) -> Self {
    Self {
      input,
      output,
      buffer: vec![],
    }
  }
  pub fn has_room(&self) -> bool {
    self.buffer.len() < 5
  }

  pub fn push_item(&mut self, item: Item) {
    self.buffer.push(item);
  }

  pub fn has_item(&self) -> bool {
    self.buffer.len() > 0
  }
  pub fn pop_item(&mut self) -> Item {
    self.buffer.pop().unwrap()
  }
}
