use serde::Serialize;
use std::rc::Rc;

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub struct AstEntityNode {
  pub children: Vec<NodeRef>,
  pub terms: Vec<Rc<str>>,
  pub label: Option<Rc<str>>,
  pub refs: Vec<Rc<str>>,
  pub ident: Option<Rc<str>>,
  pub entity_number: Option<f64>,
}
