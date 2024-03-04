use std::{cell::RefCell, ops::Range};

use cdl_lexer::{get_location_from_position, Token, TokenKind};
use serde::Serialize;

use crate::{
  ast_nodes::{
    ast_function::AstFunctionNode, AstColorNode, AstEntityNode, AstIdentifierNode, AstNumberNode,
    AstOperatorNode, AstPropertyNode, AstReferenceNode, AstScriptNode, AstStringNode,
    AstTableAliasNode, AstTitleNode, AstVPathNode, Parsable, ast_boolean::AstBooleanNode,
  },
  token_stream::TokenStream,
  types::NodeRef,
};
use anyhow::{Context, Result};

#[derive(Debug,Serialize)]
pub enum Node {
  Title(AstTitleNode),
  Entity(AstEntityNode),
  Property(AstPropertyNode),
  Identifier(AstIdentifierNode),
  Script(AstScriptNode),
  String(AstStringNode),
  Number(AstNumberNode),
  Boolean(AstBooleanNode),
  VPath(AstVPathNode),
  Color(AstColorNode),
  Reference(AstReferenceNode),
  Function(AstFunctionNode),
  Operator(AstOperatorNode),
  TableAlias(AstTableAliasNode),
}

#[derive(Debug)]
pub struct Parser {
  text: String,
  tokens: TokenStream,
  pub nodes: RefCell<Vec<Node>>,
  pub locations: RefCell<Vec<Range<usize>>>
}

impl Parser {
  pub fn new(text: &str, tokens: TokenStream) -> Parser {
    Parser {
      nodes: RefCell::new(Vec::new()),
      tokens,
      text: text.to_string(),
      locations: RefCell::new(Vec::new())
    }
  }
  pub fn parse(&mut self) -> Result<NodeRef> {
    AstScriptNode::parse(self, NodeRef(-1)).context(self.get_top_level_error_message())
  }

  fn get_top_level_error_message(&self) -> String {
    let token = self.get_current_token();
    if token.is_err() {
      "Unknown error".to_string()
    } else {
      let token = token.unwrap();
      let location = get_location_from_position(&self.text, &token.pos);
      format!(
        "Error while parsing at {}:{}",
        location.start_line, location.start_pos
      )
    }
  }

  pub(crate) fn get_current_token(&self) -> Result<&Token> {
    self.tokens.get_current_token()
  }

  pub(crate) fn get_next_token(&self, num: usize) -> Result<&Token> {
    self.tokens.get_nth_token(num)
  }

  #[allow(dead_code)]
  pub(crate) fn eat_token(&self) -> Result<Range<usize>> {
    self.tokens.eat_token()
  }

  pub(crate) fn eat_tokens(&self, num: usize) -> Result<Range<usize>> {
    self.tokens.eat_tokens(num)
  }

  pub(crate) fn eat_token_of_type(&self, kind: TokenKind) -> Result<Range<usize>> {
    self.tokens.eat_token_of_type(kind)
  }

  pub(crate) fn is_next_token_of_type(&self, kind: TokenKind) -> bool {
    return self.tokens.is_next_token_of_type(kind);
  }

  pub(crate) fn add_node(&self, n: Node, location: Range<usize>) -> NodeRef {
    let mut nodes = self.nodes.borrow_mut();
    nodes.push(n);
    let mut locations = self.locations.borrow_mut();
    locations.push(location);
    return (nodes.len() - 1).into();
  }

  pub(crate) fn get_tokens_of_kind(&self, kind: TokenKind) -> &[Token] {
    self.tokens.get_tokens_of_kind(kind)
  }

  pub(crate) fn add_child_to_node(&self, parent: NodeRef, child: NodeRef) {
    let mut nodes = self.nodes.borrow_mut();
    let node = nodes.get_mut(parent.0 as usize).unwrap();
    match node {
      Node::Entity(ent) => ent.children.push(child),
      Node::Script(script) => script.children.push(child),
      Node::Property(prop) => prop.child.push(child),
      Node::Function(func) => func.children.push(child),
      Node::Operator(op) => op.right = child,
      _ => panic!("Unknown type to set as parent {:?}", node),
    }
  }

  pub(crate) fn is_tokens_left(&self) -> bool {
    self.tokens.is_tokens_left()
  }

  pub(crate) fn eat_eol_and_comments(&mut self) {
    while self.is_tokens_left() {
      let curr_token = self.get_current_token().unwrap();
      if curr_token.kind == TokenKind::EOL
        || curr_token.kind == TokenKind::LineComment
        || curr_token.kind == TokenKind::MultiLineComment
      {
        let _ = self.eat_token();
      } else {
        break;
      }
    }
  }

  pub(crate) fn update_location_on_node(&self, node_ref: NodeRef, start: usize, end: usize) {
    let mut locations = self.locations.borrow_mut();
    locations[node_ref.0 as usize] = start..end;
  }

  pub(crate) fn get_pos_for_node(&self, node_ref: NodeRef) -> Range<usize> {
    let locations = self.locations.borrow();
    locations[node_ref.0 as usize].clone()
  }
}
