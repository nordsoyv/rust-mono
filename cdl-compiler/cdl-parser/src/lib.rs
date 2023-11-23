use std::{cell::RefCell};

use anyhow::{anyhow, Result};
use cdl_lexer::{lex, Token, TokenKind};

type NodeRef = usize;

#[derive(Debug)]
struct Parser {
  tokens: Vec<Token>,
  curr_token: usize,
  nodes: RefCell<Vec<Node>>,
}

impl Parser {
  fn get_current_token(&self) -> Option<&Token> {
    if self.curr_token < self.tokens.len() {
      return Some(&self.tokens[self.curr_token]);
    }
    None
  }

  fn get_next_token(&self) -> Option<&Token> {
    if self.curr_token + 1 < self.tokens.len() {
      return Some(&self.tokens[self.curr_token + 1]);
    }
    None
  }

  fn eat_token(&mut self) {
    self.curr_token += 1;
  }

  fn add_node(&self, n: Node) -> NodeRef {
    let mut nodes = self.nodes.borrow_mut();
    nodes.push(n);
    return nodes.len() - 1;
  }

  fn parse(&mut self) {
    self.parse_top_level();
  }
  
  fn is_tokens_left(&self) -> bool {
    self.tokens.len() > self.curr_token
  }

  fn parse_top_level(&mut self) -> Result<()> {
    while self.is_tokens_left() {
      if let Some((nodeRef, eaten)) =  self.try_parse_title(&self.tokens[self.curr_token..]) {
        self.curr_token += eaten;
      }
    }


   
    Ok(())
  }

  fn try_parse_title(&self, tokens: &[Token]) -> Option<(NodeRef, usize)> {
    if tokens[0].kind != TokenKind::Identifier("title".to_string()) {
      return None;
    }
    if let TokenKind::String(title) = &tokens[1].kind  {
      if tokens[2].kind == TokenKind::EOL {
        let nodeRef = self.add_node(Node::Title(title.to_string()));
        return Some((nodeRef, 3));
      }
    }
    
    return None;
  }
}

pub fn parse_text(text: &str) -> Result<Ast> {
  let tokens = lex(text)?;
  let mut parser = Parser {
    curr_token: 0,
    nodes: RefCell::new(Vec::new()),
    tokens: tokens,
  };
  parser.parse();

  Ok(Ast {
    nodes: parser.nodes.take(),
    script_entity: 0,
  })
}

#[derive(Debug)]
pub struct Ast {
  pub nodes: Vec<Node>,
  pub script_entity: usize,
}

#[derive(Debug)]
pub enum Node {
  Entity,
  Title(String),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_parse_title() {
    let ast = parse_text("title \"title\"\n");
    dbg!(&ast);
    assert!(ast.is_ok());
  }
}
