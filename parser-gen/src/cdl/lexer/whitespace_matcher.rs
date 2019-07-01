use crate::cdl::lexer::matcher::Matcher;

#[derive(Debug)]
pub struct WhitespaceMatcher {}

impl WhitespaceMatcher {
  pub fn new() -> WhitespaceMatcher {
    WhitespaceMatcher {}
  }
}

impl Matcher for WhitespaceMatcher {
  fn check(&self, input: &str) -> Result<(usize, Option<String>), &str> {
    let mut chars = input.chars();
    let mut len = 0;
    loop {
      let next = chars.next();
      match next {
        Some(c) => {
          if c.is_whitespace() && c != '\n' {
            len += 1;
          } else {
            return Ok((len, None));
          }
        }
        _ => return Ok((len, None)),
      }
    }
  }
}

#[test]
fn whitespace_matcher() {
  let matcher = WhitespaceMatcher::new();
  assert_eq!(Ok((0, None)), matcher.check("hallo"));
  assert_eq!(Ok((3, None)), matcher.check("   hallo"));
  assert_eq!(Ok((1, None)), matcher.check(" "));
  assert_eq!(Ok((2, None)), matcher.check("  "));
  assert_eq!(Ok((1, None)), matcher.check(" \n "));
}
