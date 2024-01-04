use anyhow::{anyhow, Result};
use std::{ops::Range, rc::Rc};

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug)]
pub struct AstTitleNode {
  pub title: Rc<str>,
  pub parent: NodeRef,
}

impl Parsable for AstTitleNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    let token_1 = parser.get_next_token(1);
    let token_2 = parser.get_next_token(2);

    if curr_token.is_err() || token_1.is_err() || token_2.is_err() {
      return false;
    }

    let curr_token = curr_token.unwrap();
    let token1 = token_1.unwrap();
    if token_2.unwrap().kind != TokenKind::EOL {
      return false;
    }
    if curr_token.kind == TokenKind::Identifier && curr_token.text == Some("title".into()) {
      if token1.kind == TokenKind::String {
        return true;
      }
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let title_keyword_token = parser.get_current_token()?;
    let title_token = parser.get_next_token(1)?;
    match &title_token.kind {
      TokenKind::String => {
        let ast_node = AstTitleNode {
          parent,
          title: title_token.text.as_ref().unwrap().clone(),
        };
        let node_ref = parser.add_node(Node::Title(ast_node),title_keyword_token.pos.start..title_token.pos.end);
        parser.eat_tokens(3)?;
        return Ok(node_ref);
      }
      _ => return Err(anyhow!("Unknown error occured while parsing Title node")),
    }
  }
}
