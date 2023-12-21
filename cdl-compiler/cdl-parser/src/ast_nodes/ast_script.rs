use std::ops::Range;

use crate::types::NodeRef;

#[derive(Debug)]
pub struct AstScriptNode {
  pub children: Vec<NodeRef>,
  pub location: Range<usize>
}
