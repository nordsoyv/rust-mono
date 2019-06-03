use crate::cdl::lexer::matcher::Matcher;

pub struct IdentifierMatcher {}

impl IdentifierMatcher {
  pub fn new() -> IdentifierMatcher {
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
          return Ok(0);
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

#[test]
fn identifier_matcher() {
  let i_matcher = IdentifierMatcher::new();
  assert_eq!(Ok(18), i_matcher.check("i-am-an-identifier"));
  assert_eq!(Ok(3), i_matcher.check("not entirely an identifier"));
  assert_eq!(Ok(0), i_matcher.check("!not at all an identifier"));
}
