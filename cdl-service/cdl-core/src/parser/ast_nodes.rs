use serde_derive::{Deserialize, Serialize};

pub type NodeRef = usize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstEntity {
  pub parent : NodeRef,
  pub terms: Vec<String>,
  pub refs: Vec<String>,
  pub entity_id: f64,
  pub identifier: String,
  pub label : String,
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
  Equal,
  And,
  Or,
  LessThan,LessThanOrEqual,
  MoreThan,MoreThanOrEqual,
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
pub struct AstReference {
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
  pub args: Option<NodeRef>,
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstVPath{
  pub parent : NodeRef,
  pub source : String,
  pub question: String,
  pub start_pos: usize,
  pub end_pos: usize,
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstTitle{
  pub parent : NodeRef,
  pub title : String,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstColor{
  pub parent : NodeRef,
  pub value : String,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstTableDecl{
  pub parent : NodeRef,
  pub name : String,
  pub path : String,
  pub start_pos: usize,
  pub end_pos: usize,
}



