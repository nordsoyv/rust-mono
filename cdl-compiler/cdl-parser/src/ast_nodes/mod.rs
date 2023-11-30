pub mod ast_entity;
pub mod ast_title;

use anyhow::Result;

use crate::{parser::Parser, types::NodeRef};

pub trait Parsable {
  fn can_parse(parser: &Parser) -> bool;
  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef>;
}
