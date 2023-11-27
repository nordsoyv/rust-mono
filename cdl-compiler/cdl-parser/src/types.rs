#[derive(Debug, Clone, Copy)]
pub struct NodeRef(pub usize);

impl From<usize> for NodeRef{
     fn from(value: usize) -> Self {
        NodeRef(value)
    }
}
