use anyhow::Result;
use cdl_lexer::TokenKind;
use serde::Serialize;
use std::rc::Rc;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::{AstVPathNode, Parsable};

#[derive(Debug,Serialize,Clone)]
pub struct AstTableAliasNode {
  pub table: Rc<str>,
  pub alias: Rc<str>,
  pub parent: NodeRef,
}

impl Parsable for AstTableAliasNode {
  fn can_parse(parser: &Parser) -> bool {
    let table_token = parser.get_current_token();
    let alias_token = parser.get_next_token(1);
    let equal_token = parser.get_next_token(2);
    if alias_token.is_err() || equal_token.is_err() || table_token.is_err() {
      return false;
    }
    let table_token = table_token.unwrap();
    if table_token.kind == TokenKind::Identifier {
      if table_token.text != Some("table".into()) {
        return false;
      }
    }
    let equal_token = equal_token.unwrap();
    let alias_token = alias_token.unwrap();
    match (&alias_token.kind, &equal_token.kind) {
      (TokenKind::Identifier, TokenKind::Equal) => return true,
      (_, _) => return false,
    }
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let table_token = parser.get_current_token()?;
    let alias_token = parser.get_next_token(1)?;
    let _ = parser.get_next_token(2)?;
    parser.eat_tokens(3)?;
    let vpath_token = parser.get_current_token()?;

    let ast_node = AstTableAliasNode {
      parent,
      alias: alias_token.text.as_ref().unwrap().clone(),
      table: vpath_token.text.as_ref().unwrap().clone(),
    };
    let node_ref = parser.add_node(
      Node::TableAlias(ast_node),
      table_token.pos.start..vpath_token.pos.end,
    );
    parser.eat_tokens(1)?;
    if parser.is_next_token_of_type(TokenKind::Colon) {
      parser.eat_token()?;
    }
    Ok(node_ref)
  }
}
