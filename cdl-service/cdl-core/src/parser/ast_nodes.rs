use serde_derive::{Deserialize, Serialize};

pub type NodeRef = usize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstEntity {
  pub parent : NodeRef,
  pub terms: Vec<String>,
  pub refs: Vec<String>,
  pub entity_id: String,
  pub children: Vec<NodeRef>,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstIdentifier {
  pub parent : NodeRef,
  pub value: String,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum Operator {
  Plus,
  Minus,
  Mul,
  Del,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstOperator {
  pub parent : NodeRef,
  pub op: Operator,
  pub left: NodeRef,
  pub right: NodeRef,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstString {
  pub parent : NodeRef,
  pub value: String,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstNumber {
  pub parent : NodeRef,
  pub value: f64,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstUnaryOp {
  pub parent : NodeRef,
  pub op : Operator,
  pub right: NodeRef,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstProperty {
  pub parent : NodeRef,
  pub name: String,
  pub rhs: NodeRef,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstFunctionCall{
  pub parent : NodeRef,
  pub name: String,
  pub args: NodeRef,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstList{
  pub parent : NodeRef,
  pub items: Vec<NodeRef>,
  pub start_pos: usize,
  pub end_pos: usize,
}



