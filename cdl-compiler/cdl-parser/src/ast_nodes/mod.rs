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

    if curr_token.unwrap().kind != TokenKind::Identifier("title".into()) {
      return false;
    }
    if let TokenKind::String(_title) = &token_1.unwrap().kind {
      return true;
    }
    if token_2.unwrap().kind != TokenKind::EOL {
      return false;
    }
    return true;
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
//   fn can_parse(parser: &Parser) -> bool {
//     let curr_token = parser.get_current_token();
//     let token_1 = parser.get_next_token(1);
//     let token_2 = parser.get_next_token(2);
//     if curr_token.is_none() || token_1.is_none() || token_2.is_none() {
//       return false;
//     }
//     let curr_token = curr_token.unwrap();
//     let token_1 = token_1.unwrap();
//     let token_2 = token_2.unwrap();
//     if token_1.kind != TokenKind::BraceOpen {
//       return false;
//     }
//     if token_2.kind != TokenKind::EOL {
//       return false;
//     }
//     if let TokenKind::String(_main_type) = &curr_token.kind {
//       return true;
//     }
//     return false;
//   }

//   fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
//     let header = AstEntityNode::parse_entity_header(parser)?;

//     let body = AstEntityNode::parse_entity_body(parser)?;
//   }
// }

// impl AstEntityNode {
//   fn parse_entity_header(parser: &mut Parser) -> Result<EntityHeaderInfo> {
//     let curr_token = parser.get_current_token().unwrap();
//     match curr_token.kind {
//       TokenKind::Identifier(main_type) => {
//         parser.eat_tokens(3);
//         return Ok(EntityHeaderInfo {
//           main_type: main_type.clone(),
//         });
//       }
//       _ => {
//         return Err(anyhow!("Unexptected token while parsing entity header"));
//       }
//     }
//   }

//   fn parse_entity_body(parser: &mut Parser) -> Result {
//     todo!()
//   }
// }

struct EntityHeaderInfo {
  main_type: Rc<str>,
}
