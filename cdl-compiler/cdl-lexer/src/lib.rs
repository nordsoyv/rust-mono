use anyhow::anyhow;
use anyhow::Result;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
enum TokenLexer<'a> {
  #[token("false", |_| false)]
  #[token("true", |_| true)]
  Bool(bool),

  #[token("\n")]
  EOL,

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

  #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
  Number(f64),

  #[regex(r#""(?:[^"]|\\")*""#, |lex| lex.slice())]
  #[regex(r#"'(?:[^']|\\')*'"#, |lex| lex.slice())]
  String(&'a str),

  #[regex("_?[a-zA-Z0-9_\\-\\.]*", |lex| lex.slice())]
  Identifier(&'a str),
  
  #[regex("@[a-zA-Z0-9_\\-\\.]*", |lex| &lex.slice()[1..])]
  Reference(&'a str),
  
  #[regex("#[0-9a-fA-F]{6}", |lex| &lex.slice()[1..])]
  Color(&'a str),

  #[regex("//[^\n]*", |lex| lex.slice())]
  LineComment(&'a str),

  #[regex(r#"/\*(?:[^*]|\*[^/])*\*/"#, |lex| lex.slice())]
  MultiLineComment(&'a str),  
}

#[derive(Debug, PartialEq)]
pub enum TokenKind<'a> {
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
  String(&'a str),
  Plus,
  Minus,
  Div,
  Mul,
  Hash,
  Percent,
  NotEqual,
  LessThan,
  LessThanOrEqual,
  MoreThan,
  MoreThanOrEqual,
  Identifier(&'a str),
  Reference(&'a str),
  Color(&'a str),
  LineComment(&'a str),
  MultiLineComment(&'a str),  
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
  kind: TokenKind<'a>,
  start_pos: usize,
  end_pos: usize,
}

pub fn lex(text: &str) -> Result<Vec<Token>> {
  let mut lexer = TokenLexer::lexer(text);
  let mut tokens: Vec<Token> = vec![];

  while let Some(lex_result) = lexer.next() {
    if lex_result.is_err() {
      return Err(anyhow!(format!("Unknown token \"{}\"", lexer.slice())));
    }
    let token = lex_result.unwrap();
    let span = lexer.span();
    tokens.push(match token {
      TokenLexer::Bool(b) => Token {
        kind: TokenKind::Boolean(b),
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::EOL => Token {
        kind: TokenKind::EOL,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::BraceOpen => Token {
        kind: TokenKind::BraceOpen,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::BraceClose => Token {
        kind: TokenKind::BraceClose,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::BracketOpen => Token {
        kind: TokenKind::BracketOpen,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::BracketClose => Token {
        kind: TokenKind::BracketClose,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::ParenOpen => Token {
        kind: TokenKind::ParenOpen,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::ParenClose => Token {
        kind: TokenKind::ParenClose,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Colon => Token {
        kind: TokenKind::Colon,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Comma => Token {
        kind: TokenKind::Comma,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Plus => Token {
        kind: TokenKind::Plus,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Minus => Token {
        kind: TokenKind::Minus,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Div => Token {
        kind: TokenKind::Div,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Mul => Token {
        kind: TokenKind::Mul,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Hash => Token {
        kind: TokenKind::Hash,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Percent => Token {
        kind: TokenKind::Percent,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::NotEqual => Token {
        kind: TokenKind::NotEqual,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::LessThan => Token {
        kind: TokenKind::LessThan,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::MoreThan => Token {
        kind: TokenKind::MoreThan,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::LessThanOrEqual => Token {
        kind: TokenKind::LessThanOrEqual,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::MoreThanOrEqual => Token {
        kind: TokenKind::MoreThanOrEqual,
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Number(n) => Token {
        kind: TokenKind::Number(n),
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::String(s) => Token {
        kind: TokenKind::String(s),
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Identifier(i) => Token {
        kind: TokenKind::Identifier(i),
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Reference(r) => Token {
        kind: TokenKind::Reference(r),
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::Color(c) => Token {
        kind: TokenKind::Color(c),
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::LineComment(l) => Token {
        kind: TokenKind::LineComment(l),
        start_pos: span.start,
        end_pos: span.end,
      },
      TokenLexer::MultiLineComment(l) => Token {
        kind: TokenKind::MultiLineComment(l),
        start_pos: span.start,
        end_pos: span.end,
      },
    });
  }
  return Ok(tokens);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn gives_error() {
    let tokens = lex("&&&&");
    assert!(tokens.is_err());
    let err = tokens.unwrap_err();
    assert_eq!(format!("{}", err), "Unknown token \"&\"");
  }

  #[test]
  fn can_parse_strings() {
    let tokens = lex("\"hello \"");
    assert!(tokens.is_ok());
    let res = tokens.unwrap();
    assert_eq!(
      res[0],
      Token {
        kind: TokenKind::String("\"hello \""),
        start_pos: 0,
        end_pos: 8
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
        kind: TokenKind::String("\"'hello' \""),
        start_pos: 0,
        end_pos: 10
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
        kind: TokenKind::String("\"hello\n\n world \""),
        start_pos: 0,
        end_pos: 16
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
        kind: TokenKind::String("'hello '"),
        start_pos: 0,
        end_pos: 8
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
        kind: TokenKind::String("'hello\n\n world '"),
        start_pos: 0,
        end_pos: 16
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
        kind: TokenKind::Identifier("hello"),
        start_pos: 0,
        end_pos: 5
      }
    );
    assert_eq!(
      res[1],
      Token {
        kind: TokenKind::Identifier("another1"),
        start_pos: 6,
        end_pos: 14
      }
    );
    assert_eq!(
      res[2],
      Token {
        kind: TokenKind::Identifier("with.dot"),
        start_pos: 15,
        end_pos: 23
      }
    );
    assert_eq!(
      res[3],
      Token {
        kind: TokenKind::Identifier("_with_underscore-and3245"),
        start_pos: 24,
        end_pos: 48
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
        kind: TokenKind::LineComment("// hello comment"),
        start_pos: 0,
        end_pos: 16
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
        kind: TokenKind::MultiLineComment("/* hello  \n\n comment */"),
        start_pos: 0,
        end_pos: 23
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
        start_pos: 5,
        end_pos: 6
      }
    );

    assert_eq!(
      res[3],
      Token {
        kind: TokenKind::EOL,
        start_pos: 14,
        end_pos: 15
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
        kind: TokenKind::Reference("hello"),
        start_pos: 0,
        end_pos: 6
      }
    );
    assert_eq!(
      res[1],
      Token {
        kind: TokenKind::Reference("another1"),
        start_pos: 7,
        end_pos: 16
      }
    );
    assert_eq!(
      res[2],
      Token {
        kind: TokenKind::Reference("with.dot"),
        start_pos: 17,
        end_pos: 26
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
        kind: TokenKind::Color("112233"),
        start_pos: 0,
        end_pos: 7
      }
    );
    assert_eq!(
      res[1],
      Token {
        kind: TokenKind::Color("acFF23"),
        start_pos: 8,
        end_pos: 15
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
        start_pos: 0,
        end_pos: 4,
      }
    );
    assert_eq!(
      tokens[1],
      Token {
        kind: TokenKind::Boolean(false),
        start_pos: 5,
        end_pos: 10,
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
      }
    );
    assert_eq!(
      tokens[1],
      Token {
        kind: TokenKind::BraceOpen,
        start_pos: 2,
        end_pos: 3,
      }
    );
    assert_eq!(
      tokens[2],
      Token {
        kind: TokenKind::BracketOpen,
        start_pos: 4,
        end_pos: 5,
      }
    );
    assert_eq!(
      tokens[3],
      Token {
        kind: TokenKind::ParenClose,
        start_pos: 6,
        end_pos: 7,
      }
    );
    assert_eq!(
      tokens[4],
      Token {
        kind: TokenKind::BraceClose,
        start_pos: 8,
        end_pos: 9,
      }
    );
    assert_eq!(
      tokens[5],
      Token {
        kind: TokenKind::BracketClose,
        start_pos: 10,
        end_pos: 11,
      }
    );
  }
}
