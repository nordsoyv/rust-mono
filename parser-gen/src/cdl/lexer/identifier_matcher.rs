use crate::cdl::lexer::matcher::Matcher;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ReferenceMatcher {}

impl ReferenceMatcher {
  pub fn new() -> ReferenceMatcher {
    ReferenceMatcher {}
  }
}

impl Matcher for ReferenceMatcher {
  fn check(&self, input: &str) -> Result<usize, &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
      Some(next) => {
        if next == '@' {
          matched.push(next)
        } else {
          return Ok(0);
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
    Ok(next_index)
  }
}

#[derive(Debug)]
pub struct EntityIdMatcher {}

impl EntityIdMatcher {
  pub fn new() -> EntityIdMatcher {
    EntityIdMatcher {}
  }
}

impl Matcher for EntityIdMatcher {
  fn check(&self, input: &str) -> Result<usize, &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
      Some(next) => {
        if next == '#' {
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

#[derive(Debug)]
pub struct NumberMatcher {}

impl NumberMatcher {
  pub fn new() -> NumberMatcher {
    NumberMatcher {}
  }
}

impl Matcher for NumberMatcher {
  fn check(&self, input: &str) -> Result<usize, &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
      Some(next) => {
        if next.is_numeric() {
          matched.push(next)
        } else {
          return Ok(0);
        }
      }
      _ => return Err("EOF"),
    }

    while let Some(next) = chars.next() {
      if next.is_numeric() {
        matched.push(next);
      } else {
        break;
      }
    }

    let next_index = matched.len();
    Ok(next_index)
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
  fn check(&self, input: &str) -> Result<usize, &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
      Some(next) => {
        if next != self.quote_char {
          return Ok(0);
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
    Ok(next_index + 2)
  }
}

#[test]
fn identifier_matcher() {
  let i_matcher = IdentifierMatcher::new();
  assert_eq!(Ok(18), i_matcher.check("i-am-an-identifier"));
  assert_eq!(Ok(3), i_matcher.check("not entirely an identifier"));
  assert_eq!(Ok(0), i_matcher.check("!not at all an identifier"));
}

#[test]
fn reference_matcher() {
  let i_matcher = ReferenceMatcher::new();
  assert_eq!(Ok(4), i_matcher.check("@ref"));
  assert_eq!(Ok(11), i_matcher.check("@path.match entirely an identifier"));
  assert_eq!(Ok(0), i_matcher.check("identifier"));
}

#[test]
fn entity_id_matcher() {
  let i_matcher = EntityIdMatcher::new();
  assert_eq!(Ok(3), i_matcher.check("#id"));
  assert_eq!(Ok(4), i_matcher.check("#not entirely an identifier"));
  assert_eq!(Ok(0), i_matcher.check("identifier"));
}

#[test]
fn number_matcher() {
  let i_matcher = NumberMatcher::new();
  assert_eq!(Ok(4), i_matcher.check("1234"));
  assert_eq!(Ok(4), i_matcher.check("1234 entirely an identifier"));
  assert_eq!(Ok(4), i_matcher.check("1234qerf"));
  assert_eq!(Ok(0), i_matcher.check("qerf"));
}

#[test]
fn string_matcher() {
  let i_matcher = StringMatcher::new('"');
  assert_eq!(Ok(6), i_matcher.check("\"1234\""));
  assert_eq!(Ok(6), i_matcher.check("\"1234\"        "));
  let i_matcher = StringMatcher::new('\'');
  assert_eq!(Ok(6), i_matcher.check("'1234'"));
  assert_eq!(Ok(6), i_matcher.check("'1234'        "));
}
