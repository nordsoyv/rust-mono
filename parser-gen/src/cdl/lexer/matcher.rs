pub type ParseResult<'a> = Result<usize, &'a str>;

pub trait Matcher {
  fn check(&self, input: &str) -> ParseResult;
}
