use anyhow::anyhow;
use anyhow::Result;
use std::rc::Rc;

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug)]
pub struct AstEntityNode {
  pub parent: NodeRef,
  pub children: Vec<NodeRef>,
  pub terms: Vec<Rc<str>>,
}

impl Parsable for AstEntityNode {
  fn can_parse(parser: &Parser) -> bool {
    let next_token = parser.get_current_token();
    if next_token.is_some() {
      let next_token = next_token.unwrap();
      if next_token.kind == TokenKind::Identifier {
        return true;
      }
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let header = AstEntityNode::parse_entity_header(parser)?;
    parser.eat_tokens(header.num_tokens);
    parser.eat_token_of_type(TokenKind::BraceOpen)?;
    parser.eat_token_of_type(TokenKind::EOL)?;

    let entity = AstEntityNode {
      children: vec![],
      parent,
      terms: header.terms,
    };
    let current_entity_ref = parser.add_node(Node::Entity(entity));
    loop {
      parser.eat_eol_and_comments();
      if AstEntityNode::can_parse(&parser) {
        let child_node_ref = AstEntityNode::parse(parser, current_entity_ref)?;
        //entity.children.push(child_node_ref);
        parser.add_child_to_node(current_entity_ref, child_node_ref);
        continue;
      }
      let curr_token = parser
        .get_current_token()
        .ok_or(anyhow!(format!("Unexpected EOF when parsing entity")))?;
      if curr_token.kind == TokenKind::BraceClose {
        parser.eat_token();
        //parser.add_node(Node::Entity(entity));
        return Ok(current_entity_ref);
      }
      return Err(anyhow!("Unexpected error while parsing entity"));
    }
  }
}

impl AstEntityNode {
  fn parse_entity_header(parser: &mut Parser) -> Result<EntityHeaderInfo> {
    let terms = parser.get_tokens_of_kind(TokenKind::Identifier);
    let terms = terms
      .into_iter()
      .map(|t| t.text.as_ref().unwrap().clone())
      .collect::<Vec<Rc<str>>>();

    return Ok(EntityHeaderInfo {
      num_tokens: terms.len(),
      terms,
    });
  }
}

struct EntityHeaderInfo {
  num_tokens: usize,
  terms: Vec<Rc<str>>,
}
