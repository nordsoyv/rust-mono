use anyhow::{anyhow, Result};
use std::{rc::Rc, ops::Range};

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug)]
pub enum QuoteKind {
  SingleQuote,
  DoubleQuote,
}

#[derive(Debug)]
pub struct AstStringNode {
  pub text: Rc<str>,
  pub parent: NodeRef,
  pub quote_kind: QuoteKind,
  pub location: Range<usize>
}

impl Parsable for AstStringNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    if curr_token.is_none() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if curr_token.kind == TokenKind::String {
      return true;
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let string_token = parser
      .get_current_token()
      .ok_or(anyhow!("Got error unwraping token for string"))?;
    let text = string_token.text.clone().unwrap();
    let quote_kind = if text.starts_with("'") {
      QuoteKind::SingleQuote
    } else {
      QuoteKind::DoubleQuote
    };
    let ast_node = AstStringNode {
      parent,
      text,
      quote_kind,
      location: string_token.pos.clone()
    };
    let node_ref = parser.add_node(Node::String(ast_node));
    parser.eat_tokens(1);
    Ok(node_ref)
  }
}
