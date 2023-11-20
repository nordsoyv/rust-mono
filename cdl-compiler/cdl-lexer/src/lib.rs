use anyhow::Result;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
enum TokenLexer {
  #[token("false", |_| false)]
  #[token("true", |_| true)]
  Bool(bool),

  #[token("{")]
  BraceOpen,

  #[token("}")]
  BraceClose,

  #[token("[")]
  BracketOpen,

  #[token("]")]
  BracketClose,

  #[token("(")]
  ParenOpen,

  #[token(")")]
  ParenClose,

  #[token(":")]
  Colon,

  #[token(",")]
  Comma,

  #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
  Number(f64),

  #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().to_owned())]
  String(String),
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
  Boolean(bool),
  BraceOpen,
  BraceClose,
  BracketOpen,
  BracketClose,
  ParenOpen,
  ParenClose,
  Colon,
  Comma,
  Number,
  String,
}

#[derive(Debug, PartialEq)]
pub struct Token {
  kind: TokenKind,
  text: Option<String>,
  start_pos: usize,
  end_pos: usize,
}

pub fn lex(text: &str) -> Result<Vec<Token>> {
  let mut lexer = TokenLexer::lexer(text);

  let mut tokens = vec![];
  while let Some(token) = lexer.next() {
    match token {
      Ok(TokenLexer::Bool(value)) => {
        tokens.push(Token {
          start_pos: lexer.span().start,
          end_pos: lexer.span().end,
          kind: TokenKind::Boolean(value),
          text: None,
        });
      },
      Ok(TokenLexer::BraceClose) => {
        tokens.push(Token {
          start_pos: lexer.span().start,
          end_pos: lexer.span().end,
          kind: TokenKind::BraceClose,
          text: None,
        });
      },
      Ok(TokenLexer::BraceOpen) => {
        tokens.push(Token {
          start_pos: lexer.span().start,
          end_pos: lexer.span().end,
          kind: TokenKind::BraceOpen,
          text: None,
        });
      },
      Ok(TokenLexer::BracketOpen) => {
        tokens.push(Token {
          start_pos: lexer.span().start,
          end_pos: lexer.span().end,
          kind: TokenKind::BracketOpen,
          text: None,
        });
      },
      Ok(TokenLexer::BracketClose) => {
        tokens.push(Token {
          start_pos: lexer.span().start,
          end_pos: lexer.span().end,
          kind: TokenKind::BracketClose,
          text: None,
        });
      },
      Ok(TokenLexer::ParenOpen) => {
        tokens.push(Token {
          start_pos: lexer.span().start,
          end_pos: lexer.span().end,
          kind: TokenKind::ParenOpen,
          text: None,
        });
      },
      Ok(TokenLexer::ParenClose) => {
        tokens.push(Token {
          start_pos: lexer.span().start,
          end_pos: lexer.span().end,
          kind: TokenKind::ParenClose,
          text: None,
        });
      },
      _ => panic!(),
    }
  }
  Ok(tokens)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_parse_booleans() {
    let tokens = lex("true false").unwrap();
    assert_eq!(
      tokens[0],
      Token {
        kind: TokenKind::Boolean(true),
        start_pos: 0,
        end_pos: 4,
        text: None
      }
    );
    assert_eq!(
      tokens[1],
      Token {
        kind: TokenKind::Boolean(false),
        start_pos: 5,
        end_pos: 10,
        text: None
      }
    );
  }
  
  #[test]
  fn can_parse_brackets() {
    let tokens = lex("( { [ ) } ]").unwrap();
    assert_eq!(
      tokens[0],
      Token {
        kind: TokenKind::ParenOpen,
        start_pos: 0,
        end_pos: 1,
        text: None
      }
    );
    assert_eq!(
      tokens[1],
      Token {
        kind: TokenKind::BraceOpen,
        start_pos: 2,
        end_pos: 3,
        text: None
      }
    );
    assert_eq!(
      tokens[2],
      Token {
        kind: TokenKind::BracketOpen,
        start_pos: 4,
        end_pos: 5,
        text: None
      }
    );
    assert_eq!(
      tokens[3],
      Token {
        kind: TokenKind::ParenClose,
        start_pos: 6,
        end_pos: 7,
        text: None
      }
    );
    assert_eq!(
      tokens[4],
      Token {
        kind: TokenKind::BraceClose,
        start_pos: 8,
        end_pos: 9,
        text: None
      }
    );
    assert_eq!(
      tokens[5],
      Token {
        kind: TokenKind::BracketClose,
        start_pos: 10,
        end_pos: 11,
        text: None
      }
    );
  }
  
}
