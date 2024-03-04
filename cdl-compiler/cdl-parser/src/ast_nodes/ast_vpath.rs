use anyhow::{anyhow, Result};
use serde::Serialize;
use std::rc::Rc;

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug,Serialize)]
pub struct AstVPathNode {
  pub parent: NodeRef,
  pub table: Option<Rc<str>>,
  pub variable: Option<Rc<str>>,
}

impl Parsable for AstVPathNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    let token_1 = parser.get_next_token(1);
    if curr_token.is_err() {
      return false;
    }

    let curr_token = curr_token.unwrap();
    if curr_token.kind == TokenKind::Colon {
      return true;
    }
    let token1 = token_1.unwrap();
    if curr_token.kind == TokenKind::Identifier {
      if token1.kind == TokenKind::Colon {
        return true;
      }
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let first_token = parser.get_current_token()?;
    let second_token = parser.get_next_token(1)?;
    let third_token = parser.get_next_token(2)?;

    let (ast_node, pos) = match (&first_token.kind, &second_token.kind, &third_token.kind) {
      (TokenKind::Identifier, TokenKind::Colon, TokenKind::Identifier) => {
        parser.eat_tokens(3)?;
        (
          AstVPathNode {
            parent,
            table: first_token.text.clone(),
            variable: third_token.text.clone(),
          },
          first_token.pos.start..third_token.pos.end,
        )
      }
      (TokenKind::Identifier, TokenKind::Colon, _) => {
        parser.eat_tokens(2)?;
        (
          AstVPathNode {
            parent,
            table: first_token.text.clone(),
            variable: None,
          },
          first_token.pos.start..second_token.pos.end,
        )
      }
      (TokenKind::Colon, TokenKind::Identifier, _) => {
        parser.eat_tokens(2)?;
        (
          AstVPathNode {
            parent,
            table: None,
            variable: second_token.text.clone(),
          },
          first_token.pos.start..second_token.pos.end,
        )
      }
      (TokenKind::Colon, _, _) => {
        parser.eat_tokens(1)?;
        (
          AstVPathNode {
            parent,
            table: None,
            variable: None,
          },
          first_token.pos.clone(),
        )
      }
      (_, _, _) => return Err(anyhow!("Unknown error occured while parsing VPath node")),
    };
    let node_ref = parser.add_node(Node::VPath(ast_node), pos);
    return Ok(node_ref);
  }
}
