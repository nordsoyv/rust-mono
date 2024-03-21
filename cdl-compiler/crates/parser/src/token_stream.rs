use std::{cell::RefCell, ops::Range};

use anyhow::{anyhow, Result};
use lexer::{Token, TokenKind};

#[derive(Debug)]
pub struct TokenStream {
  tokens: Vec<Token>,
  curr_token: RefCell<usize>,
}

impl TokenStream {
  pub fn new(tokens: Vec<Token>) -> TokenStream {
    TokenStream {
      tokens,
      curr_token: RefCell::new(0),
    }
  }
  pub fn get_current_token(&self) -> Result<&Token> {
    return self.get_nth_token(0);
  }

  pub fn get_nth_token(&self, num: usize) -> Result<&Token> {
    let curr = self.curr_token.borrow();
    if *curr + num < self.tokens.len() {
      return Ok(&self.tokens[*curr + num]);
    }
    Err(anyhow!("Expected to find a token, but got EOF instead"))
  }

  pub fn eat_token(&self) -> Result<Range<usize>> {
    self.eat_tokens(1)
  }

  pub fn eat_tokens(&self, num: usize) -> Result<Range<usize>> {
    if self.is_many_tokens_left(num - 1) {
      let pos_start = &self.get_current_token()?.pos;
      let pos_end = &self.get_nth_token(num - 1)?.pos;

      self.curr_token.replace_with(|&mut old| old + num);
      return Ok(pos_start.start..pos_end.end);
    }
    Err(anyhow!("Expected to find a token, but got EOF instead"))
  }

  pub fn eat_token_of_type(&self, kind: TokenKind) -> Result<Range<usize>> {
    let current_token = self.get_current_token();
    if let Ok(current_token) = current_token {
      if current_token.kind != kind {
        return Err(anyhow!(format!(
          "Expected {:?}, found {:?}",
          kind, current_token.kind,
        )));
      }
      self.eat_token()?;
      return Ok(current_token.pos.clone());
    }
    Err(anyhow!("Expected to find a token, but got EOF instead"))
  }

  pub fn get_tokens_of_kind(&self, kind: TokenKind) -> &[Token] {
    let mut num_tokens = 0;
    loop {
      let curr_token = self.get_nth_token(num_tokens);
      if curr_token.is_ok() {
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
    &[]
  }

  pub fn is_tokens_left(&self) -> bool {
    let curr = self.curr_token.borrow();
    if *curr + 1 < self.tokens.len() {
      return true;
    }
    false
  }

  pub fn is_next_token_of_type(&self, kind: TokenKind) -> bool {
    let curr_token = self.get_current_token();
    if curr_token.is_err() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if curr_token.kind == kind {
      return true;
    }
    false
  }

  fn is_many_tokens_left(&self, num: usize) -> bool {
    let curr = self.curr_token.borrow();
    if *curr + num < self.tokens.len() {
      return true;
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use lexer::{Token, TokenKind};

  use super::TokenStream;

  fn create_tokens() -> Vec<Token> {
    vec![
      Token {
        kind: lexer::TokenKind::Identifier,
        pos: 0..10,
        text: Some("identifier".into()),
      },
      Token {
        kind: lexer::TokenKind::Identifier,
        pos: 11..20,
        text: Some("identifier2".into()),
      },
    ]
  }

  #[test]
  fn can_get_token() {
    let stream = TokenStream::new(create_tokens());
    let token = stream.get_current_token();
    assert!(token.is_ok());
    let token = token.unwrap();
    assert_eq!(lexer::TokenKind::Identifier, token.kind);
    assert_eq!("identifier", token.text.as_ref().unwrap().to_string());
  }

  #[test]
  fn can_get_next_token() {
    let stream = TokenStream::new(create_tokens());
    let token = stream.get_nth_token(1);
    assert!(token.is_ok());
    let token = token.unwrap();
    assert_eq!(lexer::TokenKind::Identifier, token.kind);
    assert_eq!("identifier2", token.text.as_ref().unwrap().to_string());
  }

  #[test]
  fn can_eat_token() {
    let stream = TokenStream::new(create_tokens());
    let pos = stream.eat_token();
    assert!(pos.is_ok());
    let pos = pos.unwrap();
    assert_eq!(0..10, pos);
    assert_eq!(1, *stream.curr_token.borrow());
  }

  #[test]
  fn can_eat_tokens() {
    let stream = TokenStream::new(create_tokens());
    let pos = stream.eat_tokens(1);
    assert!(pos.is_ok());
    let pos = pos.unwrap();
    assert_eq!(0..10, pos);
    assert_eq!(1, *stream.curr_token.borrow());
  }

  #[test]
  fn can_eat_tokens2() {
    let stream = TokenStream::new(create_tokens());
    let pos = stream.eat_tokens(2);
    assert!(pos.is_ok());
    let pos = pos.unwrap();
    assert_eq!(0..20, pos);
    assert_eq!(2, *stream.curr_token.borrow());
  }

  #[test]
  fn eating_to_many_tokens_fail() {
    let stream = TokenStream::new(create_tokens());
    let pos = stream.eat_tokens(3);
    assert!(pos.is_err());
  }

  #[test]
  fn can_eat_token_of_type() {
    let stream = TokenStream::new(create_tokens());
    let pos = stream.eat_token_of_type(TokenKind::Identifier);
    assert!(pos.is_ok());
    let pos = pos.unwrap();
    assert_eq!(0..10, pos);
  }

  #[test]
  fn is_next_token_of_type() {
    let stream = TokenStream::new(create_tokens());
    let res = stream.is_next_token_of_type(TokenKind::Identifier);
    assert!(res);
  }
}
