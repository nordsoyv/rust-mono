pub mod ast_boolean;
pub mod ast_color;
pub mod ast_entity;
pub mod ast_formula;
pub mod ast_function;
pub mod ast_identifier;
pub mod ast_number;
pub mod ast_operator;
pub mod ast_property;
pub mod ast_reference;
pub mod ast_script;
pub mod ast_string;
pub mod ast_table_alias;
pub mod ast_title;
pub mod ast_vpath;

use anyhow::Result;
use ast::NodeRef;

use crate::parser::Parser;

pub trait Parsable {
  fn can_parse(parser: &Parser) -> bool;
  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef>;
}
