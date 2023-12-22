use anyhow::{anyhow, Result};
use std::{ops::Range, rc::Rc};

use cdl_lexer::TokenKind;

use crate::{
  parser::{Node, Parser},
  types::NodeRef,
};

use super::{
  ast_identifier::AstIdentifierNode, ast_number::AstNumberNode, ast_string::AstStringNode,
  ast_vpath::AstVPathNode, Parsable, AstColorNode,
};

#[derive(Debug)]
pub struct AstPropertyNode {
  pub name: Rc<str>,
  pub parent: NodeRef,
  pub child: NodeRef,
  pub location: Range<usize>,
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
    let (node_ref, start_pos) = {
      let name_token = parser
        .get_current_token()
        .ok_or(anyhow!("Got error unwraping token for property name"))?;
      let ast_node = AstPropertyNode {
        parent,
        name: name_token.text.as_ref().unwrap().clone(),
        child: NodeRef(-1),
        location: name_token.pos.start..usize::MAX,
      };

      let node_ref = parser.add_node(Node::Property(ast_node));
      (node_ref, name_token.pos.start)
    };
    parser.eat_tokens(2);
    let expr_node_ref = AstPropertyNode::parse_expression(parser, node_ref)?;
    let last_token_end = parser
      .eat_token_of_type(TokenKind::EOL)
      .expect("Tried parsing property, did not find EOL when exptected");
    parser.update_location_on_node(node_ref, start_pos, last_token_end);
    parser.add_child_to_node(node_ref, expr_node_ref);
    Ok(node_ref)
  }
}

impl AstPropertyNode {
  fn parse_expression(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
    loop {
      //parser.eat_eol_and_comments();
      if AstVPathNode::can_parse(&parser) {
        return AstVPathNode::parse(parser, parent);
      }
      if AstIdentifierNode::can_parse(&parser) {
        return AstIdentifierNode::parse(parser, parent);
      }
      if AstStringNode::can_parse(&parser) {
        return AstStringNode::parse(parser, parent);
      }
      if AstNumberNode::can_parse(&parser) {
        return AstNumberNode::parse(parser, parent);
      }
      if AstColorNode::can_parse(&parser) {
        return AstColorNode::parse(parser, parent);
      }
      return Err(anyhow!("Error parsing expression"));
    }
  }
}
