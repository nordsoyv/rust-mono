pub type ParseResult<'a> = Result<(usize, Option<String>), &'a str>;

pub trait Matcher {
  fn check(&self, input: &str) -> ParseResult;
}
