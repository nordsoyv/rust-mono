use anyhow::Result;
use std::{ops::Range, rc::Rc};

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug)]
pub struct AstIdentifierNode {
  pub identifier: Rc<str>,
  pub parent: NodeRef,
  pub location: Range<usize>,
}

impl Parsable for AstIdentifierNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    if curr_token.is_err() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if curr_token.kind == TokenKind::Identifier {
      return true;
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let ident_token = parser.get_current_token()?;
    let ast_node = AstIdentifierNode {
      parent,
      identifier: ident_token.text.as_ref().unwrap().clone(),
      location: ident_token.pos.clone(),
    };
    let node_ref = parser.add_node(Node::Identifier(ast_node));
    parser.eat_tokens(1)?;
    Ok(node_ref)
  }
}
