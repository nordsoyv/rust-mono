use anyhow::{anyhow, Result};
use std::rc::Rc;

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::{ast_identifier::AstIdentifierNode, Parsable, ast_string::AstStringNode, ast_number::AstNumberNode};

#[derive(Debug)]
pub struct AstPropertyNode {
  pub name: Rc<str>,
  pub parent: NodeRef,
  pub child: NodeRef,
}

impl Parsable for AstPropertyNode {
  fn can_parse(parser: &Parser) -> bool {
    let curr_token = parser.get_current_token();
    let token_1 = parser.get_next_token(1);
    if curr_token.is_none() || token_1.is_none() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    let token1 = token_1.unwrap();
    if curr_token.kind == TokenKind::Identifier && token1.kind == TokenKind::Colon {
      return true;
    }
    return false;
  }

  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    let name_token = parser
      .get_current_token()
      .ok_or(anyhow!("Got error unwraping token for property name"))?;
    let ast_node = AstPropertyNode {
      parent,
      name: name_token.text.as_ref().unwrap().clone(),
      child: NodeRef(-1),
    };
    let node_ref = parser.add_node(Node::Property(ast_node));
    parser.eat_tokens(2);
    let expr_node_ref = AstPropertyNode::parse_expression(parser, node_ref)?;
    parser.add_child_to_node(node_ref, expr_node_ref);
    parser.eat_token_of_type(TokenKind::EOL).expect("Tried parsing property, did not find EOL when exptected");
    Ok(node_ref)
  }
}

impl AstPropertyNode {
  fn parse_expression(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    loop {
      //parser.eat_eol_and_comments();
      if AstIdentifierNode::can_parse(&parser) {
        let child_node_ref = AstIdentifierNode::parse(parser, parent)?;
        return Ok(child_node_ref);
      }
      if AstStringNode::can_parse(&parser) {
        let child_node_ref = AstStringNode::parse(parser, parent)?;
        return Ok(child_node_ref);
      }
      if AstNumberNode::can_parse(&parser) {
        let child_node_ref = AstNumberNode::parse(parser, parent)?;
        return Ok(child_node_ref);
      }
      return Err(anyhow!("Error parsing expression"));
    }
  }
}
