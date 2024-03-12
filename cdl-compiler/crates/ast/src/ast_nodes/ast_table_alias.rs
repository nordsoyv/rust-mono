use serde::Serialize;
use std::rc::Rc;

#[derive(Debug, Serialize, Clone)]
pub struct AstTableAliasNode {
  pub table: Rc<str>,
  pub alias: Rc<str>,
}


