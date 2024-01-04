use std::ops::Range;

use anyhow::{anyhow, Result};

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug)]
pub struct AstBooleanNode {
  pub value: bool,
  pub parent: NodeRef,
  pub location: Range<usize>,
}

impl Parsable for AstBooleanNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    if curr_token.is_err() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if let TokenKind::Boolean(_num) = curr_token.kind {
      return true;
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let bool_token = parser.get_current_token()?;
    let value = match bool_token.kind {
      TokenKind::Boolean(b) => b,
      _ => return Err(anyhow!("Did not find number when trying to parse a number")),
    };
    let ast_node = AstBooleanNode {
      parent,
      value,
      location: bool_token.pos.clone(),
    };
    let node_ref = parser.add_node(Node::Boolean(ast_node));
    parser.eat_tokens(1)?;
    Ok(node_ref)
  }
}
