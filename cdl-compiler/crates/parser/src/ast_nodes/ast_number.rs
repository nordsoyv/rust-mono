use anyhow::{anyhow, Result};
use ast::{AstNode, AstNumberNode, Node, NodeRef};
use lexer::TokenKind;
use crate::parser::Parser;
use super::Parsable;

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
    false
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
      value /= 100.0;
      parser.eat_token()?;
    }
    let ast_node = AstNumberNode { value };
    let node_ref = parser.add_node(
      AstNode::new(Node::Number(ast_node), parent),
      number_token.pos.clone(),
    );
    Ok(node_ref)
  }
}
