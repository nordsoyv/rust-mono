use serde_derive::{Deserialize, Serialize};

pub type EntityRef = usize;
pub type PropertyRef = usize;
pub type RhsRef = usize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstEntity {
  pub parent : Parent,
  pub terms: Vec<String>,
  pub refs: Vec<String>,
  pub entity_id: String,
  pub child_entities: Vec<EntityRef>,
  pub properties: Vec<PropertyRef>,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Rhs {
  Identifier(AstIdentifier),
  String(AstString),
  Operator(AstOperator),
  Number(AstNumber),
  UnaryOp(AstUnaryOp),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Parent {
  None,
  Entity(EntityRef),
  Property(PropertyRef),
  Rhs(RhsRef)
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstIdentifier {
  pub parent : Parent,
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
  pub parent : Parent,
  pub op: Operator,
  pub left: RhsRef,
  pub right: RhsRef,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstString {
  pub parent : Parent,
  pub value: String,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstNumber {
  pub parent : Parent,
  pub value: f64,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstUnaryOp {
  pub parent : Parent,
  pub op : Operator,
  pub right: RhsRef,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstProperty {
  pub parent : Parent,
  pub name: String,
  pub rhs: RhsRef,
  pub start_pos: usize,
  pub end_pos: usize,
}
