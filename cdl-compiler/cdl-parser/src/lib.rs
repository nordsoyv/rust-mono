use std::{cell::RefCell, rc::Rc};

use anyhow::{ Result};
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

  fn get_next_token(&self, num : usize) -> Option<&Token> {
    if self.curr_token + num < self.tokens.len() {
      return Some(&self.tokens[self.curr_token + num]);
    }
    None
  }

  fn eat_token(&mut self) {
    self.curr_token += 1;
  }
  
  fn eat_tokens(&mut self, num :usize) {
    self.curr_token += num;
  }
  fn add_node(&self, n: Node) -> NodeRef {
    let mut nodes = self.nodes.borrow_mut();
    nodes.push(n);
    return nodes.len() - 1;
  }

  fn parse(&mut self) -> Result<()> {
    self.parse_top_level()?;
    Ok(())
  }
  
  fn is_tokens_left(&self) -> bool {
    self.tokens.len() > self.curr_token
  }

  fn parse_top_level(&mut self) -> Result<()> {
    while self.is_tokens_left() {
      if let Some(_node_ref) =  self.try_parse_title() {
        continue;
      }
    }
    Ok(())
  }

  fn try_parse_title(&mut self) -> Option<NodeRef> {
    let curr_token = self.get_current_token()?;
    let token_1 = self.get_next_token(1)?;
    let token_2 = self.get_next_token(2)?;
    
    if curr_token.kind != TokenKind::Identifier("title".into()) {
      return None;
    }
    if let TokenKind::String(title) = &token_1.kind  {
      if token_2.kind == TokenKind::EOL {
        let node_ref = self.add_node(Node::Title(title.clone()));
        self.eat_tokens(3);
        return Some(node_ref);
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
  let _ = parser.parse();

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
  Title(Rc<str>),
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
