use serde::Serialize;
use std::rc::Rc;

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub struct AstFunctionNode {
  pub name: Rc<str>,
  pub children: Vec<NodeRef>,
}
