use std::net::Shutdown::Read;

use identifier_matcher::EntityIdMatcher;
use identifier_matcher::IdentifierMatcher;
use identifier_matcher::NumberMatcher;
use identifier_matcher::ReferenceMatcher;
use literal_matcher::LiteralMatcher;
use matcher::Matcher;
use matcher::ParseResult;
use whitespace_matcher::WhitespaceMatcher;

use crate::cdl::lexer::identifier_matcher::StringMatcher;
use std::intrinsics::transmute;

mod identifier_matcher;
mod literal_matcher;
mod matcher;
mod whitespace_matcher;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TokenType {
  Identifier,
  EntityId,
  Reference,
  String,
  Number,
  Colon,
  Comma,
  Equal,
  LessThan,
  MoreThan,
  Percent,
  OpenBracket,
  CloseBracket,
  OpenParen,
  CloseParen,
  Plus,
  Minus,
  Hash,
  Div,
  Mul,
  EOL,
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
//        (Box::new(EntityIdMatcher::new()), TokenType::EntityId),
        (Box::new(NumberMatcher::new()), TokenType::Number),
        (Box::new(StringMatcher::new('"')), TokenType::String),
        (Box::new(StringMatcher::new('\'')), TokenType::String),
        (Box::new(LiteralMatcher::new(":")), TokenType::Colon),
        (Box::new(LiteralMatcher::new(",")), TokenType::Comma),
        (Box::new(LiteralMatcher::new("=")), TokenType::Equal),
        (Box::new(LiteralMatcher::new("<")), TokenType::LessThan),
        (Box::new(LiteralMatcher::new(">")), TokenType::MoreThan),
        (Box::new(LiteralMatcher::new("%")), TokenType::Percent),
        (Box::new(LiteralMatcher::new("{")), TokenType::OpenBracket),
        (Box::new(LiteralMatcher::new("}")), TokenType::CloseBracket),
        (Box::new(LiteralMatcher::new("(")), TokenType::OpenParen),
        (Box::new(LiteralMatcher::new(")")), TokenType::CloseParen),
        (Box::new(LiteralMatcher::new("+")), TokenType::Plus),
        (Box::new(LiteralMatcher::new("-")), TokenType::Minus),
        (Box::new(LiteralMatcher::new("/")), TokenType::Div),
        (Box::new(LiteralMatcher::new("*")), TokenType::Mul),
        (Box::new(LiteralMatcher::new("#")), TokenType::Hash),
        (Box::new(LiteralMatcher::new("\n")), TokenType::EOL),
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
        end: 20,
        kind: TokenType::Hash,
      },
      Token {
        start: 20,
        end: 25,
        kind: TokenType::Identifier,
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
        start: 8,
        end: 9,
        kind: TokenType::EOL,
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
fn lexer_lots() {
  let lexer = Lexer::new();
  let lexed = lexer.lex("value: MAX(survey:Q2,survey:interview_start=max(survey:interview_start))
value: average(score(survey:Q7), @cr.currentPeriodB2b)
thresholds: #82D854 >= 100%, #FFBD5B >= 80%, #FA5263 < 80%
riskValue: IIF(average(SCORE(survey:Q1))<7,'H!',IIF(average(SCORE(survey:Q1))>8,'L',IIF(COUNT(survey:responseid)<1,'U','M')))".to_string());
  assert_eq!(lexed.is_ok(), true);
  assert_eq!(lexed.unwrap().len(), 81);
}

#[test]
fn lexer_lots_timing() {
  let start = std::time::Instant::now();
  let lexer = Lexer::new();
  lexer.lex("value: MAX(survey:Q2,survey:interview_start=max(survey:interview_start))
value: average(score(survey:Q7), @cr.currentPeriodB2b)
thresholds: #82D854 >= 100%, #FFBD5B >= 80%, #FA5263 < 80%
riskValue: IIF(average(SCORE(survey:Q1))<7,'H!',IIF(average(SCORE(survey:Q1))>8,'L',IIF(COUNT(survey:responseid)<1,'U','M')))".to_string());
  let dur = start.elapsed();
  assert_eq!(dur < std::time::Duration::new(0,500000), true);

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
          kind: TokenType::Identifier,
        },
        Token {
          start: 11,
          end: 16,
          kind: TokenType::Identifier,
        }
      ]
    )),
    lexer.lex("   hello ! hello".to_string())
  );
}
