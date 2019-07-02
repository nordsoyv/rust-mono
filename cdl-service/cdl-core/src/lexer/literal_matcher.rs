use crate::lexer::matcher::Matcher;

#[derive(Debug)]
pub struct LiteralMatcher {
  literal: &'static str,
}

impl LiteralMatcher {
  pub fn new(literal: &'static str) -> LiteralMatcher {
    LiteralMatcher { literal }
  }
}

impl Matcher for LiteralMatcher {
  fn check(&self, input: &str) -> Result<(usize, Option<String>), &str> {
    let len = self.literal.len();
    match input.get(0..len) {
      Some(next) => {
        if next == self.literal {
          Ok((len, None))
        } else {
          Ok((0, None))
        }
      }
      _ => Err("unexpected end of input"),
    }
  }
}

#[test]
fn literal_matcher() {
  let matcher = LiteralMatcher::new("{");
  assert_eq!(matcher.check("{not"), Ok((1,None)));
  assert_eq!(matcher.check("not"), Ok((0,None)));
  let matcher = LiteralMatcher::new("not");
  assert_eq!(matcher.check("not"), Ok((3, None)));
  assert_eq!(matcher.check("!not"), Ok((0, None)));
  assert_eq!(matcher.check(""), Err("unexpected end of input"));
}
