use serde_derive::{Deserialize, Serialize};

pub type EntityRef = usize;
pub type PropertyRef = usize;
pub type RhsRef = usize;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstEntity {
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
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstIdentifier {
  pub value: String,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstString {
  pub value: String,
  pub start_pos: usize,
  pub end_pos: usize,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AstProperty {
  pub name: String,
  pub rhs: RhsRef,
  pub start_pos: usize,
  pub end_pos: usize,
}
