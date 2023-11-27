use std::rc::Rc;

use cdl_lexer::TokenKind;

use crate::{parser::{Parser, Node}, types::NodeRef};

#[derive(Debug)]
pub struct AstTitleNode {
  pub node_ref: NodeRef,
  pub title: Rc<str>,
  pub parent: NodeRef,
}

impl AstTitleNode {
  pub fn try_parse(parser: &mut Parser, parent: NodeRef) -> Option<NodeRef> {
    let curr_token = parser.get_current_token()?;
    let token_1 = parser.get_next_token(1)?;
    let token_2 = parser.get_next_token(2)?;

    if curr_token.kind != TokenKind::Identifier("title".into()) {
      return None;
    }
    if let TokenKind::String(title) = &token_1.kind {
      if token_2.kind == TokenKind::EOL {
        let node_ref = parser.get_next_node_ref();
        let ast_node = AstTitleNode {
          node_ref,
          parent,
          title: title.clone(),
        };
        parser.add_node(Node::Title(ast_node));
        parser.eat_tokens(3);
        return Some(node_ref);
      }
    }
    return None;
  }
}

#[derive(Debug)]
pub struct AstEntityNode {
  pub node_ref: NodeRef,
  pub parent: NodeRef,
  pub children: Vec<NodeRef>,
}
