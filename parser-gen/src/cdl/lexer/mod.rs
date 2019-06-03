mod identifier_matcher;
mod literal_matcher;
mod matcher;
mod whitespace_matcher;

use identifier_matcher::IdentifierMatcher;
use literal_matcher::LiteralMatcher;
use matcher::Matcher;
use matcher::ParseResult;
use whitespace_matcher::WhitespaceMatcher;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TokenType {
  Identifier,
  OpenBracket,
  CloseBracket,
}

#[derive(Debug, PartialEq)]
struct Token {
  start: usize,
  end: usize,
  kind: TokenType,
}

struct Lexer {
  matchers: Vec<(Box<dyn Matcher>, TokenType)>,
}

impl Lexer {
  pub fn new() -> Lexer {
    Lexer {
      matchers: vec![
        (Box::new(IdentifierMatcher::new()), TokenType::Identifier),
        (Box::new(LiteralMatcher::new("{")), TokenType::OpenBracket),
        (Box::new(LiteralMatcher::new("}")), TokenType::CloseBracket),
      ],
    }
  }

  pub fn lex(&self, input: String) -> Result<Vec<Token>, &str> {
    let whitespace_matcher = WhitespaceMatcher::new();
    let mut current_pos = 0;
    let end_pos = input.len();
    let mut result = vec![];
    'chars: while current_pos < end_pos {
      match whitespace_matcher.check(&input[current_pos..]) {
        Ok(0) => {}
        Ok(len) => current_pos += len,
        Err(_) => return Err("ran out of input lexing whitespace"),
      }
      if current_pos == end_pos {
        break 'chars;
      };

      for (m, token_type) in &self.matchers {
        match m.check(&input[current_pos..]) {
          Ok(0) => {}
          Ok(pos) => {
            result.push(Token {
              start: current_pos,
              end: current_pos + pos,
              kind: *token_type,
            });
            current_pos += pos;
            continue 'chars;
          }
          Err(_) => {}
        }
      }
    }

    return Ok(result);
  }
}

#[test]
fn lexer_parse_whitespace() {
  let lexer = Lexer::new();
  let res = lexer.lex("        ".to_string());
  assert_eq!(Ok(vec![]), res);
}

#[test]
fn lexer_parse_literal() {
  let lexer = Lexer::new();
  let res = lexer.lex("   {     ".to_string());
  assert_eq!(
    Ok(vec![Token {
      start: 3,
      end: 4,
      kind: TokenType::OpenBracket
    }]),
    res
  );
}

#[test]
fn lexer_parse_identifier() {
  let lexer = Lexer::new();
  assert_eq!(
    Ok(vec![Token {
      start: 3,
      end: 8,
      kind: TokenType::Identifier
    }]),
    lexer.lex("   hello     ".to_string())
  );
  assert_eq!(
    Ok(vec![
      Token {
        start: 3,
        end: 8,
        kind: TokenType::Identifier
      },
      Token {
        start: 12,
        end: 17,
        kind: TokenType::Identifier
      }
    ]),
    lexer.lex("   hello    hello ".to_string())
  );
}

#[test]
fn lexer_multiline() {
  let lexer = Lexer::new();
  assert_eq!(
    Ok(vec![
      Token {
        start: 3,
        end: 8,
        kind: TokenType::Identifier
      },
      Token {
        start: 10,
        end: 15,
        kind: TokenType::Identifier
      }
    ]),
    lexer.lex("   hello\n hello ".to_string())
  );
}

#[test]
fn lexer_parse_identifier_and_literal() {
  let lexer = Lexer::new();
  assert_eq!(
    Ok(vec![
      Token {
        start: 3,
        end: 8,
        kind: TokenType::Identifier
      },
      Token {
        start: 9,
        end: 10,
        kind: TokenType::OpenBracket
      },
      Token {
        start: 10,
        end: 11,
        kind: TokenType::CloseBracket
      }
    ]),
    lexer.lex("   hello {} ".to_string())
  );
}
