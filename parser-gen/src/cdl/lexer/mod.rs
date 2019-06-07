mod identifier_matcher;
mod literal_matcher;
mod matcher;
mod whitespace_matcher;

use identifier_matcher::IdentifierMatcher;
use identifier_matcher::EntityIdMatcher;
use identifier_matcher::ReferenceMatcher;
use identifier_matcher::NumberMatcher;
use literal_matcher::LiteralMatcher;
use matcher::Matcher;
use matcher::ParseResult;
use whitespace_matcher::WhitespaceMatcher;
use std::net::Shutdown::Read;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TokenType {
  Identifier,
  EntityId,
  Reference,
  OpenBracket,
  CloseBracket,
  Number,
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
        (Box::new(ReferenceMatcher::new()), TokenType::Reference),
        (Box::new(EntityIdMatcher::new()), TokenType::EntityId),
        (Box::new(NumberMatcher::new()), TokenType::Number),
        (Box::new(LiteralMatcher::new("{")), TokenType::OpenBracket),
        (Box::new(LiteralMatcher::new("}")), TokenType::CloseBracket),
      ],
    }
  }

  pub fn lex(
    &self,
    input: String,
  ) -> Result<Vec<Token>, (Vec<String>, Vec<Token>)> {
    let whitespace_matcher = WhitespaceMatcher::new();
    let mut current_pos = 0;
    let end_pos = input.len();
    let mut result = vec![];
    let mut errors = vec![];
    'chars: while current_pos < end_pos {
      match whitespace_matcher.check(&input[current_pos..]) {
        Ok(0) => {}
        Ok(len) => current_pos += len,
        Err(_) => {}
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
      // char we could not lex - skip to next
      errors.push(format!("Unknown char at pos {}", current_pos));
      current_pos += 1;
    }
    if errors.len() > 0 {
      return Err((errors, result));
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
      kind: TokenType::OpenBracket,
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
      kind: TokenType::Identifier,
    }]),
    lexer.lex("   hello     ".to_string())
  );
  assert_eq!(
    Ok(vec![
      Token {
        start: 3,
        end: 8,
        kind: TokenType::Identifier,
      },
      Token {
        start: 12,
        end: 17,
        kind: TokenType::Identifier,
      }
    ]),
    lexer.lex("   hello    hello ".to_string())
  );
}

#[test]
fn lexer_parse_identifier_entityid_reference() {
  let lexer = Lexer::new();
  assert_eq!(
    Ok(vec![
      Token {
        start: 3,
        end: 8,
        kind: TokenType::Identifier,
      },
      Token {
        start: 12,
        end: 18,
        kind: TokenType::Reference,
      },
      Token {
        start: 19,
        end: 25,
        kind: TokenType::EntityId,
      },
      Token {
        start: 26,
        end: 32,
        kind: TokenType::Number,
      }
    ]),
    lexer.lex("   hello    @hello #hello 121234".to_string())
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
        kind: TokenType::Identifier,
      },
      Token {
        start: 10,
        end: 15,
        kind: TokenType::Identifier,
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
        kind: TokenType::Identifier,
      },
      Token {
        start: 9,
        end: 10,
        kind: TokenType::OpenBracket,
      },
      Token {
        start: 10,
        end: 11,
        kind: TokenType::CloseBracket,
      }
    ]),
    lexer.lex("   hello {} ".to_string())
  );
}

#[test]
fn lexer_unknown_char() {
  let lexer = Lexer::new();
  assert_eq!(
    Err((
      vec!["Unknown char at pos 9".to_string()],
      vec![
        Token {
          start: 3,
          end: 8,
          kind: TokenType::Identifier
        },
        Token {
          start: 11,
          end: 16,
          kind: TokenType::Identifier
        }
      ]
    )),
    lexer.lex("   hello ! hello".to_string())
  );
}
