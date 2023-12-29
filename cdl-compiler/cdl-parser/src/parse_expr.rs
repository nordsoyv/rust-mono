use crate::{
  ast_nodes::{
    ast_function::AstFunctionNode, AstColorNode, AstIdentifierNode, AstNumberNode, AstOperatorNode,
    AstReferenceNode, AstStringNode, AstVPathNode, Parsable,
  },
  parser::Parser,
  types::NodeRef,
};
use anyhow::{anyhow, Result};
use cdl_lexer::TokenKind;

pub fn parse_arg_list(parser: &mut Parser, parent: NodeRef) -> Result<Vec<NodeRef>> {
  let mut node_refs = vec![];
  loop {
    let current_token = parser
      .get_current_token()
      .ok_or(anyhow!("Got unexptected EOF while parsing a list"))?;
    match current_token.kind {
      TokenKind::ParenClose => {
        return Ok(node_refs);
      }
      TokenKind::Comma => parser.eat_token(),
      _ => node_refs.push(parse_expression(parser, parent)?),
    }
  }
}

pub fn parse_list(parser: &mut Parser, parent: NodeRef) -> Result<Vec<NodeRef>> {
  let mut node_refs = vec![];
  loop {
    let current_token = parser
      .get_current_token()
      .ok_or(anyhow!("Got unexptected EOF while parsing a list"))?;
    match current_token.kind {
      TokenKind::EOL => {
        return Ok(node_refs);
      }
      TokenKind::Comma => parser.eat_token(),
      _ => node_refs.push(parse_expression(parser, parent)?),
    }
  }
}

pub fn parse_expression(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
  let term_node_ref = parse_term(parser, parent)?;
  if AstOperatorNode::can_parse_term(parser) {
    let operator_ref = AstOperatorNode::parse_operator_term(parser, parent, term_node_ref)?;
    return Ok(operator_ref);
  } else {
    return Ok(term_node_ref);
  }
}

pub fn parse_term(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
  let factor_node_ref = parse_factor(parser, parent)?;
  if AstOperatorNode::can_parse_factor(parser) {
    let operator_ref = AstOperatorNode::parse_operator_factor(parser, parent, factor_node_ref)?;
    return Ok(operator_ref);
  } else {
    return Ok(factor_node_ref);
  }
}

pub fn parse_factor(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
  if AstVPathNode::can_parse(&parser) {
    return AstVPathNode::parse(parser, parent);
  }
  if AstFunctionNode::can_parse(&parser) {
    return AstFunctionNode::parse(parser, parent);
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
  if AstReferenceNode::can_parse(&parser) {
    return AstReferenceNode::parse(parser, parent);
  }
  if AstColorNode::can_parse(&parser) {
    return AstColorNode::parse(parser, parent);
  }
  if parser.is_next_token_of_type(TokenKind::ParenOpen) {
    parser.eat_token();
    let expr_node = parse_expression(parser, parent)?;
    let location = parser.get_pos_for_node(expr_node);
    let end = parser.eat_token_of_type(TokenKind::ParenClose)?;
    parser.update_location_on_node(expr_node, location.start, end);
    return Ok(expr_node);
  }
  dbg!(parser.get_current_token());
  return Err(anyhow!("Error parsing expression"));
}
