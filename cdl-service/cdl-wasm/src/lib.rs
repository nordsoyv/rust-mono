mod utils;

use wasm_bindgen::prelude::*;
use cdl_core::lexer::{Lexer, Token, TokenType};
use cdl_core::parser::{Parser, parser_to_ast};
use serde_derive::{Deserialize, Serialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
  alert(&format!("hello {}!!!!!!!!!", s));
}


//#[wasm_bindgen]
//#[derive(Debug, Deserialize, Serialize)]
//struct LexResult {
//  pub error: String,
//  pub tokens: Vec<Token>,
//}

#[wasm_bindgen]
pub fn lex(s: &str) -> JsValue {
  let lexer = Lexer::new();
  let res = lexer.lex(s.to_string());
  match res {
    Err((err, tokens)) => {
      return JsValue::from_serde(&err).unwrap();
    }
    Ok(tokens) => {
      return JsValue::from_serde(&tokens).unwrap();
    }
  };
}


#[wasm_bindgen]
pub fn parse(s: &str) -> JsValue {
  let lexer = Lexer::new();
  let res_tokens = lexer.lex(s.to_string());
  let tokens = match res_tokens {
    Ok(t) =>t,
    Err(_) => return JsValue::from_str("error lexing")
  };
  let mut parser = Parser::new();
  parser.parse(tokens);
  let res = parser_to_ast(parser);
  let json =JsValue::from_serde(&res);
  match json {
    Ok(js) => return js,
    Err(_) => return JsValue::from_str("error parsing")
  }


//  match res {
//    Err(e) => {
//      return JsValue::from_serde("error").unwrap();
//    }
//    Ok(tokens) => {
//      return JsValue::from_serde(&tokens).unwrap();
//    }
//  };
}

