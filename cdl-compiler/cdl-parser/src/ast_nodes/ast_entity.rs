use anyhow::anyhow;
use anyhow::Result;
use std::ops::Range;
use std::rc::Rc;
use std::vec;

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::ast_property::AstPropertyNode;
use super::AstTableAliasNode;
use super::Parsable;

#[derive(Debug)]
pub struct AstEntityNode {
  pub parent: NodeRef,
  pub children: Vec<NodeRef>,
  pub terms: Vec<Rc<str>>,
  pub location: Range<usize>,
  pub label: Option<Rc<str>>,
  pub refs: Vec<Rc<str>>,
  pub ident: Option<Rc<str>>,
  pub entity_number: Option<f64>,
}

impl Parsable for AstEntityNode {
  fn can_parse(parser: &Parser) -> bool {
    let next_token = parser.get_current_token();
    if next_token.is_ok() {
      let next_token = next_token.unwrap();
      if next_token.kind == TokenKind::Identifier {
        return true;
      }
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let header = AstEntityNode::parse_entity_header(parser)?;
    let is_config_hub = if header.terms.len() > 1
      && header.terms[0] == "config".into()
      && header.terms[1] == "hub".into()
    {
      true
    } else {
      false
    };

    let entity = AstEntityNode {
      children: vec![],
      parent,
      terms: header.terms,
      location: header.start_loc..0,
      label: header.label,
      refs: header.refs,
      ident: header.ident,
      entity_number: header.entity_number,
    };
    let current_entity_ref = parser.add_node(Node::Entity(entity));

    let next_token = parser.get_current_token()?;
    if next_token.kind == TokenKind::EOL {
      return Ok(current_entity_ref);
    }
    parser.eat_token_of_type(TokenKind::BraceOpen)?;
    loop {
      parser.eat_eol_and_comments();
      if AstPropertyNode::can_parse(&parser) {
        let child_node_ref = AstPropertyNode::parse(parser, current_entity_ref)?;
        parser.add_child_to_node(current_entity_ref, child_node_ref);
        continue;
      }
      if AstTableAliasNode::can_parse(&parser) {
        if !is_config_hub {
          return Err(anyhow!("Table Alias not allowed outside config hub"));
        }
        let child_node_ref = AstTableAliasNode::parse(parser, current_entity_ref)?;
        parser.add_child_to_node(current_entity_ref, child_node_ref);
        continue;
      }
      if AstEntityNode::can_parse(&parser) {
        let child_node_ref = AstEntityNode::parse(parser, current_entity_ref)?;
        parser.add_child_to_node(current_entity_ref, child_node_ref);
        continue;
      }

      let curr_token = parser.get_current_token()?;
      if curr_token.kind == TokenKind::BraceClose {
        parser.eat_token()?;
        parser.update_location_on_node(current_entity_ref, header.start_loc, curr_token.pos.end);
        return Ok(current_entity_ref);
      }
      return Err(anyhow!("Unexpected error while parsing entity"));
    }
  }
}

impl AstEntityNode {
  fn parse_entity_header(parser: &mut Parser) -> Result<EntityHeaderInfo> {
    let terms = parser.get_tokens_of_kind(TokenKind::Identifier);
    let start_loc = terms[0].pos.start;
    let terms = terms
      .into_iter()
      .map(|t| t.text.as_ref().unwrap().clone())
      .collect::<Vec<Rc<str>>>();
    parser.eat_tokens(terms.len())?;

    let label_token = parser.get_tokens_of_kind(TokenKind::String);
    let label = if label_token.len() > 0 {
      parser.eat_token()?;
      label_token[0].text.clone()
    } else {
      None
    };

    let ref_tokens = parser.get_tokens_of_kind(TokenKind::Reference);
    let refs = if ref_tokens.len() > 0 {
      parser.eat_tokens(ref_tokens.len())?;
      ref_tokens.iter().map(|r| r.text.clone().unwrap()).collect()
    } else {
      vec![]
    };

    let ident = if parser.is_next_token_of_type(TokenKind::Hash) {
      let ident_token = parser.get_next_token(1);
      if ident_token.is_ok() {
        parser.eat_tokens(2)?;
        ident_token.unwrap().text.clone()
      } else {
        None
      }
    } else {
      None
    };

    let entity_number = {
      let next_token = parser.get_current_token();
      if next_token.is_ok() {
        let next_token = next_token.unwrap();
        if let TokenKind::Number(entity_number) = next_token.kind {
          parser.eat_token()?;
          Some(entity_number)
        } else {
          None
        }
      } else {
        None
      }
    };

    return Ok(EntityHeaderInfo {
      terms,
      start_loc,
      label,
      refs,
      ident,
      entity_number,
    });
  }
}

#[derive(Debug)]
struct EntityHeaderInfo {
  terms: Vec<Rc<str>>,
  start_loc: usize,
  label: Option<Rc<str>>,
  refs: Vec<Rc<str>>,
  ident: Option<Rc<str>>,
  entity_number: Option<f64>,
}
