use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct GameState {
  pub counter : i32
}

impl GameState {
  pub fn new() -> GameState {
    GameState{
      counter: 0
    }
  }
}

static mut INSTANCE: OnceCell<GameState> = OnceCell::new();


unsafe fn get_mut_state() -> &'static mut GameState {
  INSTANCE.get_mut().unwrap()
}

pub fn tick(ms: i32) -> &'static GameState {
  let gs = unsafe {
    get_mut_state()
  };
  gs.counter += 1;
  return gs;
}

pub fn init(){
  unsafe {
    INSTANCE.set(GameState::new()).expect("Could not set initial GameState");
  }
}