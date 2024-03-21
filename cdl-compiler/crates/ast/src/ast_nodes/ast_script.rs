use serde::Serialize;

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub struct AstScriptNode {
  pub children: Vec<NodeRef>,
}
