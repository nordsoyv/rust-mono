use anyhow::Result;
use ast::{AstIdentifierNode, AstNode, Node, NodeRef};
use lexer::TokenKind;

use crate::parser::Parser;

use super::Parsable;

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
      identifier: ident_token.text.as_ref().unwrap().clone(),
    };
    let node_ref = parser.add_node(
      AstNode::new(Node::Identifier(ast_node), parent),
      ident_token.pos.clone(),
    );
    parser.eat_tokens(1)?;
    Ok(node_ref)
  }
}
