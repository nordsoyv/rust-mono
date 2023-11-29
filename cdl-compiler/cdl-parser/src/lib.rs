mod ast_nodes;
mod types;
mod parser;

use anyhow::Result;
use std::{cell::RefCell};
use cdl_lexer::lex;
use parser::{Parser, Node};
use types::NodeRef;

pub fn parse_text(text: &str) -> Result<Ast> {
  let tokens = lex(text)?;
  let mut parser = Parser {
    curr_token: 0,
    nodes: RefCell::new(Vec::new()),
    tokens: tokens,
  };
  let root_ref = parser.parse()?;

  Ok(Ast {
    nodes: parser.nodes.take(),
    script_entity: root_ref,
  })
}

#[derive(Debug)]
pub struct Ast {
  pub nodes: Vec<Node>,
  pub script_entity: NodeRef,
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_parse_title() {
    let ast = parse_text("title \"dashboard title\"\n");
    assert!(ast.is_ok());
    dbg!(&ast.unwrap());
  }
  #[test]
  fn can_parse_entity() {
    let ast = parse_text(r"maintype {

    }   
    ");
    dbg!(&ast);
    assert!(ast.is_ok());
    
  }
}
