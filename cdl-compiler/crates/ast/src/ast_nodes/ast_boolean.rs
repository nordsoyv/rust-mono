use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AstBooleanNode {
  pub value: bool,
}
