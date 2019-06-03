mod identifier_matcher;
mod literal_matcher;
mod matcher;
mod whitespace_matcher;

use identifier_matcher::IdentifierMatcher;
use literal_matcher::LiteralMatcher;
use matcher::Matcher;
use matcher::ParseResult;
use whitespace_matcher::WhitespaceMatcher;

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
  Identifier,
  OpenBracket,
}

#[derive(Debug, PartialEq)]
struct Token {
  start: usize,
  end: usize,
  kind: TokenType,
}

#[derive(Debug)]
struct Lexer {}

impl Lexer {
  pub fn new() -> Lexer {
    Lexer {}
  }

  pub fn lex(&self, input: String) -> Result<Vec<Token>, &str> {
    let mut current_pos = 0;
    let identifier_matcher = IdentifierMatcher::new();
    let open_bracket_matcher = LiteralMatcher::new("{");
    let whitespace_matcher = WhitespaceMatcher::new();
    let end_pos = input.len();
    let mut result = vec![];
    while current_pos < end_pos {
      match whitespace_matcher.check(&input[current_pos..]) {
        Ok(0) => {}
        Ok(len) => current_pos += len,
        Err(_) => return Err("ran out of input lexing whitespace"),
      }
      if current_pos == end_pos {
        break;
      };
      match identifier_matcher.check(&input[current_pos..]) {
        Ok(0) => {}
        Ok(pos) => {
          result.push(Token {
            start: current_pos,
            end: current_pos + pos,
            kind: TokenType::Identifier,
          });
          current_pos += pos;
          continue;
        }
        Err(_) => {}
      }
      match open_bracket_matcher.check(&input[current_pos..]) {
        Ok(0) => {}
        Ok(pos) => {
          result.push(Token {
            start: current_pos,
            end: current_pos + pos,
            kind: TokenType::OpenBracket,
          });
          current_pos += pos;
          continue;
        }
        Err(_) => {}
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
      }
    ]),
    lexer.lex("   hello { ".to_string())
  );
}
