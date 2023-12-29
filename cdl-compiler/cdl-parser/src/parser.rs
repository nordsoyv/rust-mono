use std::{cell::RefCell, ops::Range};

use cdl_lexer::{Token, TokenKind};

use crate::{
  ast_nodes::{
    ast_function::AstFunctionNode, AstColorNode, AstEntityNode, AstIdentifierNode, AstNumberNode,
    AstOperatorNode, AstPropertyNode, AstReferenceNode, AstScriptNode, AstStringNode, AstTitleNode,
    AstVPathNode, Parsable,
  },
  types::NodeRef,
};
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub enum Node {
  Title(AstTitleNode),
  Entity(AstEntityNode),
  Property(AstPropertyNode),
  Identifier(AstIdentifierNode),
  Script(AstScriptNode),
  String(AstStringNode),
  Number(AstNumberNode),
  VPath(AstVPathNode),
  Color(AstColorNode),
  Reference(AstReferenceNode),
  Function(AstFunctionNode),
  Operator(AstOperatorNode),
}

#[derive(Debug)]
pub struct Parser {
  pub tokens: Vec<Token>,
  pub curr_token: RefCell<usize>,
  pub nodes: RefCell<Vec<Node>>,
}

impl Parser {
  pub fn get_current_token(&self) -> Option<&Token> {
    let curr = self.curr_token.borrow();
    if *curr < self.tokens.len() {
      return Some(&self.tokens[*curr]);
    }
    None
  }

  pub fn get_next_token(&self, num: usize) -> Option<&Token> {
    let curr = self.curr_token.borrow();
    if *curr + num < self.tokens.len() {
      return Some(&self.tokens[*curr + num]);
    }
    None
  }

  #[allow(dead_code)]
  pub fn eat_token(&self) {
    self.curr_token.replace_with(|&mut old| old + 1);
  }

  pub fn eat_tokens(&self, num: usize) {
    self.curr_token.replace_with(|&mut old| old + num);
  }

  pub fn eat_token_of_type(&mut self, kind: TokenKind) -> Result<usize> {
    let end_pos = {
      let current_token = self
        .get_current_token()
        .ok_or(anyhow!(format!("Expected {:?}, found EOF", kind)))?;
      if current_token.kind != kind {
        return Err(anyhow!(format!(
          "Expected {:?}, found {:?}",
          kind, current_token.kind
        )));
      }
      current_token.pos.end
    };
    self.eat_token();
    Ok(end_pos)
  }

  pub fn is_next_token_of_type(&self, kind: TokenKind) -> bool {
    let curr_token = self.get_current_token();
    if curr_token.is_none() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if curr_token.kind == kind {
      return true;
    }
    return false;
  }

  pub fn add_node(&self, n: Node) -> NodeRef {
    let mut nodes = self.nodes.borrow_mut();
    nodes.push(n);
    return (nodes.len() - 1).into();
  }

  // pub fn get_next_node_ref(&self) -> NodeRef {
  //   return self.nodes.borrow().len().into();
  // }

  pub fn parse(&mut self) -> Result<NodeRef> {
    Ok(self.parse_top_level()?)
  }

  pub fn get_tokens_of_kind(&self, kind: TokenKind) -> &[Token] {
    let mut num_tokens = 0;
    loop {
      let curr_token = self.get_next_token(num_tokens);
      if curr_token.is_some() {
        let curr_token = curr_token.unwrap();
        if curr_token.kind == kind {
          num_tokens += 1;
        } else {
          break;
        }
      }
    }
    if num_tokens > 0 {
      let curr = *self.curr_token.borrow();
      let end_token = curr + num_tokens;
      return &self.tokens[curr..end_token];
    }
    return &[];
  }

  pub fn add_child_to_node(&self, parent: NodeRef, child: NodeRef) {
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

  fn is_tokens_left(&self) -> bool {
    self.tokens.len() > *self.curr_token.borrow()
  }

  pub fn eat_eol_and_comments(&mut self) {
    while self.is_tokens_left() {
      let curr_token = self.get_current_token().unwrap();
      if curr_token.kind == TokenKind::EOL
        || curr_token.kind == TokenKind::LineComment
        || curr_token.kind == TokenKind::MultiLineComment
      {
        self.eat_token();
      } else {
        break;
      }
    }
  }

  fn parse_top_level(&mut self) -> Result<NodeRef> {
    let root_node = AstScriptNode {
      children: vec![],
      location: 0..self.tokens[self.tokens.len() - 1].pos.end,
    };
    let root_node_ref = self.add_node(Node::Script(root_node));
    while self.is_tokens_left() {
      self.eat_eol_and_comments();
      if AstTitleNode::can_parse(self) {
        let node_ref = AstTitleNode::parse(self, root_node_ref)?;
        self.add_child_to_node(root_node_ref, node_ref);
        continue;
      }
      if AstEntityNode::can_parse(self) {
        let node_ref = AstEntityNode::parse(self, root_node_ref)?;
        self.add_child_to_node(root_node_ref, node_ref);
        continue;
      }
    }
    Ok(root_node_ref)
  }

  pub(crate) fn update_location_on_node(&self, node_ref: NodeRef, start: usize, end: usize) {
    let mut nodes = self.nodes.borrow_mut();
    let node = nodes.get_mut(node_ref.0 as usize).unwrap();
    match node {
      Node::Color(node) => node.location = start..end,
      Node::Entity(node) => node.location = start..end,
      Node::Function(node) => node.location = start..end,
      Node::Identifier(node) => node.location = start..end,
      Node::Number(node) => node.location = start..end,
      Node::Operator(node) => node.location = start..end,
      Node::Property(node) => node.location = start..end,
      Node::Reference(node) => node.location = start..end,
      Node::Script(node) => node.location = start..end,
      Node::String(node) => node.location = start..end,
      Node::Title(node) => node.location = start..end,
      Node::VPath(node) => node.location = start..end,
    }
  }

  pub fn get_pos_for_node(&self, node_ref: NodeRef) -> Range<usize> {
    let nodes = self.nodes.borrow();
    let node = nodes.get(node_ref.0 as usize).unwrap();
    match node {
      Node::Property(prop) => prop.location.clone(),
      Node::Entity(ent) => ent.location.clone(),
      Node::Function(func) => func.location.clone(),
      Node::Color(color) => color.location.clone(),
      Node::Identifier(node) => node.location.clone(),
      Node::Number(node) => node.location.clone(),
      Node::Operator(node) => node.location.clone(),
      Node::Reference(node) => node.location.clone(),
      Node::Script(node) => node.location.clone(),
      Node::String(node) => node.location.clone(),
      Node::Title(node) => node.location.clone(),
      Node::VPath(node) => node.location.clone(),
    }
  }
}
