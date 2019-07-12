use crate::lexer::matcher::Matcher;

#[derive(Debug)]
pub struct IdentifierMatcher {}

impl IdentifierMatcher {
  pub fn new() -> IdentifierMatcher {
    IdentifierMatcher {}
  }
}

impl Matcher for IdentifierMatcher {
  fn check(&self, input: &str) -> Result<(usize, Option<String>), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
      Some(next) => {
        if next.is_alphabetic() {
          matched.push(next)
        } else {
          return Ok((0, None));
        }
      }
      _ => return Err("EOF"),
    }

    while let Some(next) = chars.next() {
      if next.is_alphanumeric() || next == '-' || next == '_' || next == '.' {
        matched.push(next);
      } else {
        break;
      }
    }

    let next_index = matched.len();
    Ok((next_index, Some(matched)))
  }
}

#[derive(Debug)]
pub struct CommentsMatcher {}

impl CommentsMatcher {
  pub fn new() -> CommentsMatcher {
    CommentsMatcher {}
  }
}

impl Matcher for CommentsMatcher {
  fn check(&self, input: &str) -> Result<(usize, Option<String>), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
      Some(next) => {
        if next == '/' {
          matched.push(next)
        } else {
          return Ok((0, None));
        }
      }
      _ => return Err("EOF"),
    }

    match chars.next() {
      Some(next) => {
        if next == '/' {
          matched.push(next)
        } else {
          return Ok((0, None));
        }
      }
      _ => return Err("EOF"),
    }

    while let Some(next) = chars.next() {
      if next != '\n' {
        matched.push(next);
      } else {
        break;
      }
    }

    let next_index = matched.len();
    Ok((next_index, None))
  }
}

#[derive(Debug)]
pub struct ReferenceMatcher {}

impl ReferenceMatcher {
  pub fn new() -> ReferenceMatcher {
    ReferenceMatcher {}
  }
}

impl Matcher for ReferenceMatcher {
  fn check(&self, input: &str) -> Result<(usize, Option<String>), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
      Some(next) => {
        if next == '@' {
          matched.push(next)
        } else {
          return Ok((0, None));
        }
      }
      _ => return Err("EOF"),
    }

    while let Some(next) = chars.next() {
      if next.is_alphanumeric() || next == '-' || next == '_' || next == '.' {
        matched.push(next);
      } else {
        break;
      }
    }

    let next_index = matched.len();
    Ok((next_index, Some(matched[1..].to_string())))
  }
}

#[derive(Debug)]
pub struct ColorMatcher {}

impl ColorMatcher {
  pub fn new() -> ColorMatcher {
    ColorMatcher {}
  }
}

impl Matcher for ColorMatcher {
  fn check(&self, input: &str) -> Result<(usize, Option<String>), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();
    match chars.next() {
      Some(next) => {
        if next == '#' {
          matched.push(next);
        } else {
          return Ok((0, None));
        }
      }
      _ => return Err("EOF"),
    }

    while let Some(next) = chars.next() {
      if next.is_ascii_hexdigit() {
        matched.push(next);
      } else {
        break;
      }
    }

    let next_index = matched.len();
    if next_index != 7 { // all colors are 6 digits long
      return Ok((0, None));
    }
    Ok((next_index, Some(matched[1..].to_string())))
  }
}


#[derive(Debug)]
pub struct NumberMatcher {}

impl NumberMatcher {
  pub fn new() -> NumberMatcher {
    NumberMatcher {}
  }
}

impl Matcher for NumberMatcher {
  fn check(&self, input: &str) -> Result<(usize, Option<String>), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();
    match chars.next() {
      Some(next) => {
        if next.is_numeric() {
          matched.push(next)
        } else {
          return Ok((0, None));
        }
      }
      _ => return Err("EOF"),
    }

    while let Some(next) = chars.next() {
      if next.is_numeric() || next == '.' {
        matched.push(next);
      } else if next == '%' {
        matched.push(next);
        break;
      }else {
        break;
      }
    }

    let next_index = matched.len();
    Ok((next_index, Some(matched)))
  }
}

#[derive(Debug)]
pub struct StringMatcher {
  quote_char: char
}

impl StringMatcher {
  pub fn new(quote_char: char) -> StringMatcher {
    StringMatcher {
      quote_char
    }
  }
}

impl Matcher for StringMatcher {
  fn check(&self, input: &str) -> Result<(usize, Option<String>), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
      Some(next) => {
        if next != self.quote_char {
          return Ok((0, None));
        }
      }
      _ => return Err("EOF"),
    }

    while let Some(next) = chars.next() {
      if next != self.quote_char {
        matched.push(next);
      } else {
        break;
      }
    }

    let next_index = matched.len();
    Ok((next_index + 2, Some(matched)))
  }
}

#[test]
fn identifier_matcher() {
  let i_matcher = IdentifierMatcher::new();
  assert_eq!(Ok((18, Some("i-am-an-identifier".to_string()))), i_matcher.check("i-am-an-identifier"));
  assert_eq!(Ok((3, Some("not".to_string()))), i_matcher.check("not entirely an identifier"));
  assert_eq!(Ok((0, None)), i_matcher.check("!not at all an identifier"));
}

#[test]
fn reference_matcher() {
  let i_matcher = ReferenceMatcher::new();
  assert_eq!(Ok((4, Some("ref".to_string()))), i_matcher.check("@ref"));
  assert_eq!(Ok((11, Some("path.match".to_string()))), i_matcher.check("@path.match entirely an identifier"));
  assert_eq!(Ok((0, None)), i_matcher.check("identifier"));
}

#[test]
fn number_matcher() {
  let i_matcher = NumberMatcher::new();
  assert_eq!(Ok((4, Some("1234".to_string()))), i_matcher.check("1234"));
  assert_eq!(Ok((4, Some("1234".to_string()))), i_matcher.check("1234 entirely an identifier"));
  assert_eq!(Ok((4, Some("1234".to_string()))), i_matcher.check("1234qerf"));
  assert_eq!(Ok((5, Some("1234%".to_string()))), i_matcher.check("1234%"));
  assert_eq!(Ok((0, None)), i_matcher.check("qerf"));
}

#[test]
fn string_matcher() {
  let i_matcher = StringMatcher::new('"');
  assert_eq!(Ok((6, Some("1234".to_string()))), i_matcher.check("\"1234\""));
  assert_eq!(Ok((6, Some("1234".to_string()))), i_matcher.check("\"1234\"        "));
  let i_matcher = StringMatcher::new('\'');
  assert_eq!(Ok((6, Some("1234".to_string()))), i_matcher.check("'1234'"));
  assert_eq!(Ok((6, Some("1234".to_string()))), i_matcher.check("'1234'        "));
}


#[test]
fn comment_matcher() {
  let i_matcher = CommentsMatcher::new();
  assert_eq!(Ok((8, None)), i_matcher.check("// hello"));
  assert_eq!(Ok((9, None)), i_matcher.check("// hello \n not a comment"));
//  assert_eq!(Ok((6, Some("1234".to_string()))), i_matcher.check("\"1234\"        "));
//  let i_matcher = StringMatcher::new('\'');
//  assert_eq!(Ok((6, Some("1234".to_string()))), i_matcher.check("'1234'"));
//  assert_eq!(Ok((6, Some("1234".to_string()))), i_matcher.check("'1234'        "));
}

