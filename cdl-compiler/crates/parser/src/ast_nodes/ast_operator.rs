use anyhow::{anyhow, Result};

use ast::{AstNode, AstOperatorNode, Node, NodeRef, Operator};
use lexer::TokenKind;

use crate::{parse_expr::parse_expression, parser::Parser};

pub fn can_parse_term(parser: &mut Parser) -> bool {
  let curr_token = parser.get_current_token();
  if curr_token.is_err() {
    return false;
  }
  let curr_token = curr_token.unwrap();
  match curr_token.kind {
    TokenKind::Div
    | TokenKind::Mul
    | TokenKind::Equal
    | TokenKind::NotEqual
    | TokenKind::LessThan
    | TokenKind::LessThanOrEqual
    | TokenKind::MoreThan
    | TokenKind::MoreThanOrEqual
    | TokenKind::And
    | TokenKind::Or => true,
    _ => false,
  }
}

pub fn can_parse_factor(parser: &mut Parser) -> bool {
  let curr_token = parser.get_current_token();
  if curr_token.is_err() {
    return false;
  }
  let curr_token = curr_token.unwrap();
  match curr_token.kind {
    TokenKind::Plus | TokenKind::Minus => true,
    _ => false,
  }
}

pub fn parse_operator_term(parser: &mut Parser, parent: NodeRef, left: NodeRef) -> Result<NodeRef> {
  let operator_token = parser.get_current_token()?;
  parser.eat_token()?;
  let operator = match operator_token.kind {
    TokenKind::Div => Operator::Div,
    TokenKind::Mul => Operator::Mul,
    TokenKind::Equal => Operator::Equal,
    TokenKind::NotEqual => Operator::NotEqual,
    TokenKind::LessThan => Operator::LessThan,
    TokenKind::LessThanOrEqual => Operator::LessThanOrEqual,
    TokenKind::MoreThan => Operator::MoreThan,
    TokenKind::MoreThanOrEqual => Operator::MoreThanOrEqual,
    TokenKind::And => Operator::And,
    TokenKind::Or => Operator::Or,
    _ => return Err(anyhow!("Unknown token when parsing operator")),
  };
  let operator_node = AstOperatorNode {
    left,
    right: NodeRef(0),
    operator,
  };
  let operator_node_ref = parser.add_node(
    AstNode::new(Node::Operator(operator_node), parent),
    operator_token.pos.start..usize::MAX,
  );
  let right_node = parse_expression(parser, operator_node_ref)?;
  parser.add_child_to_node(operator_node_ref, right_node);
  let left_pos = parser.get_pos_for_node(left);
  let right_pos = parser.get_pos_for_node(right_node);
  parser.update_location_on_node(operator_node_ref, left_pos.start, right_pos.end);
  Ok(operator_node_ref)
}

pub fn parse_operator_factor(
  parser: &mut Parser,
  parent: NodeRef,
  left: NodeRef,
) -> Result<NodeRef> {
  let operator_token = parser.get_current_token()?;
  parser.eat_token()?;
  let operator = match operator_token.kind {
    TokenKind::Plus => Operator::Plus,
    TokenKind::Minus => Operator::Minus,
    _ => return Err(anyhow!("Unknown token when parsing operator")),
  };
  let operator_node = AstOperatorNode {
    left,
    right: NodeRef(0),
    operator,
  };
  let operator_node_ref = parser.add_node(
    AstNode::new(Node::Operator(operator_node), parent),
    operator_token.pos.start..usize::MAX,
  );
  let right_node = parse_expression(parser, operator_node_ref)?;
  parser.add_child_to_node(operator_node_ref, right_node);
  let left_pos = parser.get_pos_for_node(left);
  let right_pos = parser.get_pos_for_node(right_node);
  parser.update_location_on_node(operator_node_ref, left_pos.start, right_pos.end);
  Ok(operator_node_ref)
}
