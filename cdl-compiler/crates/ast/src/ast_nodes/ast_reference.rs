use serde::Serialize;
use std::{cell::Cell, rc::Rc};

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub struct AstReferenceNode {
  pub ident: Rc<str>,
  pub resolved_node: Cell<NodeRef>,
}

impl AstReferenceNode {
  pub fn set_reference(&self, node_ref: NodeRef){
    self.resolved_node.set(node_ref)
  }
}