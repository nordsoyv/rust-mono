use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct NodeRef(pub usize);

impl From<usize> for NodeRef{
     fn from(value: usize) -> Self {
        NodeRef(value)
    }
}

impl Debug for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}