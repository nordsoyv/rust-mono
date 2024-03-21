use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AstNumberNode {
  pub value: f64,
}
