use anyhow::Result;

use ast::{AstColorNode, AstNode, Node, NodeRef};
use lexer::TokenKind;

use crate::parser::Parser;

use super::Parsable;

impl Parsable for AstColorNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    if curr_token.is_err() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if curr_token.kind == TokenKind::Color {
      return true;
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let color_token = parser.get_current_token()?;
    let ast_node = AstNode::new(
      Node::Color(AstColorNode {
        color: color_token.text.as_ref().unwrap().clone(),
      }),
      parent,
    );
    let node_ref = parser.add_node(ast_node, color_token.pos.clone());
    parser.eat_tokens(1)?;
    Ok(node_ref)
  }
}
