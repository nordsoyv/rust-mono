pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

pub trait Parser<'a, Output> {
  fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
  fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput>
  where
    Self: Sized + 'a,
    Output: 'a,
    NewOutput: 'a,
    F: Fn(Output) -> NewOutput + 'a,
  {
    BoxedParser::new(map(self, map_fn))
  }

  fn pred<F>(self, pred_fn: F) -> BoxedParser<'a, Output>
  where
    Self: Sized + 'a,
    Output: 'a,
    F: Fn(&Output) -> bool + 'a,
  {
    BoxedParser::new(pred(self, pred_fn))
  }

  fn and_then<F, NextParser, NewOutput>(
    self,
    f: F,
  ) -> BoxedParser<'a, NewOutput>
  where
    Self: Sized + 'a,
    Output: 'a,
    NewOutput: 'a,
    NextParser: Parser<'a, NewOutput> + 'a,
    F: Fn(Output) -> NextParser + 'a,
  {
    BoxedParser::new(and_then(self, f))
  }
}

impl<'a, F, Output> Parser<'a, Output> for F
where
  F: Fn(&'a str) -> ParseResult<Output>,
{
  fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
    self(input)
  }
}

pub struct BoxedParser<'a, Output> {
  parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
  fn new<P>(parser: P) -> Self
  where
    P: Parser<'a, Output> + 'a,
  {
    BoxedParser {
      parser: Box::new(parser),
    }
  }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
  fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
    self.parser.parse(input)
  }
}

pub fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
  move |input: &'a str| match input.get(0..expected.len()) {
    Some(next) if next == expected => Ok((&input[expected.len()..], ())),
    _ => Err(input),
  }
}

pub fn identifier(input: &str) -> ParseResult<String> {
  let mut matched = String::new();
  let mut chars = input.chars();

  match chars.next() {
    Some(next) if next.is_alphabetic() => matched.push(next),
    _ => return Err(input),
  }

  while let Some(next) = chars.next() {
    if next.is_alphanumeric() || next == '-' {
      matched.push(next);
    } else {
      break;
    }
  }

  let next_index = matched.len();
  Ok((&input[next_index..], matched))
}

pub fn pair<'a, P1, P2, R1, R2>(
  parser1: P1,
  parser2: P2,
) -> impl Parser<'a, (R1, R2)>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
{
  move |input| {
    parser1.parse(input).and_then(|(next_input, result1)| {
      parser2
        .parse(next_input)
        .map(|(last_input, result2)| (last_input, (result1, result2)))
    })
  }
}

pub fn three<'a, P1, P2, P3, R1, R2, R3>(
  parser1: P1,
  parser2: P2,
  parser3: P3,
) -> impl Parser<'a, (R1, R2, R3)>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
  P3: Parser<'a, R3>,
{
  move |input| {
    parser1.parse(input).and_then(|(second_input, result1)| {
      parser2
        .parse(second_input)
        .and_then(|(third_input, result2)| {
          parser3.parse(third_input).map(|(last_input, result3)| {
            (last_input, (result1, result2, result3))
          })
        })
    })
  }
}

pub fn four<'a, P1, P2, P3, P4, R1, R2, R3, R4>(
  parser1: P1,
  parser2: P2,
  parser3: P3,
  parser4: P4,
) -> impl Parser<'a, (R1, R2, R3, R4)>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
  P3: Parser<'a, R3>,
  P4: Parser<'a, R4>,
{
  move |input| {
    parser1.parse(input).and_then(|(second_input, result1)| {
      parser2
        .parse(second_input)
        .and_then(|(third_input, result2)| {
          parser3
            .parse(third_input)
            .and_then(|(fourth_input, result3)| {
              parser4.parse(fourth_input).map(|(last_input, result4)| {
                (last_input, (result1, result2, result3, result4))
              })
            })
        })
    })
  }
}

pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
  P: Parser<'a, A>,
  F: Fn(A) -> B,
{
  move |input| {
    parser
      .parse(input)
      .map(|(next_input, result)| (next_input, map_fn(result)))
  }
}

pub fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
{
  map(pair(parser1, parser2), |(left, _right)| left)
}

pub fn right<'a, P1, P2, R1, R2>(
  parser1: P1,
  parser2: P2,
) -> impl Parser<'a, R2>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
{
  map(pair(parser1, parser2), |(_left, right)| right)
}

pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
  P: Parser<'a, A>,
{
  move |mut input| {
    let mut result = Vec::new();

    if let Ok((next_input, first_item)) = parser.parse(input) {
      input = next_input;
      result.push(first_item);
    } else {
      return Err(input);
    }

    while let Ok((next_input, next_item)) = parser.parse(input) {
      input = next_input;
      result.push(next_item);
    }

    Ok((input, result))
  }
}

pub fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
  P: Parser<'a, A>,
{
  move |mut input| {
    let mut result = Vec::new();

    while let Ok((next_input, next_item)) = parser.parse(input) {
      input = next_input;
      result.push(next_item);
    }

    Ok((input, result))
  }
}

pub fn any_char(input: &str) -> ParseResult<char> {
  match input.chars().next() {
    Some(next) => Ok((&input[next.len_utf8()..], next)),
    _ => Err(input),
  }
}

pub fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
  P: Parser<'a, A>,
  F: Fn(&A) -> bool,
{
  move |input| {
    if let Ok((next_input, value)) = parser.parse(input) {
      if predicate(&value) {
        return Ok((next_input, value));
      }
    }
    Err(input)
  }
}

pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
  any_char.pred(|c| c.is_whitespace())
}

pub fn space1<'a>() -> impl Parser<'a, Vec<char>> {
  one_or_more(whitespace_char())
}

pub fn space0<'a>() -> impl Parser<'a, Vec<char>> {
  zero_or_more(whitespace_char())
}

pub fn quoted_string<'a>() -> impl Parser<'a, String> {
  right(
    match_literal("\""),
    left(
      zero_or_more(any_char.pred(|c| *c != '"')),
      match_literal("\""),
    ),
  )
  .map(|chars| chars.into_iter().collect())
}

pub fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
  P1: Parser<'a, A>,
  P2: Parser<'a, A>,
{
  move |input| match parser1.parse(input) {
    ok @ Ok(_) => ok,
    Err(_) => parser2.parse(input),
  }
}

pub fn one_of<'a, P1, R1>(parsers: Vec<P1>) -> impl Parser<'a, R1>
where
  P1: Parser<'a, R1>,
{
  move |input| {
    for parser in &parsers {
      match parser.parse(input) {
        ok @ Ok(_) => return ok,
        Err(_) => continue,
      }
    }
    return Err(input);
  }
}

pub fn optional<'a, P1, R1>(parser: P1, default: R1) -> impl Parser<'a, R1>
where
  P1: Parser<'a, R1>,
  R1: Clone,
{
  move |input| match parser.parse(input) {
    ok @ Ok(_) => ok,
    Err(_) => Ok((input, default.clone())),
  }
}

pub fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
  P: Parser<'a, A>,
  NextP: Parser<'a, B>,
  F: Fn(A) -> NextP,
{
  move |input| match parser.parse(input) {
    Ok((next_input, result)) => f(result).parse(next_input),
    Err(err) => Err(err),
  }
}

pub fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
  P: Parser<'a, A>,
{
  right(space0(), left(parser, space0()))
}

#[test]
fn literal_parser() {
  let parse_joe = match_literal("Hello Joe!");
  assert_eq!(Ok(("", ())), parse_joe.parse("Hello Joe!"));
  assert_eq!(
    Ok((" Hello Robert!", ())),
    parse_joe.parse("Hello Joe! Hello Robert!")
  );
  assert_eq!(Err("Hello Mike!"), parse_joe.parse("Hello Mike!"));
}

#[test]
fn identifier_parser() {
  assert_eq!(
    Ok(("", "i-am-an-identifier".to_string())),
    identifier("i-am-an-identifier")
  );
  assert_eq!(
    Ok((" entirely an identifier", "not".to_string())),
    identifier("not entirely an identifier")
  );
  assert_eq!(
    Err("!not at all an identifier"),
    identifier("!not at all an identifier")
  );
}

#[test]
fn pair_combinator() {
  let tag_opener = pair(match_literal("<"), identifier);
  assert_eq!(
    Ok(("/>", ((), "my-first-element".to_string()))),
    tag_opener.parse("<my-first-element/>")
  );
  assert_eq!(Err("oops"), tag_opener.parse("oops"));
  assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
}

#[test]
fn three_combinator() {
  let tag_opener = three(match_literal("<"), identifier, match_literal(">"));
  assert_eq!(
    Ok(("", ((), "my-first-element".to_string(), ()))),
    tag_opener.parse("<my-first-element>")
  );
  assert_eq!(Err("oops"), tag_opener.parse("oops"));
  assert_eq!(Err(""), tag_opener.parse("<oops"));
}

#[test]
fn four_combinator() {
  let tag_opener = four(
    match_literal("<"),
    identifier,
    match_literal("|"),
    match_literal(">"),
  );
  assert_eq!(
    Ok(("", ((), "my-first-element".to_string(), (), ()))),
    tag_opener.parse("<my-first-element|>")
  );
  assert_eq!(Err("oops"), tag_opener.parse("oops"));
  assert_eq!(Err(""), tag_opener.parse("<oops"));
}

#[test]
fn right_combinator() {
  let tag_opener = right(match_literal("<"), identifier);
  assert_eq!(
    Ok(("/>", "my-first-element".to_string())),
    tag_opener.parse("<my-first-element/>")
  );
  assert_eq!(Err("oops"), tag_opener.parse("oops"));
  assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
}

#[test]
fn one_or_more_combinator() {
  let parser = one_or_more(match_literal("ha"));
  assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
  assert_eq!(Err("ahah"), parser.parse("ahah"));
  assert_eq!(Err(""), parser.parse(""));
}

#[test]
fn zero_or_more_combinator() {
  let parser = zero_or_more(match_literal("ha"));
  assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
  assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
  assert_eq!(Ok(("", vec![])), parser.parse(""));
}

#[test]
fn predicate_combinator() {
  let parser = pred(any_char, |c| *c == 'o');
  assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
  assert_eq!(Err("lol"), parser.parse("lol"));
}

#[test]
fn quoted_string_parser() {
  assert_eq!(
    Ok(("", "Hello Joe!".to_string())),
    quoted_string().parse("\"Hello Joe!\"")
  );
}

#[test]
fn optional_parser() {
  let optional_parser = optional(identifier, "default".to_string());

  assert_eq!(
    Ok(("", "ident".to_string())),
    optional_parser.parse("ident")
  );
  assert_eq!(
    Ok(("1234", "default".to_string())),
    optional_parser.parse("1234")
  );
}

#[test]
fn one_of_parser() {
  let one_of_parser = one_of(vec![
    match_literal("a"),
    match_literal("b"),
    match_literal("c"),
    match_literal("d"),
  ]);
  assert_eq!(Ok(("", ())), one_of_parser.parse("a"));
  assert_eq!(Ok(("", ())), one_of_parser.parse("b"));
  assert_eq!(Ok(("", ())), one_of_parser.parse("c"));
  assert_eq!(Ok(("", ())), one_of_parser.parse("d"));
  assert_eq!(Err("e"), one_of_parser.parse("e"));
}
