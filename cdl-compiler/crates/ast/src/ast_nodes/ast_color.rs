use serde::Serialize;
use std::rc::Rc;

#[derive(Debug, Serialize, Clone)]
pub struct AstColorNode {
  pub color: Rc<str>,
}
