use crate::common::{
    Parser,
    zero_or_more,
    one_or_more,
    match_literal,
    right,
    pair,
    three,
    four,
    space0,
    space1,
    quoted_string,
    identifier,
    left,
    either,
    whitespace_wrap,
    optional,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    name: String,
    rhs: String,
}

fn colon<'a>() -> impl Parser<'a, ()> {
    whitespace_wrap(match_literal(":"))
}

fn field<'a>() -> impl Parser<'a, Field> {
    three(identifier, colon(), identifier)
        .map(move |(name, _, value)| {
            Field {
                name,
                rhs: value,
            }
        })
}

#[test]
fn match_field() {
    assert_eq!(
        Ok(("",Field {
            name: "name".to_string(),
            rhs: "value".to_string(),
        })),
        field().parse("name : value")
    )
}