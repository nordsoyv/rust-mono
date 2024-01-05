use anyhow::Result;
use std::rc::Rc;

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
    };
    let node_ref = parser.add_node(Node::Identifier(ast_node), ident_token.pos.clone());
    parser.eat_tokens(1)?;
    Ok(node_ref)
  }
}
