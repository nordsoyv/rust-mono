use anyhow::{anyhow, Result};
use std::{rc::Rc, ops::Range};

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug)]
pub struct AstColorNode {
  pub color: Rc<str>,
  pub parent: NodeRef,
  pub location: Range<usize>
}

impl Parsable for AstColorNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    if curr_token.is_none() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if curr_token.kind == TokenKind::Color {
      return true;
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let color_token = parser
      .get_current_token()
      .ok_or(anyhow!("Got error unwraping token for color"))?;
    let ast_node = AstColorNode {
      parent,
      color: color_token.text.as_ref().unwrap().clone(),
      location: color_token.pos.clone()
    };
    let node_ref = parser.add_node(Node::Color(ast_node));
    parser.eat_tokens(1);
    Ok(node_ref)
  }
}
