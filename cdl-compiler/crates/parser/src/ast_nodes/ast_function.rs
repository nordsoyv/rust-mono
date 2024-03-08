use anyhow::{Context, Result};
use serde::Serialize;
use std::rc::Rc;

use lexer::TokenKind;

use crate::{
  parse_expr::{parse_arg_list, parse_bracket_arg_list},
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug, Serialize, Clone)]
pub struct AstFunctionNode {
  pub name: Rc<str>,
  pub children: Vec<NodeRef>,
}

impl AstFunctionNode {}

impl Parsable for AstFunctionNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    let next_token = parser.get_next_token(1);
    if curr_token.is_err() || next_token.is_err() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if curr_token.kind == TokenKind::Identifier {
      let next_token = next_token.unwrap();
      if next_token.kind == TokenKind::ParenOpen {
        return true;
      }
      if next_token.kind == TokenKind::BracketOpen {
        return true;
      }
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let (ast_node, start_pos) = {
      let func_name_token = parser
        .get_current_token()
        .context("Error while parsing Function")?;

      let ast_node = AstFunctionNode {
        children: vec![],
        name: func_name_token.text.as_ref().unwrap().clone(),
      };
      (ast_node, func_name_token.pos.start)
    };
    //parser.start_group(format!("Function {:?}", &ast_node.name));
    let node_ref = parser.add_node(Node::Function(ast_node), start_pos..usize::MAX, parent);
    parser
      .eat_tokens(1)
      .context("Error while parsing Function")?;
    let is_paren = {
      let token = parser.get_current_token()?;
      parser.eat_token()?;
      if token.kind == TokenKind::ParenOpen {
        true
      } else {
        false
      }
    };

    let args = if is_paren {
      parse_arg_list(parser, node_ref)?
    } else {
      parse_bracket_arg_list(parser, node_ref)?
    };

    args
      .iter()
      .for_each(|a| parser.add_child_to_node(node_ref, *a));

    let end_pos = {
      if is_paren {
        parser.eat_token_of_type(TokenKind::ParenClose)?
      } else {
        parser.eat_token_of_type(TokenKind::BracketClose)?
      }
    };
    parser.update_location_on_node(node_ref, start_pos, end_pos.end);
    //  parser.end_group("");
    Ok(node_ref)
  }
}
