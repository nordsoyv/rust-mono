use serde::Serialize;
use std::rc::Rc;

use crate::NodeRef;


#[derive(Debug, Serialize, Clone)]
pub struct AstPropertyNode {
  pub name: Rc<str>,
  pub child: Vec<NodeRef>,
}


