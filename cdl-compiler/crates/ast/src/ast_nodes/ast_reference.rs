use serde::Serialize;
use std::rc::Rc;

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub struct AstReferenceNode {
  pub ident: Rc<str>,
  pub resolved_node: NodeRef,
}
