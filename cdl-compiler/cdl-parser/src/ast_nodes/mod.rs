pub mod ast_entity;
pub mod ast_title;
pub mod ast_property;
pub mod ast_identifier;

use anyhow::Result;

use crate::{parser::Parser, types::NodeRef};

pub trait Parsable {
  fn can_parse(parser: &Parser) -> bool;
  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef>;
}
