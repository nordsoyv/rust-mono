use anyhow::Result;
use serde::Serialize;
use std::rc::Rc;

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;
#[derive(Debug,Serialize,Clone)]
pub enum QuoteKind {
  SingleQuote,
  DoubleQuote,
}

#[derive(Debug,Serialize,Clone)]
pub struct AstStringNode {
  pub text: Rc<str>,
  pub parent: NodeRef,
  pub quote_kind: QuoteKind,
}

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
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let string_token = parser.get_current_token()?;
    let text = string_token.text.clone().unwrap();
//    parser.trace(&format!("Parsing String \"{:?}\"", text ));
    let quote_kind = if text.starts_with("'") {
      QuoteKind::SingleQuote
    } else {
      QuoteKind::DoubleQuote
    };
    let ast_node = AstStringNode {
      parent,
      text,
      quote_kind,
    };
    let node_ref = parser.add_node(Node::String(ast_node), string_token.pos.clone());
    parser.eat_tokens(1)?;
    Ok(node_ref)
  }
}
