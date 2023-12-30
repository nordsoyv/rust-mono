use anyhow::{anyhow, Result};
use std::{ ops::Range, rc::Rc};

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug)]
pub struct AstTableAliasNode {
  pub table: Rc<str>,
  pub alias: Rc<str>,
  pub parent: NodeRef,
  pub location: Range<usize>,
}

impl Parsable for AstTableAliasNode {
  fn can_parse(parser: &Parser) -> bool {
    let table_token = parser.get_current_token();
    let alias_token = parser.get_next_token(1);
    let equal_token = parser.get_next_token(2);
    if alias_token.is_none() || equal_token.is_none() || table_token.is_none() {
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
    let table_token = parser
      .get_current_token()
      .ok_or(anyhow!("Got error unwraping token for table alias"))?;
    let alias_token = parser
      .get_next_token(1)
      .ok_or(anyhow!("Got error unwraping token for table alias"))?;
    let _ = parser
      .get_next_token(2)
      .ok_or(anyhow!("Got error unwraping token for table alias"))?;
    let vpath_token = parser
      .get_next_token(3)
      .ok_or(anyhow!("Got error unwraping token for table alias"))?;

    let ast_node = AstTableAliasNode {
      parent,
      alias: alias_token.text.as_ref().unwrap().clone(),
      table: vpath_token.text.as_ref().unwrap().clone(),
      location: table_token.pos.start..vpath_token.pos.end,
    };
    let node_ref = parser.add_node(Node::TableAlias(ast_node));
    parser.eat_tokens(4);
    Ok(node_ref)
  }
}
