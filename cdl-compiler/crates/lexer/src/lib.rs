use anyhow::anyhow;
use anyhow::Result;
use logos::Lexer;
use logos::Logos;
use logos::Span;
use serde::Serialize;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug,Clone,Serialize,PartialEq,Eq,Hash)]
pub struct LexedStr(pub Rc<str>);

impl From<&str> for LexedStr {
  fn from(value: &str) -> Self {
    LexedStr(value.into())
  }
}

impl Display for LexedStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.0)
    }
}

impl LexedStr {
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

fn to_rcstr(lex: &mut Lexer<TokenLexer>) -> LexedStr {
  let slice = lex.slice();
  slice.into()
}

fn to_rcstr_skip1(lex: &mut Lexer<TokenLexer>) -> LexedStr {
  let slice = &lex.slice()[1..];
  slice.into()
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
enum TokenLexer {
  #[token("false", |_| false)]
  #[token("true", |_| true)]
  Bool(bool),

  #[token("\n")]
  #[token("\r\n")]
  Eol,

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

  #[token("+")]
  Plus,
  #[token("-")]
  Minus,
  #[token("/")]
  Div,
  #[token("*")]
  Mul,
  #[token("#")]
  Hash,
  #[token("%")]
  Percent,
  #[token("=")]
  Equal,
  #[token("!=")]
  NotEqual,
  #[token("<")]
  LessThan,
  #[token(">")]
  MoreThan,
  #[token("<=")]
  LessThanOrEqual,
  #[token(">=")]
  MoreThanOrEqual,

  #[regex(r"(?i)and")]
  And,
  #[regex(r"(?i)or")]
  Or,

  #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
  Number(f64),

  #[regex(r#""(?:[^"]|\\")*""#, to_rcstr)]
  #[regex(r#"'(?:[^']|\\')*'"#, to_rcstr)]
  String(LexedStr),

  #[regex("_?[$a-zA-Z0-9_\\-\\.]*", to_rcstr)]
  Identifier(LexedStr),

  #[regex("@[a-zA-Z0-9_\\-\\.]*", to_rcstr_skip1)]
  Reference(LexedStr),

  #[regex("\\^[a-zA-Z0-9_\\-\\.]*", to_rcstr_skip1)]
  HierarchyReference(LexedStr),

  #[regex("#[0-9a-fA-F]{6}", to_rcstr_skip1)]
  Color(LexedStr),

  #[regex("//[^\n]*", to_rcstr)]
  LineComment(LexedStr),

  #[regex(r#"/\*(?:[^*]|\*[^/])*\*/"#, to_rcstr)]
  MultiLineComment(LexedStr),
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
  Boolean(bool),
  EOL,
  BraceOpen,
  BraceClose,
  BracketOpen,
  BracketClose,
  ParenOpen,
  ParenClose,
  Colon,
  Comma,
  Number(f64),
  String,
  Plus,
  Minus,
  Div,
  Mul,
  Hash,
  Equal,
  Percent,
  NotEqual,
  LessThan,
  LessThanOrEqual,
  And,
  Or,
  MoreThan,
  MoreThanOrEqual,
  Identifier,
  Reference,
  HierarchyReference,
  Color,
  LineComment,
  MultiLineComment,
}

#[derive(Debug, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub pos: Span,
  pub text: Option<LexedStr>,
}

#[derive(Debug, Default)]
pub struct Location {
  pub start_line: usize,
  pub start_pos: usize,
  pub end_line: usize,
  pub end_pos: usize,
}

pub fn get_location_from_position(text: &str, position: &Span) -> Location {
  let mut location = Location::default();
  let mut line_number = 1;
  let mut line_pos = 1;
  for (curr_pos, char) in text.chars().enumerate() {
    if curr_pos == position.start {
      location.start_line = line_number;
      location.start_pos = line_pos;
    }
    if curr_pos == position.end {
      location.end_line = line_number;
      location.end_pos = line_pos;
    }
    if char == '\n' {
      line_number += 1;
      line_pos = 0;
    }
    line_pos += 1;
  }
  location
}

#[tracing::instrument(name = "lexer")]
pub fn lex(text: &str) -> Result<Vec<Token>> {
  let mut lexer = TokenLexer::lexer(text);
  let mut tokens: Vec<Token> = vec![];

  while let Some(lex_result) = lexer.next() {
    if lex_result.is_err() {
      let location = get_location_from_position(text, &lexer.span());
      return Err(anyhow!(format!(
        "[{}:{}]: Unknown token \"{}\"",
        location.start_line,
        location.end_line,
        lexer.slice(),
      )));
    }
    let token = lex_result.unwrap();
    let span = lexer.span();
    tokens.push(match token {
      TokenLexer::Bool(b) => Token {
        kind: TokenKind::Boolean(b),
        pos: span,
        text: None,
      },
      TokenLexer::Eol => Token {
        kind: TokenKind::EOL,
        pos: span,
        text: None,
      },
      TokenLexer::BraceOpen => Token {
        kind: TokenKind::BraceOpen,
        pos: span,
        text: None,
      },
      TokenLexer::BraceClose => Token {
        kind: TokenKind::BraceClose,
        pos: span,
        text: None,
      },
      TokenLexer::BracketOpen => Token {
        kind: TokenKind::BracketOpen,
        pos: span,
        text: None,
      },
      TokenLexer::BracketClose => Token {
        kind: TokenKind::BracketClose,
        pos: span,
        text: None,
      },
      TokenLexer::ParenOpen => Token {
        kind: TokenKind::ParenOpen,
        pos: span,
        text: None,
      },
      TokenLexer::ParenClose => Token {
        kind: TokenKind::ParenClose,
        pos: span,
        text: None,
      },
      TokenLexer::Colon => Token {
        kind: TokenKind::Colon,
        pos: span,
        text: None,
      },
      TokenLexer::Comma => Token {
        kind: TokenKind::Comma,
        pos: span,
        text: None,
      },
      TokenLexer::Plus => Token {
        kind: TokenKind::Plus,
        pos: span,
        text: None,
      },
      TokenLexer::Minus => Token {
        kind: TokenKind::Minus,
        pos: span,
        text: None,
      },
      TokenLexer::Div => Token {
        kind: TokenKind::Div,
        pos: span,
        text: None,
      },
      TokenLexer::Mul => Token {
        kind: TokenKind::Mul,
        pos: span,
        text: None,
      },
      TokenLexer::Hash => Token {
        kind: TokenKind::Hash,
        pos: span,
        text: None,
      },
      TokenLexer::Percent => Token {
        kind: TokenKind::Percent,
        pos: span,
        text: None,
      },
      TokenLexer::Equal => Token {
        kind: TokenKind::Equal,
        pos: span,
        text: None,
      },
      TokenLexer::NotEqual => Token {
        kind: TokenKind::NotEqual,
        pos: span,
        text: None,
      },
      TokenLexer::LessThan => Token {
        kind: TokenKind::LessThan,
        pos: span,
        text: None,
      },
      TokenLexer::MoreThan => Token {
        kind: TokenKind::MoreThan,
        pos: span,
        text: None,
      },
      TokenLexer::LessThanOrEqual => Token {
        kind: TokenKind::LessThanOrEqual,
        pos: span,
        text: None,
      },
      TokenLexer::MoreThanOrEqual => Token {
        kind: TokenKind::MoreThanOrEqual,
        pos: span,
        text: None,
      },
      TokenLexer::And => Token {
        kind: TokenKind::And,
        pos: span,
        text: None,
      },
      TokenLexer::Or => Token {
        kind: TokenKind::Or,
        pos: span,
        text: None,
      },
      TokenLexer::Number(n) => Token {
        kind: TokenKind::Number(n),
        pos: span,
        text: None,
      },
      TokenLexer::String(s) => Token {
        kind: TokenKind::String,
        pos: span,
        text: Some(s.clone()),
      },
      TokenLexer::Identifier(i) => Token {
        kind: TokenKind::Identifier,
        pos: span,
        text: Some(i.clone()),
      },
      TokenLexer::Reference(r) => Token {
        kind: TokenKind::Reference,
        pos: span,
        text: Some(r.clone()),
      },
      TokenLexer::HierarchyReference(r) => Token {
        kind: TokenKind::HierarchyReference,
        pos: span,
        text: Some(r.clone()),
      },
      TokenLexer::Color(c) => Token {
        kind: TokenKind::Color,
        pos: span,
        text: Some(c.clone()),
      },
      TokenLexer::LineComment(l) => Token {
        kind: TokenKind::LineComment,
        pos: span,
        text: Some(l.clone()),
      },
      TokenLexer::MultiLineComment(l) => Token {
        kind: TokenKind::MultiLineComment,
        pos: span,
        text: Some(l.clone()),
      },
    });
  }
  Ok(tokens)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn gives_error() {
    let tokens = lex("&&&&");
    assert!(tokens.is_err());
    let err = tokens.unwrap_err();
    assert_eq!(format!("{}", err), "[1:1]: Unknown token \"&\"");
  }

  #[test]
  fn can_parse_strings() {
    let tokens = lex("\"hello \"");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::String,
        text: Some("\"hello \"".into()),
        pos: 0..8
      }
    );
  }
  #[test]
  fn can_parse_mixed_strings() {
    let tokens = lex("\"'hello' \"");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::String,
        text: Some("\"'hello' \"".into()),
        pos: 0..10
      }
    );
  }
  #[test]
  fn can_parse_multiline_strings() {
    let tokens = lex("\"hello\n\n world \"");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::String,
        text: Some("\"hello\n\n world \"".into()),
        pos: 0..16
      }
    );
  }

  #[test]
  fn can_parse_quote_strings() {
    let tokens = lex("'hello '");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::String,
        text: Some("'hello '".into()),
        pos: 0..8
      }
    );
  }

  #[test]
  fn can_parse_quote_multiline_strings() {
    let tokens = lex("'hello\n\n world '");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::String,
        text: Some("'hello\n\n world '".into()),
        pos: 0..16
      }
    );
  }

  #[test]
  fn can_parse_identifiers() {
    let tokens = lex("hello another1 with.dot _with_underscore-and3245");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::Identifier,
        text: Some("hello".into()),
        pos: 0..5
      }
    );
    assert_eq!(
      res[1],
      Token {
        kind: TokenKind::Identifier,
        text: Some("another1".into()),
        pos: 6..14
      }
    );
    assert_eq!(
      res[2],
      Token {
        kind: TokenKind::Identifier,
        text: Some("with.dot".into()),
        pos: 15..23
      }
    );
    assert_eq!(
      res[3],
      Token {
        kind: TokenKind::Identifier,
        text: Some("_with_underscore-and3245".into()),
        pos: 24..48
      }
    );
  }
  #[test]
  fn can_parse_operators() {
    let tokens = lex("AND and or OR");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::And,
        text: None,
        pos: 0..3
      }
    );
    assert_eq!(
      res[1],
      Token {
        kind: TokenKind::And,
        text: None,
        pos: 4..7
      }
    );
    assert_eq!(
      res[2],
      Token {
        kind: TokenKind::Or,
        text: None,
        pos: 8..10
      }
    );
    assert_eq!(
      res[3],
      Token {
        kind: TokenKind::Or,
        text: None,
        pos: 11..13
      }
    );
  }

  #[test]
  fn can_parse_line_comments() {
    let tokens = lex("// hello comment");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::LineComment,
        text: Some("// hello comment".into()),
        pos: 0..16
      }
    );
  }

  #[test]
  fn can_parse_multiline_comments() {
    let tokens = lex("/* hello  \n\n comment */");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::MultiLineComment,
        text: Some("/* hello  \n\n comment */".into()),
        pos: 0..23
      }
    );
  }

  #[test]
  fn can_parse_eol() {
    let tokens = lex("hello\nanother1\n");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[1],
      Token {
        kind: TokenKind::EOL,
        text: None,
        pos: 5..6
      }
    );

    assert_eq!(
      res[3],
      Token {
        kind: TokenKind::EOL,
        text: None,
        pos: 14..15
      }
    );
  }

  #[test]
  fn can_parse_reference() {
    let tokens = lex("@hello @another1 @with.dot");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::Reference,
        text: Some("hello".into()),
        pos: 0..6
      }
    );
    assert_eq!(
      res[1],
      Token {
        kind: TokenKind::Reference,
        text: Some("another1".into()),
        pos: 7..16
      }
    );
    assert_eq!(
      res[2],
      Token {
        kind: TokenKind::Reference,
        text: Some("with.dot".into()),
        pos: 17..26
      }
    );
  }

  #[test]
  fn can_parse_color() {
    let tokens = lex("#112233 #acFF23");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::Color,
        text: Some("112233".into()),
        pos: 0..7
      }
    );
    assert_eq!(
      res[1],
      Token {
        kind: TokenKind::Color,
        text: Some("acFF23".into()),
        pos: 8..15
      }
    );
  }

  #[test]
  fn can_parse_vpath() {
    let tokens = lex("SitesHierarchySimplified:^hierarchy");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::Identifier,
        text: Some("SitesHierarchySimplified".into()),
        pos: 0..24
      }
    );
    assert_eq!(
      res[1],
      Token {
        kind: TokenKind::Colon,
        text: None,
        pos: 24..25
      }
    );
    assert_eq!(
      res[2],
      Token {
        kind: TokenKind::HierarchyReference,
        text: Some("hierarchy".into()),
        pos: 25..35
      }
    );
  }

  #[test]
  fn can_parse_booleans() {
    let tokens = lex("true false").unwrap();
    assert_eq!(
      tokens[0],
      Token {
        kind: TokenKind::Boolean(true),
        text: None,
        pos: 0..4
      }
    );
    assert_eq!(
      tokens[1],
      Token {
        kind: TokenKind::Boolean(false),
        text: None,
        pos: 5..10
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
        text: None,
        pos: 0..1
      }
    );
    assert_eq!(
      tokens[1],
      Token {
        kind: TokenKind::BraceOpen,
        text: None,
        pos: 2..3
      }
    );
    assert_eq!(
      tokens[2],
      Token {
        kind: TokenKind::BracketOpen,
        text: None,
        pos: 4..5
      }
    );
    assert_eq!(
      tokens[3],
      Token {
        kind: TokenKind::ParenClose,
        text: None,
        pos: 6..7
      }
    );
    assert_eq!(
      tokens[4],
      Token {
        kind: TokenKind::BraceClose,
        text: None,
        pos: 8..9
      }
    );
    assert_eq!(
      tokens[5],
      Token {
        kind: TokenKind::BracketClose,
        text: None,
        pos: 10..11
      }
    );
  }

  #[test]
  fn can_parse_large_file() {
    let file = include_str!("../../../test_script/test.cdl");
    let tokens = lex(file);
    if tokens.is_err() {
      dbg!(&tokens);
    }

    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(147203, res.len());
  }
  #[test]
  fn can_parse_number() {
    let tokens = lex("1 1.1 -3245.2").unwrap();

    assert_eq!(
      tokens[0],
      Token {
        kind: TokenKind::Number(1.0),
        text: None,
        pos: 0..1
      }
    );
    assert_eq!(
      tokens[1],
      Token {
        kind: TokenKind::Number(1.1),
        text: None,
        pos: 2..5
      }
    );
    assert_eq!(
      tokens[2],
      Token {
        kind: TokenKind::Number(-3245.2),
        text: None,
        pos: 6..13
      }
    );
  }
}
