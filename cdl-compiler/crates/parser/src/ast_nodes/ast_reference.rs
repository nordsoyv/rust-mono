use anyhow::Result;
use ast::{AstNode, AstReferenceNode, Node, NodeRef};
use lexer::TokenKind;

use crate::parser::Parser;

use super::Parsable;

impl Parsable for AstReferenceNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    if curr_token.is_err() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if curr_token.kind == TokenKind::Reference {
      return true;
    }
    false
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let ref_token = parser.get_current_token()?;
    let ast_node = AstReferenceNode {
      ident: ref_token.text.as_ref().unwrap().clone(),
      resolved_node: NodeRef(-1).into(),
    };
    let node_ref = parser.add_node(
      AstNode::new(Node::Reference(ast_node), parent),
      ref_token.pos.clone(),
    );
    parser.eat_tokens(1)?;
    Ok(node_ref)
  }
}
