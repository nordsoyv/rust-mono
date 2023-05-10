use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]
#[wasm_bindgen]
pub struct GameState {
  pub counter : i32
}


#[wasm_bindgen]
impl GameState {
  pub fn new() -> GameState {
    GameState{
      counter: 0
    }
  }
  pub fn tick(&mut self ,ms: i32) {
    self.counter += 1;
  }

}