use crate::common::{
  either, four, identifier, left, match_literal, one_or_more, optional, pair,
  quoted_string, right, space0, space1, three, whitespace_wrap, zero_or_more,
  Parser,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
  name: String,
  rhs: Rhs,
}

#[derive(Clone, Debug, PartialEq)]
enum Rhs {
  Number(f64),
  Identifier(String),
  QuotedString(String),
}

fn colon<'a>() -> impl Parser<'a, ()> {
  whitespace_wrap(match_literal(":"))
}

fn field<'a>() -> impl Parser<'a, Field> {
  three(identifier, colon(), identifier).map(move |(name, _, value)| Field {
    name,
    rhs: Rhs::Identifier(value),
  })
}

#[test]
fn match_field() {
  assert_eq!(
    Ok((
      "",
      Field {
        name: "name".to_string(),
        rhs: Rhs::Identifier("value".to_string()),
      }
    )),
    field().parse("name : value")
  )
}
