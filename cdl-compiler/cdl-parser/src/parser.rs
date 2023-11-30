use std::cell::RefCell;

use cdl_lexer::{Token, TokenKind};

use crate::ast_nodes::{ast_entity::AstEntityNode, ast_title::AstTitleNode};
use crate::ast_nodes::Parsable;
use crate::types::NodeRef;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub enum Node {
  Title(AstTitleNode),
  Entity(AstEntityNode),
}

#[derive(Debug)]
pub struct Parser {
  pub tokens: Vec<Token>,
  pub curr_token: usize,
  pub nodes: RefCell<Vec<Node>>,
}

impl Parser {
  pub fn get_current_token(&self) -> Option<&Token> {
    if self.curr_token < self.tokens.len() {
      return Some(&self.tokens[self.curr_token]);
    }
    None
  }

  pub fn get_next_token(&self, num: usize) -> Option<&Token> {
    if self.curr_token + num < self.tokens.len() {
      return Some(&self.tokens[self.curr_token + num]);
    }
    None
  }

  #[allow(dead_code)]
  pub fn eat_token(&mut self) {
    self.curr_token += 1;
  }

  pub fn eat_tokens(&mut self, num: usize) {
    self.curr_token += num;
  }

  pub fn eat_token_of_type(&mut self, kind: TokenKind) -> Result<()> {
    let current_token = self
      .get_current_token()
      .ok_or(anyhow!(format!("Expected {:?}, found EOF", kind)))?;
    if current_token.kind != kind {
      return Err(anyhow!(format!(
        "Expected {:?}, found {:?}",
        kind, current_token.kind
      )));
    }
    self.eat_token();
    Ok(())
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
      let end_token = self.curr_token + num_tokens;
      return &self.tokens[self.curr_token..end_token];
    }
    return &[];
  }

  pub fn add_child_to_node(&self, parent: NodeRef, child: NodeRef) {
    let mut nodes = self.nodes.borrow_mut();
    let node = nodes.get_mut(parent.0 as usize).unwrap();
    match node {
      Node::Entity(ent) => ent.children.push(child),
      _ => {}
    }
  }

  fn is_tokens_left(&self) -> bool {
    self.tokens.len() > self.curr_token
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
    let root_node = AstEntityNode {
      children: vec![],
      parent: NodeRef(-1),
      terms: vec![],
    };
    let root_node_ref = self.add_node(Node::Entity(root_node));
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
}
