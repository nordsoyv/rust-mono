use anyhow::Result;

use ast::{AstNode, AstStringNode, Node, NodeRef, QuoteKind};
use lexer::TokenKind;

use crate::parser::Parser;

use super::Parsable;

impl Parsable for AstStringNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    if curr_token.is_err() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if curr_token.kind == TokenKind::String {
      return true;
    }
    false
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let string_token = parser.get_current_token()?;
    let text = string_token.text.clone().unwrap();
    //parser.trace("Parsing String");
    let quote_kind = if text.0.starts_with('\'') {
      QuoteKind::SingleQuote
    } else {
      QuoteKind::DoubleQuote
    };
    let ast_node = AstStringNode { text, quote_kind };
    let node_ref = parser.add_node(
      AstNode::new(Node::String(ast_node), parent),
      string_token.pos.clone(),
    );
    parser.eat_tokens(1)?;
    Ok(node_ref)
  }
}
