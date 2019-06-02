pub type ParseResult<'a> = Result<usize, &'a str>;

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
  Identifier,
}

#[derive(Debug, PartialEq)]
struct Token {
  start: usize,
  end: usize,
  kind:TokenType,
}

#[derive(Debug)]
struct Lexer {}

trait Matcher {
  fn check(&self, input: &str) -> ParseResult;
}

impl Lexer {
  pub fn new() -> Lexer {
    Lexer {}
  }

  pub fn lex(&self, input: String) -> Result<Vec<Token>, &str> {
    let mut current_pos = 0;
    let identifier_matcher = IdentifierMatcher::new();
    let end_pos = input.len();
    let mut result = vec![];
    while current_pos < end_pos {
      dbg!(current_pos);
      match whitespace(&input[current_pos..]) {
        Ok(len) => current_pos += len,
        Err(_) => return Err("ran out of input lexing whitespace"),
      }
      if current_pos == end_pos {
        break;
      };
      match identifier_matcher.check(&input[current_pos..]) {
        Ok(pos) => {
          result.push(Token{start:current_pos, end: current_pos+pos, kind:TokenType::Identifier});
          //result.push(LexToken::Identifier(current_pos, current_pos + pos));
          current_pos += pos;
          continue;
        }
        Err(_) => {}
      }
    }

    return Ok(result);
  }
}

struct LiteralMatcher {
  literal: &'static str,
}

impl LiteralMatcher {
  fn new(literal: &'static str) -> LiteralMatcher {
    LiteralMatcher { literal }
  }
}

impl Matcher for LiteralMatcher {
  fn check(&self, input: &str) -> Result<usize, &str> {
    let len = self.literal.len();
    match input.get(0..len) {
      Some(next) => {
        if next == self.literal {
          Ok(len)
        } else {
          Ok(0)
        }
      }
      _ => Err("unexpected end of input"),
    }
  }
}

struct IdentifierMatcher {}

impl IdentifierMatcher {
  fn new() -> IdentifierMatcher {
    IdentifierMatcher {}
  }
}

impl Matcher for IdentifierMatcher {
  fn check(&self, input: &str) -> Result<usize, &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
      Some(next) => {
        if next.is_alphabetic() {
          matched.push(next)
        } else {
          return Ok(0)
        }
      }
      _ => return Err("EOF"),
    }

    while let Some(next) = chars.next() {
      if next.is_alphanumeric() || next == '-' || next == '_' {
        matched.push(next);
      } else {
        break;
      }
    }

    let next_index = matched.len();
    Ok(next_index)
  }
}

fn whitespace(input: &str) -> ParseResult {
  let mut chars = input.chars();
  let mut len = 0;
  loop {
    let next = chars.next();
    match next {
      Some(c) => {
        if c.is_whitespace() {
          len += 1;
        } else {
          return Ok(len);
        }
      }
      _ => return Ok(len),
    }
  }
}

#[test]
fn whitespace_parser() {
  assert_eq!(Ok(0), whitespace("hallo"));
  assert_eq!(Ok(3), whitespace("   hallo"));
  assert_eq!(Ok(1), whitespace(" "));
  assert_eq!(Ok(2), whitespace("  "));
}

#[test]
fn lexer_parse_whitespace() {
  let lexer = Lexer::new();
  let res = lexer.lex("        ".to_string());
  assert_eq!(Ok(vec![]), res);
}

#[test]
fn lexer_parse_identifier() {
  let lexer = Lexer::new();
  assert_eq!(
    Ok(vec![Token {start:3, end:8, kind:TokenType::Identifier }]),
    lexer.lex("   hello     ".to_string())
  );
  assert_eq!(
    Ok(vec![
      Token{ start:3, end:8, kind:TokenType::Identifier},
      Token{ start:12, end:17, kind:TokenType::Identifier}
    ]),
    lexer.lex("   hello    hello ".to_string())
  );
}

#[test]
fn literal_matcher() {
  let matcher = LiteralMatcher::new("{");
  assert_eq!(matcher.check("{not"), Ok(1));
  assert_eq!(matcher.check("not"), Ok(0));
  let matcher = LiteralMatcher::new("not");
  assert_eq!(matcher.check("not"), Ok(3));
  assert_eq!(matcher.check("!not"), Ok(0));
  assert_eq!(matcher.check(""), Err("unexpected end of input"));
}

#[test]
fn identifier_matcher() {
  let i_matcher = IdentifierMatcher::new();
  assert_eq!(Ok(18), i_matcher.check("i-am-an-identifier"));
  assert_eq!(Ok(3), i_matcher.check("not entirely an identifier"));
  assert_eq!(-
    Ok(0),
    i_matcher.check("!not at all an identifier")
  );
}
