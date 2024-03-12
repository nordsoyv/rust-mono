use serde::Serialize;
use std::rc::Rc;

#[derive(Debug, Serialize, Clone)]
pub struct AstTitleNode {
  pub title: Rc<str>,
}


