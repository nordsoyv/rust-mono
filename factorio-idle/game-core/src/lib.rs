extern crate cfg_if;
extern crate wasm_bindgen;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use crate::game::GameState;
use gloo_utils::format::JsValueSerdeExt;

mod utils;
mod game;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello,{}!", name));
}

// #[wasm_bindgen]
// pub fn tick(ms: i32) -> JsValue {
//     let next_state = game::tick(ms);
//     //  JsValue::from_serde
//     let a =JsValue::from_serde(&next_state).unwrap();
//     a
//    // serde_wasm_bindgen::to_value(&next_state).unwrap()
// }
//
// #[wasm_bindgen]
// pub fn init_game() {
//     game::init()
// }
