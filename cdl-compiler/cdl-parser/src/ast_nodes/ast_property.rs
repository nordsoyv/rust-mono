use anyhow::{Result, bail};
use std::{ops::Range, rc::Rc};

use cdl_lexer::TokenKind;

use crate::{
  parse_expr::parse_list,
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug)]
pub struct AstPropertyNode {
  pub name: Rc<str>,
  pub parent: NodeRef,
  pub child: Vec<NodeRef>,
}

impl Parsable for AstPropertyNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    let token_1 = parser.get_next_token(1);
    if curr_token.is_err() || token_1.is_err() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    let token1 = token_1.unwrap();
    if curr_token.kind == TokenKind::Identifier && token1.kind == TokenKind::Colon {
      return true;
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let (node_ref, start_pos) = {
      let name_token = parser.get_current_token()?;
      let ast_node = AstPropertyNode {
        parent,
        name: name_token.text.as_ref().unwrap().clone(),
        child: vec![],
      };

      let node_ref = parser.add_node(Node::Property(ast_node),  name_token.pos.start..usize::MAX);
      (node_ref, name_token.pos.start)
    };
    parser.eat_tokens(2)?;
    let children = parse_list(parser, node_ref)?;
    let next_token = parser.get_current_token()?;

    let last_token_end = if next_token.kind == TokenKind::BraceClose {
      &next_token.pos
    }else if next_token.kind == TokenKind::EOL {
      parser.eat_token()?;
      &next_token.pos
    }else {
      bail!("Tried parsing property, did not find EOL when exptected");
    };
    // let last_token_end = parser
    //   .eat_token_of_type(TokenKind::EOL)
    //   .expect("Tried parsing property, did not find EOL when exptected");
    parser.update_location_on_node(node_ref, start_pos, last_token_end.end);
    children
      .iter()
      .for_each(|c| parser.add_child_to_node(node_ref, *c));
    //parser.add_child_to_node(node_ref, children);
    Ok(node_ref)
  }
}
