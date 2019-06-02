pub type ParseResult<'a> = Result<usize, &'a str>;

#[derive(Debug, PartialEq, Eq)]
enum LexToken {
  Identifier(usize, usize),
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

  pub fn lex(&self, input: String) -> Result<Vec<LexToken>, &str> {
    let mut current_pos = 0;
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
      match identifier(&input[current_pos..]) {
        Ok(pos) => {
          result.push(LexToken::Identifier(current_pos, current_pos + pos));
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
  fn new(literal: &'static str)-> LiteralMatcher {
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

fn identifier(input: &str) -> ParseResult {
  let mut matched = String::new();
  let mut chars = input.chars();

  match chars.next() {
    Some(next) if next.is_alphabetic() => matched.push(next),
    _ => return Err("Not an identifier"),
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
fn identifier_parser() {
  assert_eq!(Ok(18), identifier("i-am-an-identifier"));
  assert_eq!(Ok(3), identifier("not entirely an identifier"));
  assert_eq!(
    Err("Not an identifier"),
    identifier("!not at all an identifier")
  );
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
    Ok(vec![LexToken::Identifier(3, 8)]),
    lexer.lex("   hello     ".to_string())
  );
  assert_eq!(
    Ok(vec![
      LexToken::Identifier(3, 8),
      LexToken::Identifier(12, 17)
    ]),
    lexer.lex("   hello    hello ".to_string())
  );
}

#[test]
fn literal_matcher(){
  let matcher = LiteralMatcher::new("{");
  assert_eq!(matcher.check("{not"), Ok(1));
  assert_eq!(matcher.check("not"), Ok(0));
  let matcher = LiteralMatcher::new("not");
  assert_eq!(matcher.check("not"), Ok(3));
  assert_eq!(matcher.check("!not"), Ok(0));
  assert_eq!(matcher.check(""), Err("unexpected end of input"));

}