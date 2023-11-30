use std::fmt::Debug;

#[derive(Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeRef(pub isize);

impl From<usize> for NodeRef {
  fn from(value: usize) -> Self {
    NodeRef(value as isize)
  }
}

impl Debug for NodeRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
