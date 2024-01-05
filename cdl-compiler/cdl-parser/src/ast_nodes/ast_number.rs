use anyhow::{anyhow, Result};

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug)]
pub struct AstNumberNode {
  pub value: f64,
  pub parent: NodeRef,
}

impl Parsable for AstNumberNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    if curr_token.is_err() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if let TokenKind::Number(_num) = curr_token.kind {
      return true;
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let number_token = parser.get_current_token()?;
    let mut value = match number_token.kind {
      TokenKind::Number(num) => num,
      _ => return Err(anyhow!("Did not find number when trying to parse a number")),
    };
    parser.eat_token()?;
    let maybe_percent_token = parser.get_current_token()?;
    if maybe_percent_token.kind == TokenKind::Percent {
      value = value / 100.0;
      parser.eat_token()?;
    }
    let ast_node = AstNumberNode {
      parent,
      value,
    };
    let node_ref = parser.add_node(Node::Number(ast_node), number_token.pos.clone());    
    Ok(node_ref)
  }
}