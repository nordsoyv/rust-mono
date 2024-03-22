use crate::parser::Parser;

use super::Parsable;
use anyhow::anyhow;
use anyhow::Result;
use ast::AstEntityNode;
use ast::AstNode;
use ast::AstPropertyNode;
use ast::AstTableAliasNode;
use ast::Node;
use ast::NodeRef;
use lexer::LexedStr;
use lexer::TokenKind;
use log::trace;

#[derive(Debug)]
struct EntityHeaderInfo {
  terms: Vec<LexedStr>,
  start_loc: usize,
  label: Option<LexedStr>,
  refs: Vec<LexedStr>,
  ident: Option<LexedStr>,
  entity_number: Option<f64>,
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
    false
  }

  #[tracing::instrument(name = "parse_entity", skip(parser, parent))]
  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let header = parse_entity_header(parser)?;
    let entity = AstEntityNode {
      children: vec![].into(),
      terms: header.terms,
      label: header.label,
      refs: header.refs,
      ident: header.ident,
      entity_number: header.entity_number,
    };
    let ast_node = AstNode::new(Node::Entity(entity), parent);
    let current_entity_ref = parser.add_node(ast_node, header.start_loc..usize::MAX);
    trace!("current_entity_ref {:?}", current_entity_ref);
    let next_token = parser.get_current_token()?;
    if next_token.kind == TokenKind::EOL {
      return Ok(current_entity_ref);
    }
    parser.eat_token_of_type(TokenKind::BraceOpen)?;
    loop {
      parser.eat_eol_and_comments();
      if AstPropertyNode::can_parse(parser) {
        let child_node_ref = AstPropertyNode::parse(parser, current_entity_ref)?;
        parser.add_child_to_node(current_entity_ref, child_node_ref);
        continue;
      }
      if AstTableAliasNode::can_parse(parser) {
        // if !is_config_hub {
        //   return Err(anyhow!("Table Alias not allowed outside config hub"));
        // }
        let child_node_ref = AstTableAliasNode::parse(parser, current_entity_ref)?;
        parser.add_child_to_node(current_entity_ref, child_node_ref);
        continue;
      }
      if AstEntityNode::can_parse(parser) {
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

pub fn can_parse_anonymous_entity(parser: &Parser) -> bool {
  let next_token = parser.get_current_token();
  if next_token.is_ok() {
    let next_token = next_token.unwrap();
    if next_token.kind == TokenKind::BraceOpen {
      return true;
    }
  }
  false
}

pub fn parse_anonymous_entity(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
  let open_brace_token_pos = {
    let open_brace_token = parser.get_current_token()?;
    open_brace_token.pos.start
  };
  let entity = AstEntityNode {
    children: vec![].into(),
    terms: vec![],
    label: None,
    refs: vec![],
    ident: None,
    entity_number: None,
  };
  let current_entity_ref = parser.add_node(
    AstNode::new(Node::Entity(entity), parent),
    open_brace_token_pos..usize::MAX,
  );

  // let next_token = parser.get_current_token()?;
  // if next_token.kind == TokenKind::EOL {
  //   return Ok(current_entity_ref);
  // }

  parser.eat_token_of_type(TokenKind::BraceOpen)?;
  loop {
    parser.eat_eol_and_comments();
    if AstPropertyNode::can_parse(parser) {
      let child_node_ref = AstPropertyNode::parse(parser, current_entity_ref)?;
      parser.add_child_to_node(current_entity_ref, child_node_ref);
      continue;
    }
    if AstEntityNode::can_parse(parser) {
      let child_node_ref = AstEntityNode::parse(parser, current_entity_ref)?;
      parser.add_child_to_node(current_entity_ref, child_node_ref);
      continue;
    }

    let curr_token = parser.get_current_token()?;
    if curr_token.kind == TokenKind::BraceClose {
      parser.eat_token()?;
      parser.update_location_on_node(current_entity_ref, open_brace_token_pos, curr_token.pos.end);
      return Ok(current_entity_ref);
    }
    return Err(anyhow!("Unexpected error while parsing entity"));
  }
}

fn parse_entity_header(parser: &mut Parser) -> Result<EntityHeaderInfo> {
  let terms = parser.get_tokens_of_kind(TokenKind::Identifier);
  let start_loc = terms[0].pos.start;
  let terms = terms
    .iter()
    .map(|t| t.text.as_ref().unwrap().clone())
    .collect::<Vec<LexedStr>>();
  parser.eat_tokens(terms.len())?;

  let label_token = parser.get_tokens_of_kind(TokenKind::String);
  let label = if !label_token.is_empty() {
    parser.eat_token()?;
    label_token[0].text.clone()
  } else {
    None
  };

  let ref_tokens = parser.get_tokens_of_kind(TokenKind::Reference);
  let refs = if !ref_tokens.is_empty() {
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

  Ok(EntityHeaderInfo {
    terms,
    start_loc,
    label,
    refs,
    ident,
    entity_number,
  })
}
