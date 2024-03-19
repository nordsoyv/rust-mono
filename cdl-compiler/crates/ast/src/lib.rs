mod ast;
mod ast_nodes;

use serde::Serialize;
use std::fmt::Debug;

pub use ast_nodes::AstBooleanNode;
pub use ast_nodes::AstColorNode;
pub use ast_nodes::AstEntityNode;
pub use ast_nodes::AstFormulaNode;
pub use ast_nodes::AstFunctionNode;
pub use ast_nodes::AstIdentifierNode;
pub use ast_nodes::AstNumberNode;
pub use ast_nodes::AstOperatorNode;
pub use ast_nodes::AstPropertyNode;
pub use ast_nodes::AstReferenceNode;
pub use ast_nodes::AstScriptNode;
pub use ast_nodes::AstStringNode;
pub use ast_nodes::AstTableAliasNode;
pub use ast_nodes::AstTitleNode;
pub use ast_nodes::AstVPathNode;
pub use ast_nodes::Operator;
pub use ast_nodes::QuoteKind;

pub use ast::Ast;

#[derive(Debug, Serialize, Clone)]
pub enum Node {
  Title(AstTitleNode),
  Entity(AstEntityNode),
  Property(AstPropertyNode),
  Identifier(AstIdentifierNode),
  Script(AstScriptNode),
  String(AstStringNode),
  Number(AstNumberNode),
  Boolean(AstBooleanNode),
  VPath(AstVPathNode),
  Color(AstColorNode),
  Reference(AstReferenceNode),
  Function(AstFunctionNode),
  Operator(AstOperatorNode),
  TableAlias(AstTableAliasNode),
  Formula(AstFormulaNode),
}

impl Node {
  pub fn is_reference(&self) -> bool {
    matches!(self, Node::Reference(_))
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct NodeRef(pub isize);

impl From<usize> for NodeRef {
  fn from(value: usize) -> Self {
    NodeRef(value as isize)
  }
}

impl Debug for NodeRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Serialize, Clone)]
pub struct AstNode {
  pub parent: NodeRef,
  pub node_data: Node,
  pub processed: bool,
}

impl AstNode {
  pub fn new(node: Node, parent: NodeRef) -> AstNode {
    AstNode {
      node_data: node,
      parent,
      processed: false
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
