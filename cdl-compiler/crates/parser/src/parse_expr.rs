use crate::{
  ast_nodes::{ast_entity::{can_parse_anonymous_entity, parse_anonymous_entity}, ast_operator::{can_parse_factor, can_parse_term, parse_operator_factor, parse_operator_term}, Parsable}, parser::Parser
};
use anyhow::{anyhow, Result};
use ast::{AstBooleanNode, AstColorNode, AstEntityNode, AstFormulaNode, AstFunctionNode, AstIdentifierNode, AstNumberNode, AstReferenceNode, AstStringNode, AstVPathNode, NodeRef};
use lexer::TokenKind;

pub fn parse_arg_list(parser: &mut Parser, parent: NodeRef) -> Result<Vec<NodeRef>> {
  let mut node_refs = vec![];
  loop {
    let current_token = parser.get_current_token()?;
    match current_token.kind {
      TokenKind::ParenClose => {
        return Ok(node_refs);
      }
      TokenKind::Comma => {
        let _ = parser.eat_token()?;
      }
      _ => node_refs.push(parse_expression(parser, parent)?),
    }
  }
}

pub fn parse_bracket_arg_list(parser: &mut Parser, _parent: NodeRef) -> Result<Vec<NodeRef>> {
  let node_refs = vec![];
  loop {
    let current_token = parser.get_current_token()?;
    match current_token.kind {
      TokenKind::BracketClose => {
        return Ok(node_refs);
      }
      TokenKind::Comma => {
        let _ = parser.eat_token()?;
      }
      _ => {
        parser.eat_token()?;
      }
    }
  }
}

pub fn parse_list(parser: &mut Parser, parent: NodeRef) -> Result<Vec<NodeRef>> {
  let mut node_refs = vec![];
  //parser.start_group("List".to_string());
  loop {
    let current_token = parser.get_current_token()?;
    match current_token.kind {
      TokenKind::EOL => {
        //    parser.end_group("EOL");
        return Ok(node_refs);
      }
      TokenKind::BraceClose => {
        //    parser.end_group("BraceClose");
        return Ok(node_refs);
      }
      TokenKind::LineComment => {
        let _ = parser.eat_token()?;
      }
      TokenKind::Comma => {
        //      parser.trace("Found comma in list");
        let _ = parser.eat_token()?;
        let next_token = parser.get_current_token()?;
        if next_token.kind == TokenKind::EOL {
          //      parser.trace("Found EOL after comma in list");
          parser.eat_token()?;
          node_refs.push(parse_expression(parser, parent)?);
        }
      }
      _ => node_refs.push(parse_expression(parser, parent)?),
    }
  }
}

pub fn parse_expression(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
  let term_node_ref = parse_term(parser, parent)?;
  if can_parse_term(parser) {
    let operator_ref = parse_operator_term(parser, parent, term_node_ref)?;
    Ok(operator_ref)
  } else {
    Ok(term_node_ref)
  }
}

pub fn parse_term(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
  let factor_node_ref = parse_factor(parser, parent)?;
  if can_parse_factor(parser) {
    let operator_ref =parse_operator_factor(parser, parent, factor_node_ref)?;
    Ok(operator_ref)
  } else {
    Ok(factor_node_ref)
  }
}

pub fn parse_factor(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
  if can_parse_anonymous_entity(parser) {
    return parse_anonymous_entity(parser, parent);
  }
  if AstVPathNode::can_parse(parser) {
    return AstVPathNode::parse(parser, parent);
  }
  if AstFunctionNode::can_parse(parser) {
    return AstFunctionNode::parse(parser, parent);
  }
  if AstFormulaNode::can_parse(parser) {
    return AstFormulaNode::parse(parser, parent);
  }
  if AstIdentifierNode::can_parse(parser) {
    return AstIdentifierNode::parse(parser, parent);
  }
  if AstStringNode::can_parse(parser) {
    return AstStringNode::parse(parser, parent);
  }
  if AstNumberNode::can_parse(parser) {
    return AstNumberNode::parse(parser, parent);
  }
  if AstReferenceNode::can_parse(parser) {
    return AstReferenceNode::parse(parser, parent);
  }
  if AstColorNode::can_parse(parser) {
    return AstColorNode::parse(parser, parent);
  }
  if AstBooleanNode::can_parse(parser) {
    return AstBooleanNode::parse(parser, parent);
  }
  if AstEntityNode::can_parse(parser) {
    return AstEntityNode::parse(parser, parent);
  }

  if parser.is_next_token_of_type(TokenKind::LineComment) {
    parser.eat_token()?;
  }

  if parser.is_next_token_of_type(TokenKind::ParenOpen) {
    parser.eat_token()?;
    let expr_node = parse_expression(parser, parent)?;
    let location = parser.get_pos_for_node(expr_node);
    let end = parser.eat_token_of_type(TokenKind::ParenClose)?;
    parser.update_location_on_node(expr_node, location.start, end.end);
    return Ok(expr_node);
  }
  //dbg!(parser.get_current_token());
  let token = parser.get_current_token()?;
  Err(anyhow!(
    "Error parsing expression, current token: {:?}",
    token.kind
  ))
}
