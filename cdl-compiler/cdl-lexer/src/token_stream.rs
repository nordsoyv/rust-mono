use std::{ops::Range, rc::Rc};

use anyhow::{anyhow, Result};
use logos::{Lexer, Logos};

use crate::{get_location_from_position, Token, TokenKind, TokenLexer};

const MAX_TOKEN_ELEM: usize = 4;

#[derive(Debug)]
pub struct TokenStream<'a> {
  tokens: Vec<Token>,
  current_token: usize, // pointer to the next token to return to user. Should always be smaller or equal to  read_token
  read_token: usize,    // pointer to how many tokens we have read into the array
  lexer: Lexer<'a, TokenLexer>,
}

impl<'a> TokenStream<'a> {
  pub fn init(text: &str) -> Result<TokenStream> {
    let mut lexer = TokenLexer::lexer(text);
    let mut tokens = Vec::with_capacity(MAX_TOKEN_ELEM);
    for _ in 0..MAX_TOKEN_ELEM {
      tokens.push(Token {
        kind: TokenKind::Unknown,
        pos: 0..0,
        text: None,
      })
    }

    let first_token = lexer.next();
    if first_token.is_none() {
      return Err(anyhow!("Could not read token from stream"));
    }
    let first_token = first_token.unwrap();
    if first_token.is_err() {
      let location = get_location_from_position(lexer.source(), &lexer.span());
      return Err(anyhow!(
        "[{}:{}]: Unknown token \"{}\"",
        location.start_line,
        location.end_line,
        lexer.slice(),
      ));
    }
    let first_token = first_token.unwrap();
    let mapped_token = map_to_token_lexer(&first_token);
    tokens[0].kind = mapped_token.0;
    tokens[0].pos = lexer.span();
    tokens[0].text = mapped_token.1;

    Ok(TokenStream {
      tokens,
      current_token: 0,
      read_token: 0,
      lexer,
    })
  }

  pub fn eat_token(&mut self) -> Result<Range<usize>> {
    self.eat_tokens(1)
  }

  pub fn eat_tokens(&mut self, num: usize) -> Result<Range<usize>> {
    let start_pos = self.get_current_token()?.pos.start;
    let end_pos = self.get_nth_token(num - 1)?.pos.end;
    self.current_token += num;
    return Ok(start_pos..end_pos);
  }

  pub fn get_current_token(&mut self) -> Result<&Token> {
    self.get_nth_token(0)
  }

  pub fn get_next_token(&mut self) -> Result<&Token> {
    self.get_nth_token(1)
  }

  pub fn get_nth_token(&mut self, nth: usize) -> Result<&Token> {
    loop {
      if self.current_token + nth > self.read_token {
        self.read_next_token()?;
      } else {
        break;
      }
    }
    if self.read_token - self.current_token > MAX_TOKEN_ELEM {
      return Err(anyhow!("Tried to read too far ahead"));
    }
    let token = &self.tokens[self.current_token + nth % MAX_TOKEN_ELEM];
    Ok(token)
  }

  fn get_token_from_lexer(&mut self) -> Result<TokenLexer> {
    let first_token = self.lexer.next();
    if first_token.is_none() {
      return Err(anyhow!("Could not read token from stream"));
    }
    let first_token = first_token.unwrap();
    if first_token.is_err() {
      let location = get_location_from_position(self.lexer.source(), &self.lexer.span());
      return Err(anyhow!(
        "[{}:{}]: Unknown token \"{}\"",
        location.start_line,
        location.end_line,
        self.lexer.slice(),
      ));
    }
    Ok(first_token.unwrap())
  }

  fn read_next_token(&mut self) -> Result<()> {
    self.read_token += 1;
    let token = self.get_token_from_lexer()?;
    let mapped_token = map_to_token_lexer(&token);

    self.tokens[self.read_token % MAX_TOKEN_ELEM].kind = mapped_token.0;
    self.tokens[self.read_token % MAX_TOKEN_ELEM].pos = self.lexer.span();
    self.tokens[self.read_token % MAX_TOKEN_ELEM].text = mapped_token.1;
    Ok(())
  }

  pub fn is_next_token_of_type(&mut self, kind: TokenKind) -> bool {
    let curr_token = self.get_current_token();
    if curr_token.is_err() {
      return false;
    }
    let curr_token = curr_token.unwrap();
    if curr_token.kind == kind {
      return true;
    }
    return false;
  }
  pub fn eat_token_of_type(&mut self, kind: TokenKind) -> Result<Range<usize>> {
    let pos = {
      let current_token = self.get_current_token();
      if current_token.is_err() {
        return Err(anyhow!(format!("Expected {:?}, found EOF", kind)));
      }
      let current_token = current_token.unwrap();
      let token_kind = &current_token.kind;
      if *token_kind != kind {
        return Err(anyhow!(format!(
          "Expected {:?}, found {:?}",
          kind, token_kind,
        )));
      }
      current_token.pos.clone()
    };
    
    self.eat_token()?;
    return Ok(pos.clone());
  }
}

fn map_to_token_lexer(token: &TokenLexer) -> (TokenKind, Option<Rc<str>>) {
  match token {
    TokenLexer::Bool(b) => (TokenKind::Boolean(*b), None),
    TokenLexer::EOL => (TokenKind::EOL, None),
    TokenLexer::BraceOpen => (TokenKind::BraceOpen, None),
    TokenLexer::BraceClose => (TokenKind::BraceClose, None),
    TokenLexer::BracketOpen => (TokenKind::BracketOpen, None),
    TokenLexer::BracketClose => (TokenKind::BracketClose, None),
    TokenLexer::ParenOpen => (TokenKind::ParenOpen, None),
    TokenLexer::ParenClose => (TokenKind::ParenClose, None),
    TokenLexer::Colon => (TokenKind::Colon, None),
    TokenLexer::Comma => (TokenKind::Comma, None),
    TokenLexer::Plus => (TokenKind::Plus, None),
    TokenLexer::Minus => (TokenKind::Minus, None),
    TokenLexer::Div => (TokenKind::Div, None),
    TokenLexer::Mul => (TokenKind::Mul, None),
    TokenLexer::Hash => (TokenKind::Hash, None),
    TokenLexer::Percent => (TokenKind::Percent, None),
    TokenLexer::Equal => (TokenKind::Equal, None),
    TokenLexer::NotEqual => (TokenKind::NotEqual, None),
    TokenLexer::LessThan => (TokenKind::LessThan, None),
    TokenLexer::MoreThan => (TokenKind::MoreThan, None),
    TokenLexer::LessThanOrEqual => (TokenKind::LessThanOrEqual, None),
    TokenLexer::MoreThanOrEqual => (TokenKind::MoreThanOrEqual, None),
    TokenLexer::And => (TokenKind::And, None),
    TokenLexer::Or => (TokenKind::Or, None),
    TokenLexer::Number(n) => (TokenKind::Number(*n), None),
    TokenLexer::String(s) => (TokenKind::String, Some(s.clone())),
    TokenLexer::Identifier(i) => (TokenKind::Identifier, Some(i.clone())),
    TokenLexer::Reference(r) => (TokenKind::Reference, Some(r.clone())),
    TokenLexer::Color(c) => (TokenKind::Color, Some(c.clone())),
    TokenLexer::LineComment(l) => (TokenKind::LineComment, Some(l.clone())),
    TokenLexer::MultiLineComment(l) => (TokenKind::MultiLineComment, Some(l.clone())),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create_test_stream(text: &str) -> TokenStream {
    let stream = TokenStream::init(text);
    assert!(stream.is_ok());
    stream.unwrap()
  }

  fn create_identifier_stream() -> TokenStream<'static> {
    create_test_stream("identifier identifier2")
  }

  #[test]
  fn can_get_current_token() {
    let mut stream = create_test_stream("1");
    assert_eq!(
      stream.get_current_token().unwrap(),
      &Token {
        kind: TokenKind::Number(1.0),
        text: None,
        pos: 0..1
      }
    );
  }

  #[test]
  fn can_get_next_token() {
    let mut stream = create_test_stream("1 2");
    assert_eq!(
      stream.get_next_token().unwrap(),
      &Token {
        kind: TokenKind::Number(2.0),
        text: None,
        pos: 2..3
      }
    );
  }

  #[test]
  fn can_get_nth_token() {
    let mut stream = create_test_stream("1 2");
    assert_eq!(
      stream.get_nth_token(1).unwrap(),
      &Token {
        kind: TokenKind::Number(2.0),
        text: None,
        pos: 2..3
      }
    );
  }

  #[test]
  fn getting_more_than_max_elem_tokens_bails() {
    let mut stream = create_test_stream("1 2 3 4 5 6 7 8");
    let token = stream.get_nth_token(MAX_TOKEN_ELEM + 1);
    assert!(token.is_err());
    let err = token.err().unwrap();
    assert_eq!("Tried to read too far ahead", err.to_string());
  }

  #[test]
  fn can_get_token() {
    let mut stream = create_identifier_stream();
    let token = stream.get_current_token();
    assert!(token.is_ok());
    let token = token.unwrap();
    assert_eq!(TokenKind::Identifier, token.kind);
    assert_eq!("identifier", token.text.as_ref().unwrap().to_string());
  }

  #[test]
  fn can_get_next_token2() {
    let mut stream = create_identifier_stream();
    let token = stream.get_nth_token(1);
    assert!(token.is_ok());
    let token = token.unwrap();
    assert_eq!(TokenKind::Identifier, token.kind);
    assert_eq!("identifier2", token.text.as_ref().unwrap().to_string());
  }

  #[test]
  fn can_eat_token() {
    let mut stream = create_identifier_stream();
    let pos = stream.eat_token();
    assert!(pos.is_ok());
    let pos = pos.unwrap();
    assert_eq!(0..10, pos);
    assert_eq!(1, stream.current_token);
  }

  #[test]
  fn can_eat_tokens() {
    let mut stream = create_identifier_stream();
    let pos = stream.eat_tokens(1);
    assert!(pos.is_ok());
    let pos = pos.unwrap();
    assert_eq!(0..10, pos);
    assert_eq!(1, stream.current_token);
  }

  #[test]
  fn can_eat_tokens2() {
    let mut stream = create_identifier_stream();
    let pos = stream.eat_tokens(2);
    assert!(pos.is_ok());
    let pos = pos.unwrap();
    assert_eq!(0..22, pos);
    assert_eq!(2, stream.current_token);
  }

  #[test]
  fn eating_to_many_tokens_fail() {
    let mut stream = create_identifier_stream();
    let pos = stream.eat_tokens(3);
    assert!(pos.is_err());
  }

  #[test]
  fn can_eat_token_of_type() {
    let mut stream = create_identifier_stream();
    let pos = stream.eat_token_of_type(TokenKind::Identifier);
    assert!(pos.is_ok());
    let pos = pos.unwrap();
    assert_eq!(0..10, pos);
  }

  #[test]
  fn is_next_token_of_type() {
    let mut stream = create_identifier_stream();
    let res = stream.is_next_token_of_type(TokenKind::Identifier);
    assert!(res);
  }
}
