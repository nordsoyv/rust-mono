pub mod ast_entity;
pub mod ast_identifier;
pub mod ast_number;
pub mod ast_property;
pub mod ast_script;
pub mod ast_string;
pub mod ast_title;
pub mod ast_vpath;
pub mod ast_color;
pub mod ast_reference;
pub mod ast_function;

use anyhow::Result;

use crate::{parser::Parser, types::NodeRef};

pub use ast_color::AstColorNode;
pub use ast_entity::AstEntityNode;
pub use ast_identifier::AstIdentifierNode;
pub use ast_number::AstNumberNode;
pub use ast_property::AstPropertyNode;
pub use ast_script::AstScriptNode;
pub use ast_string::AstStringNode;
pub use ast_title::AstTitleNode;
pub use ast_vpath::AstVPathNode;
pub use ast_reference::AstReferenceNode;

pub trait Parsable {
  fn can_parse(parser: &Parser) -> bool;
  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef>;
}

