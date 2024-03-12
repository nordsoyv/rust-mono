use serde::Serialize;
use std::rc::Rc;

#[derive(Debug, Serialize, Clone)]
pub struct AstVPathNode {
  pub table: Option<Rc<str>>,
  pub variable: Option<Rc<str>>,
  pub function: Option<Rc<str>>,
  pub is_hierarchy: bool,
}


