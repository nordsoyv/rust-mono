use crate::{
  ast_nodes::{
    ast_function::AstFunctionNode, AstColorNode, AstIdentifierNode, AstNumberNode,
    AstReferenceNode, AstStringNode, AstVPathNode, Parsable,
  },
  parser::Parser,
  types::NodeRef,
};
use anyhow::{anyhow, Result};
use cdl_lexer::TokenKind;

pub fn parse_expression(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef> {
  loop {
    //parser.eat_eol_and_comments();
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
    dbg!(parser.get_current_token());
    return Err(anyhow!("Error parsing expression"));
  }
}

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
