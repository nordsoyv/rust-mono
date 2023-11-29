use anyhow::anyhow;
use anyhow::Result;
use cdl_lexer::TokenKind;
use std::rc::Rc;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

pub trait Parsable {
  fn can_parse(parser: &Parser) -> bool;
  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef>;
}

#[derive(Debug)]
pub struct AstTitleNode {
  pub node_ref: NodeRef,
  pub title: Rc<str>,
  pub parent: NodeRef,
}

impl Parsable for AstTitleNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    let token_1 = parser.get_next_token(1);
    let token_2 = parser.get_next_token(2);

    if curr_token.is_none() || token_1.is_none() || token_2.is_none() {
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
    let title_token = parser
      .get_next_token(1)
      .ok_or(anyhow!("Got error unwraping token for title"))?;
    let node_ref = parser.get_next_node_ref();
    match &title_token.kind {
      TokenKind::String => {
        let ast_node = AstTitleNode {
          node_ref,
          parent,
          title: title_token.text.as_ref().unwrap().clone(),
        };
        parser.add_node(Node::Title(ast_node));
        parser.eat_tokens(3);
        return Ok(node_ref);
      }
      _ => return Err(anyhow!("Unknown error occured while parsing Title node")),
    }
  }
}

#[derive(Debug)]
pub struct AstEntityNode {
  pub node_ref: NodeRef,
  pub parent: NodeRef,
  pub children: Vec<NodeRef>,
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

    let node_ref = parser.get_next_node_ref();
    let mut entity = AstEntityNode {
      children: vec![],
      node_ref,
      parent
    };

    loop {
      let curr_token = parser.get_current_token().ok_or(anyhow!(format!("Unexpected EOF when parsing entity")))?;
      if curr_token.kind == TokenKind::EOL {
        parser.eat_token();
        continue;
      }
      if curr_token.kind == TokenKind::LineComment {
        parser.eat_token();
        continue;
      }
      if curr_token.kind == TokenKind::MultiLineComment {
        parser.eat_token();
        continue;
      }
      if curr_token.kind == TokenKind::BraceClose {
        parser.eat_token();
        parser.add_node(Node::Entity(entity));
        return Ok(node_ref);
      }
      return Err(anyhow!("Unexpected error while parsing entity"));
    }
  
  }
}

impl AstEntityNode {
  fn parse_entity_header(parser: &mut Parser) -> Result<EntityHeaderInfo> {
    let terms = parser.get_tokens_of_kind(TokenKind::Identifier);
    let terms = terms.into_iter().map(|t| t.text.as_ref().unwrap().clone()).collect::<Vec<Rc<str>>>();

    return Ok(EntityHeaderInfo {
      num_tokens: terms.len(),
      terms,
    });
  }
}

//   fn parse_entity_body(parser: &mut Parser) -> Result {
//     todo!()
//   }
// }

struct EntityHeaderInfo {
  num_tokens: usize,
  terms: Vec<Rc<str>>,
}
