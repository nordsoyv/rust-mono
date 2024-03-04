use anyhow::{Result, Context};
use serde::Serialize;
use std::rc::Rc;

use cdl_lexer::TokenKind;

use crate::{
  parse_expr::parse_arg_list,
  parser::{Node, Parser},
  types::NodeRef,
};

use super::Parsable;

#[derive(Debug,Serialize)]
pub struct AstFunctionNode {
  pub name: Rc<str>,
  pub children: Vec<NodeRef>,
  pub parent: NodeRef,
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
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let (ast_node, start_pos) = {
      let func_name_token = parser.get_current_token().context("Error while parsing Function")?;

      let ast_node = AstFunctionNode {
        parent,
        children: vec![],
        name: func_name_token.text.as_ref().unwrap().clone(),
      };
      (ast_node, func_name_token.pos.start)
    };

    let node_ref = parser.add_node(Node::Function(ast_node),start_pos..usize::MAX);
    parser.eat_tokens(2).context("Error while parsing Function")?;

    let args = parse_arg_list(parser, node_ref)?;

    args
      .iter()
      .for_each(|a| parser.add_child_to_node(node_ref, *a));

    let end_pos = parser.eat_token_of_type(TokenKind::ParenClose)?;
    parser.update_location_on_node(node_ref, start_pos, end_pos.end);
    Ok(node_ref)
  }
}
