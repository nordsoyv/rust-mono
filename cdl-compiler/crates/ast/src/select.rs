use crate::{Ast, Node, NodeRef};

pub fn select_property(ast: &Ast, name: &str) -> Vec<NodeRef> {
  let mut result = vec![];

  for (index, node) in ast.nodes.borrow().iter().enumerate() {
    let node_data = &node.node_data;
    if let Node::Property(property) = node_data {
      if name.eq(&property.name.0.to_string()) {
        result.push(index.into());
      }
    }
  }

  result
}

pub fn select_property_value(ast: &Ast, name: &str) -> Vec<NodeRef> {
  let mut result = vec![];

  for node in ast.nodes.borrow().iter() {
    let node_data = &node.node_data;
    if let Node::Property(property) = node_data {
      if name.eq(&property.name.0.to_string()) {
        result.push(*property.children.borrow().get(0).unwrap());
      }
    }
  }

  result
}
