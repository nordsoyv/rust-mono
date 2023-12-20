use anyhow::{anyhow, Result};
use std::rc::Rc;

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug)]
pub struct AstVPathNode {
  pub parent: NodeRef,
  pub table: Rc<str>,
  pub variable: Option<Rc<str>>,
}

impl Parsable for AstVPathNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    let token_1 = parser.get_next_token(1);
    if curr_token.is_none() || token_1.is_none() {
      return false;
    }

    let curr_token = curr_token.unwrap();
    let token1 = token_1.unwrap();
    if curr_token.kind == TokenKind::Identifier {
      if token1.kind == TokenKind::Colon {
        return true;
      }
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let table_token = parser
      .get_next_token(0)
      .ok_or(anyhow!("Got error unwraping token for VPath"))?;
    let variable = {
      let variable_token = parser.get_next_token(2);
      if variable_token.is_some() {
        let v = variable_token.unwrap();
        dbg!(&v);
        match &v.kind {
          TokenKind::Identifier => v.text.clone(),
          _ => None
        }
      } else {
        None
      }
    };

    let table = match &table_token.kind {
      TokenKind::Identifier => table_token.text.clone().unwrap(),
      _ => return Err(anyhow!("Unknown error occured while parsing VPath node")),
    };
    parser.eat_tokens(2);
    if variable.is_some() {
      parser.eat_token();
    }
    let ast_node = AstVPathNode {
      parent,
      table,
      variable,
    };
    let node_ref = parser.add_node(Node::VPath(ast_node));
    return Ok(node_ref);
  }
}
