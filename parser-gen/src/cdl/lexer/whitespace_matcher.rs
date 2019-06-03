use crate::cdl::lexer::matcher::Matcher;

#[derive(Debug)]
pub struct WhitespaceMatcher {}

impl WhitespaceMatcher {
  pub fn new() -> WhitespaceMatcher {
    WhitespaceMatcher {}
  }
}

impl Matcher for WhitespaceMatcher {
  fn check(&self, input: &str) -> Result<usize, &str> {
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
}

#[test]
fn whitespace_matcher() {
  let matcher = WhitespaceMatcher::new();
  assert_eq!(Ok(0), matcher.check("hallo"));
  assert_eq!(Ok(3), matcher.check("   hallo"));
  assert_eq!(Ok(1), matcher.check(" "));
  assert_eq!(Ok(2), matcher.check("  "));
}
