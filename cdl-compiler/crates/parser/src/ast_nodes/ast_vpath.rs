use anyhow::{anyhow, Result};
use ast::{AstNode, AstVPathNode, Node, NodeRef};
use lexer::TokenKind;

use crate::parser::Parser;

use super::Parsable;

impl Parsable for AstVPathNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    let token_1 = parser.get_next_token(1);
    if curr_token.is_err() {
      return false;
    }

    let curr_token = curr_token.unwrap();
    if curr_token.kind == TokenKind::Colon {
      return true;
    }
    let token1 = token_1.unwrap();
    if curr_token.kind == TokenKind::Identifier && token1.kind == TokenKind::Colon {
      return true;
    }
    false
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let first_token = parser.get_current_token()?;
    let second_token = parser.get_next_token(1)?;
    let third_token = parser.get_next_token(2)?;
    let fourth_token = parser.get_next_token(3)?;

    let (ast_node, pos) = match (
      &first_token.kind,
      &second_token.kind,
      &third_token.kind,
      &fourth_token.kind,
    ) {
      (TokenKind::Identifier, TokenKind::Colon, TokenKind::Identifier, TokenKind::ParenOpen) => {
        parser.eat_tokens(4)?;
        let parent_close_pos = parser.eat_token_of_type(TokenKind::ParenClose)?;
        (
          AstVPathNode {
            table: first_token.text.clone(),
            variable: None,
            function: third_token.text.clone(),
            is_hierarchy: false,
          },
          first_token.pos.start..parent_close_pos.end,
        )
      }
      (TokenKind::Identifier, TokenKind::Colon, TokenKind::Identifier, _) => {
        parser.eat_tokens(3)?;
        (
          AstVPathNode {
            table: first_token.text.clone(),
            variable: third_token.text.clone(),
            function: None,
            is_hierarchy: false,
          },
          first_token.pos.start..third_token.pos.end,
        )
      }
      (TokenKind::Identifier, TokenKind::Colon, TokenKind::HierarchyReference, _) => {
        parser.eat_tokens(3)?;
        (
          AstVPathNode {
            table: first_token.text.clone(),
            variable: third_token.text.clone(),
            function: None,
            is_hierarchy: true,
          },
          first_token.pos.start..third_token.pos.end,
        )
      }
      (TokenKind::Identifier, TokenKind::Colon, _, _) => {
        parser.eat_tokens(2)?;
        (
          AstVPathNode {
            table: first_token.text.clone(),
            variable: None,
            function: None,
            is_hierarchy: false,
          },
          first_token.pos.start..second_token.pos.end,
        )
      }
      (TokenKind::Colon, TokenKind::Identifier, TokenKind::ParenOpen, _) => {
        parser.eat_tokens(3)?;
        let parent_close_pos = parser.eat_token_of_type(TokenKind::ParenClose)?;
        (
          AstVPathNode {
            table: None,
            variable: None,
            function: second_token.text.clone(),
            is_hierarchy: false,
          },
          first_token.pos.start..parent_close_pos.end,
        )
      }
      (TokenKind::Colon, TokenKind::Identifier, _, _) => {
        parser.eat_tokens(2)?;
        (
          AstVPathNode {
            table: None,
            variable: second_token.text.clone(),
            function: None,
            is_hierarchy: false,
          },
          first_token.pos.start..second_token.pos.end,
        )
      }
      (TokenKind::Colon, TokenKind::HierarchyReference, _, _) => {
        parser.eat_tokens(2)?;
        (
          AstVPathNode {
            table: None,
            variable: second_token.text.clone(),
            function: None,
            is_hierarchy: true,
          },
          first_token.pos.start..second_token.pos.end,
        )
      }
      (TokenKind::Colon, _, _, _) => {
        parser.eat_tokens(1)?;
        (
          AstVPathNode {
            table: None,
            variable: None,
            function: None,
            is_hierarchy: false,
          },
          first_token.pos.clone(),
        )
      }
      (_, _, _, _) => return Err(anyhow!("Unknown error occurred while parsing VPath node")),
    };
    let node_ref = parser.add_node(AstNode::new(Node::VPath(ast_node), parent), pos);
    Ok(node_ref)
  }
}
