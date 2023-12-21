use std::ops::Range;

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
  pub location: Range<usize>
}

impl Parsable for AstNumberNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    if curr_token.is_none() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if let TokenKind::Number(_num) = curr_token.kind {
      return true;
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let number_token = parser
      .get_current_token()
      .ok_or(anyhow!("Got error unwraping token for number"))?;
    let value = match number_token.kind {
      TokenKind::Number(num) => num,
      _ => return Err(anyhow!("Did not find number when trying to parse a number")),
    };
    let ast_node = AstNumberNode {
      parent,
      value,
      location: number_token.pos.clone()
    };
    let node_ref = parser.add_node(Node::Number(ast_node));
    parser.eat_tokens(1);
    Ok(node_ref)
  }
}
