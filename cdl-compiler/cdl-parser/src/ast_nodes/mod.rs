use anyhow::anyhow;
use anyhow::Result;
use cdl_lexer::TokenKind;
use std::rc::Rc;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

pub trait Parsable {
  fn can_parse(parser: &Parser) -> Option<()>;
  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef>;
}

#[derive(Debug)]
pub struct AstTitleNode {
  pub node_ref: NodeRef,
  pub title: Rc<str>,
  pub parent: NodeRef,
}

impl Parsable for AstTitleNode {
  fn can_parse(parser: &Parser) -> Option<()> {
    let curr_token = parser.get_current_token()?;
    let token_1 = parser.get_next_token(1)?;
    let token_2 = parser.get_next_token(2)?;

    if curr_token.kind != TokenKind::Identifier("title".into()) {
      return None;
    }

     if let TokenKind::String(_title) = &token_1.kind  {
      return Some(());
     }

    if token_2.kind != TokenKind::EOL {
      return None;
    }
    return Some(());
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let title_token = parser
      .get_next_token(1)
      .ok_or(anyhow!("Got error unwraping token for title"))?;
    let node_ref = parser.get_next_node_ref();
    match &title_token.kind {
      TokenKind::String(title) => {
        let ast_node = AstTitleNode {
          node_ref,
          parent,
          title: title.clone(),
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

// impl Parsable for AstEntityNode {
//   fn try_parse(parser: &mut Parser, parent: NodeRef) -> Result<Option<NodeRef>> {
//     let header_info = AstEntityNode::try_parse_entity_header(parser)?;
//     let body = AstEntityNode::parse
//   }
// }

// impl AstEntityNode {
//   fn try_parse_entity_header(parser: &mut Parser) -> Option<EntityHeaderInfo> {
//     let curr_token = parser.get_current_token()?;
//     let token_1 = parser.get_next_token(1)?;
//     if let TokenKind::Identifier(main_type) = &token_1.kind {
//       if token_1.kind == TokenKind::BracketOpen {
//         parser.eat_tokens(2);
//         return Some(EntityHeaderInfo {
//           main_type: main_type.clone(),
//         });
//       }
//     }
//     return None;
//   }
// }

// struct EntityHeaderInfo {
//   main_type: Rc<str>,
// }
