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
use serde::Serialize;

use crate::{parser::Parser, types::NodeRef, Node};

pub use ast_color::AstColorNode;
pub use ast_entity::AstEntityNode;
pub use ast_formula::AstFormulaNode;
pub use ast_identifier::AstIdentifierNode;
pub use ast_number::AstNumberNode;
pub use ast_operator::AstOperatorNode;
pub use ast_property::AstPropertyNode;
pub use ast_reference::AstReferenceNode;
pub use ast_script::AstScriptNode;
pub use ast_string::AstStringNode;
pub use ast_table_alias::AstTableAliasNode;
pub use ast_title::AstTitleNode;
pub use ast_vpath::AstVPathNode;

pub trait Parsable {
  fn can_parse(parser: &Parser) -> bool;
  fn parse(parser: &mut Parser, parent: NodeRef) -> Result<NodeRef>;
}

#[derive(Debug, Serialize,Clone)]
pub struct AstNode {
  pub parent: NodeRef,
  pub node_data: Node,
}

impl AstNode {
  pub fn new(node: Node, parent: NodeRef) -> AstNode {
    AstNode {
      node_data: node,
      parent,
    }
  }

  pub fn add_child_to_node(&mut self, child: NodeRef) {
    let node_data = &mut self.node_data;
    match node_data {
      Node::Entity(ref mut ent) => ent.children.push(child),
      Node::Script(ref mut script) => script.children.push(child),
      Node::Property(ref mut prop) => prop.child.push(child),
      Node::Function(ref mut func) => func.children.push(child),
      Node::Operator(ref mut op) => op.right = child,
      _ => panic!("Unknown type to set as parent {:?}", node_data),
    };
  }
}
